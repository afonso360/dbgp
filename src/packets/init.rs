use url_serde;
use url::Url;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Init {
    pub appid: String,
    pub idekey: String,
    pub session: String,
    pub thread: String,
    pub parent: String,
    pub language: String,
    pub protocol_version: String,

    #[serde(with = "url_serde")]
    pub fileuri: Url,
}

#[cfg(test)]
mod tests {
    use serde_xml_rs::deserialize;
    use super::Init;
    use url::Url;

    #[test]
    fn deserialize_init_packet_simple() {
            let s = r##"
            <?xml version="1.0" encoding="UTF-8"?>
            <init appid="APPID"
                  idekey="IDE_KEY"
                  session="DBGP_COOKIE"
                  thread="THREAD_ID"
                  parent="PARENT_APPID"
                  language="LANGUAGE_NAME"
                  protocol_version="1.0"
                  fileuri="file://path/to/file" />
            "##;

            let res = Init {
                appid: String::from("APPID"),
                idekey: String::from("IDE_KEY"),
                session: String::from("DBGP_COOKIE"),
                thread: String::from("THREAD_ID"),
                parent: String::from("PARENT_APPID"),
                language: String::from("LANGUAGE_NAME"),
                protocol_version: String::from("1.0"),
                fileuri: Url::parse("file://path/to/file").unwrap(),
            };

            let init: Init = deserialize(s.as_bytes()).unwrap();
            assert_eq!(init, res);
    }

    // TODO: Parse with child elements

}
