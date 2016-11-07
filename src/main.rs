#![feature(integer_atomics)]
extern crate regex;
extern crate hyper;
extern crate crossbeam;
use std::sync::mpsc;

mod becommand;
mod bepackets;
mod packet;
mod rcon_error;

mod rcon;
use rcon::RConClient;

fn main() {
    let client = RConClient::new(2312);
    let (tx, rx) = mpsc::channel();

    crossbeam::scope(|scope| {
        scope.spawn(move || { client.start("bla", tx).unwrap(); });
        scope.spawn(move || {
            loop {
                match rx.recv().unwrap() {
                    _ => println!("PACKET RECEIVED")
                };
            }
        })
    });
}