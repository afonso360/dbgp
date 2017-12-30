extern crate dbgp;
extern crate serde_xml_rs;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;

fn handle_client(mut stream: TcpStream) {
    //let session = Session::new(stream, SessionType::Server);

    // read 20 bytes at a time from stream echoing back to stream
    use dbgp::packets::{Init, Packet};
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { // connection was closed
                    break;
                }

                let message = String::from_utf8_lossy(&read[..]).into_owned();
                println!("message: {}\n", message);



                let packet: Packet<Init> = Packet::deserialize(BufReader::new(&read[..])).unwrap();
                println!("parsed: {:?}\n", packet);

                stream.write("feature_get -i 0 -n supports_async\0".as_bytes()).unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    println!("New connection");
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}
