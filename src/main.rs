#![feature(integer_atomics)]
extern crate regex;
extern crate hyper;
extern crate crossbeam;
use std::sync::mpsc;
use std::sync::Arc;

mod becommand;
use becommand::BECommand;
mod bepackets;
use bepackets::RemotePacket;
mod packet;
mod rcon_error;

mod rcon;
use rcon::RConClient;

fn main() {
    let client = Arc::new(RConClient::new(2312));
    let (tx, rx) = mpsc::channel();

    crossbeam::scope(|scope| {
        {
            let client = client.clone();
            scope.spawn(move || { client.start("bla", tx).unwrap(); });
        }
        scope.spawn(move || {
            loop {
                match rx.recv().unwrap() {
                    RemotePacket::Login(success) => {
                        if success {
                            println!("successfully logged in.");
                            client.send(BECommand::Players).unwrap();
                        }
                    },
                    RemotePacket::Command(ref seq, ref data) => println!("received command response (seq# {}): {}", seq, data),
                    RemotePacket::Log(_, ref data) => println!("[LOG] {}", data),
                    _ => println!("PACKET RECEIVED")
                };
            }
        })
    });
}