use std::thread;
use std::net;

extern crate regex;
//extern crate hyper;
use regex::Regex;

use std::sync::Arc;
use std::sync::Mutex;

mod commands;
use commands::parse_packet;
use commands::{RconRequest, RemotePacket};

mod packet;
use packet::construct;

mod rcon;
use rcon::RConClient;

mod whitelist;
use whitelist::is_listed;

fn main() {
    let mut client = RConClient::new(2312);
    client.connect();
    println!("sent {}", client.socket.send(&construct(RconRequest::Login, Vec::from("bla"))).unwrap());

    let client = Arc::new(client);

    let re_guid = Regex::new("Player #([0-9]+) Ron Johnson - GUID: ([0-9a-f]+)").unwrap();

    {
        let mut seq = 0;
        let client = client.clone();
        loop {
            let mut recv: Vec<u8> = vec![];
            // This should be optimized somehow? feels way too hacky
            recv.resize(512, 0x0);
            let mut c = client.socket.recv_from(&mut recv).unwrap();
            recv.resize(c.0, 0x0);

            let mut rp = parse_packet(recv);
            client.send_ack(&rp);

            match rp {
                RemotePacket::Login(success) => {
                    if !success {
                        panic!("could not log in");
                    }

                    println!("logged in.");
                    {
                        let client = client.clone();
                        thread::spawn(move || {
                            loop {
                                println!("bytes sent in keep-alive: {}", client.socket.send(&construct(RconRequest::Command, vec![0x00])).unwrap());
                                thread::sleep(std::time::Duration::from_secs(20));
                            }
                        });
                    }
                },
                RemotePacket::Command(ref seq, ref msg) => {
                    println!("cmd ack received for {}, message {}", seq, msg);
                }
                RemotePacket::Log(_, ref message) => {
                    println!("LOG: {}", message);
                    for cap in re_guid.captures_iter(&message) {
                        let guid = cap.at(2).unwrap();

                        if !is_listed(&guid) {
                            let mut kick = format!("kick {} not whitelisted", cap.at(1).unwrap()).into_bytes();
                            kick.insert(0, seq);
                            seq += 1;
                            println!("{}", client.socket.send(&construct(RconRequest::Command, kick)).unwrap());
                        }
                    }
                }
                _ => ()
            };
        }
    }
}