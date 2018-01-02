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
    let mut command_state = 0;
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }

                let message = String::from_utf8_lossy(&read[..]).into_owned();
                println!("message: {}\n", message);



                let packet_reader = BufReader::new(&read[..]);
                let packet: AllPackets = Packet::deserialize(packet_reader).unwrap();
                println!("parsed: {:?}\n", packet);

                match command_state {
                    0 => {
                        let cmd = (dbgp::commands::Break {}).serialize(transaction_id);
                        stream.write(cmd.as_bytes()).unwrap();
                    }
                    1 => {
                        let cmd = (dbgp::commands::Run {}).serialize(transaction_id);
                        stream.write(cmd.as_bytes()).unwrap();
                    }
                    _ => {
                        let cmd = (dbgp::commands::Status {}).serialize(transaction_id);
                        stream.write(cmd.as_bytes()).unwrap();
                    }
                }

                transaction_id += 1;
                command_state += 1;

                thread::sleep_ms(200);

                let cmd = (dbgp::commands::Run {}).serialize(transaction_id);
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
