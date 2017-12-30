use std::io;
use serde::{Serialize, Deserialize};
use std::str::{self, FromStr};
use super::Result;
use serde_xml_rs;

#[derive(Debug, Clone, PartialEq)]
pub struct Packet<I> {
    /// Do *not* rely on this, it could be anything!
    /// but we should't read more than this amount of bytes
    pub data_length: u64,
    pub inner: I,
}

impl<'de, I: Deserialize<'de>> Packet<I> {
    pub fn deserialize<R: io::BufRead>(mut input: R) -> Result<Self> {
        // Parse packet length
        let mut data_length_buf = Vec::new();
        input.read_until(b'\0', &mut data_length_buf)?;
        data_length_buf.pop();

        let data_length_str = str::from_utf8(&data_length_buf)?;
        let data_length = u64::from_str(data_length_str)?;

        let mut inner_buf = Vec::new();
        input.read_until(b'\0', &mut inner_buf)?;
        inner_buf.pop();

        let inner: I = serde_xml_rs::deserialize(&inner_buf[..])?;

        Ok(Packet {
            data_length,
            inner,
        })
    }
}

impl<I: Serialize> Packet<I> {
    // TODO: The serializer wrongly encodes attributes
    // SEE: https://github.com/RReverser/serde-xml-rs/issues/49
    pub fn serialize(&self) -> Result<String> {
        let xml_prefix = r#"<?xml version="1.0" encoding="UTF-8" ?>"#;

        let mut buffer = Vec::new();
        serde_xml_rs::serialize(&self.inner, &mut buffer)?;
        let inner = String::from_utf8(buffer)?;

        Ok(format!("{}\0{}{}\0", inner.len() + xml_prefix.len(), xml_prefix, inner))
    }
}


#[cfg(test)]
mod tests {
    use super::Packet;
    use std::io::{BufReader, Read};


    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    struct Item {
        pub a: String,
    }

    #[test]
    fn deserialize_init_packet_simple() {
        let xml = r##"<?xml version="1.0" encoding="UTF-8" ?><Item a="s"/>"##;
        let source_string = format!("{}\0{}\0", xml.len(), xml);
        let source = source_string.as_bytes();
        let inner: Item = Item{
            a: "s".to_owned(),
        };

        let p: Packet<Item> = Packet::deserialize(BufReader::new(source)).unwrap();

        assert_eq!(Packet{
            data_length: xml.len() as u64,
            inner,
        }, p);
    }


    #[test]
    #[ignore]
    fn serialize_init_packet_simple() {
        let xml = r##"<?xml version="1.0" encoding="UTF-8" ?><Item a="s"/>"##;
        let target = format!("{}\0{}\0", xml.len(), xml);

        let packet: Packet<Item> = Packet{
            data_length: xml.len() as u64,
            inner: Item{
                a: "s".to_owned(),
            },
        };

        let p: String = packet.serialize().unwrap();
        assert_eq!(p, target);
    }
}

