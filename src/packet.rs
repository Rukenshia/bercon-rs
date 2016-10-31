extern crate crc;
use self::crc::crc32;
use self::crc::Hasher32;
use super::commands::RconRequest;
use std::mem;

fn calc_crc(payload: &Vec<u8>) -> [u8; 4] {
    unsafe { mem::transmute(crc32::checksum_ieee(payload.as_slice())) }
}

fn create_header() -> Vec<u8> {
    vec![0x42, 0x45]
}

pub fn construct<'a>(command: RconRequest, payload: Vec<u8>) -> Vec<u8> {
    let mut v = create_header();
    let mut pbv: Vec<u8> = vec![0xFF];
    pbv.push(command as u8);
    pbv.append(&mut Vec::from(payload));

    let mut crc: [u8; 4] = calc_crc(&pbv);
    v.append(unsafe { &mut Vec::from_raw_parts(&mut crc[0], 4, 4) });

    v.append(&mut pbv);
    v
}