error_chain! {
    foreign_links {
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
        Utf8(::std::str::Utf8Error);
        StringUtf8(::std::string::FromUtf8Error);
        Xml(::serde_xml_rs::Error);
    }

}

pub mod init;
pub mod packet;

pub use self::init::Init;
pub use self::packet::Packet;
