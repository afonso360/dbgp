extern crate dbgp;
extern crate serde_xml_rs;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, BufReader};

fn handle_client(mut stream: TcpStream) {
    //let session = Session::new(stream, SessionType::Server);

    // read 20 bytes at a time from stream echoing back to stream
    use dbgp::packets::{Packet, AllPackets};
    use dbgp::commands::Command;
    let mut transaction_id = 0;
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { // connection was closed
                    break;
                }

                let message = String::from_utf8_lossy(&read[..]).into_owned();
                println!("message: {}\n", message);



                let packet: AllPackets = Packet::deserialize(BufReader::new(&read[..])).unwrap();
                println!("parsed: {:?}\n", packet);

                //let cmd = (dbgp::commands::Status{}).serialize(transaction_id);
                let cmd = (dbgp::commands::FeatureSet{
                    name: String::from("max_depth"),
                    value: String::from("20"),
                }).serialize(transaction_id);
                stream.write(cmd.as_bytes()).unwrap();
                transaction_id += 1;
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
