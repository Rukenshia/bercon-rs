use std::net;
use super::commands::{RconRequest, RemotePacket};
use super::packet::construct;
use std::io;

pub struct RConClient {
    pub socket: net::UdpSocket,
    port: u16,
    seq: u8,
}

impl RConClient {
    pub fn new(port: u16) -> Self {
        let ip = net::Ipv4Addr::new(127, 0, 0, 1);
        let this_thing = net::SocketAddrV4::new(ip, 23308);


        let socket = net::UdpSocket::bind(this_thing).unwrap();

        RConClient {
            port: port,
            socket: socket,
            seq: 0,
        }
    }

    pub fn connect(&self) {
        let ip = net::Ipv4Addr::new(127, 0, 0, 1);
        let be_server = net::SocketAddrV4::new(ip, self.port);

        self.socket.connect(net::SocketAddr::V4(be_server)).unwrap();
    }

    pub fn send_ack(&self, rp: &RemotePacket) -> bool {
        match rp {
            &RemotePacket::Log(seq, _) => self.socket.send(&construct(RconRequest::Log, vec![seq])).is_ok(),
            _ => true,
        }
    }
}