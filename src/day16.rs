#![allow(unused)]
use std::str::FromStr;

use itertools::Itertools;

fn to_bits(n: u32) -> Vec<u8> {
    let mut bits = Vec::new();
    let mut n = n;
    while n > 0 {
        bits.push((n % 2) as u8);
        n /= 2;
    }
    bits
}

fn from_bits(bits: &[u8]) -> u32 {
    let mut n: u32 = 0;
    for &bit in bits {
        n = n * 2 + bit as u32;
    }
    n
}

fn hex_to_bin(s: &str) -> Result<Vec<u8>, ()> {
    let mut bits = Vec::with_capacity(4 * s.len());
    for c in s.chars() {
        let block: [u8; 4] = match c {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => return Err(()),
        };
        bits.extend_from_slice(&block);
    }
    Ok(bits)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LengthTypeId {
    TotalLength,
    CountSubpackets,
}

#[derive(Debug, Clone, PartialEq)]
enum PacketType {
    Literal(usize),
    Operator((LengthTypeId, Box<Packet>)),
}

impl PacketType {
    fn literal_from_bits(bits: &[u8]) -> Result<PacketType, ()> {
        let mut n = 0;
        for mut group in &bits.iter().chunks(5) {
            let last_group = match group.next() {
                Some(0) => true,
                Some(1) => false,
                _ => return Err(()),
            };
            for &bit in group {
                n = n * 2 + bit as usize;
            }
            if last_group {
                return Ok(PacketType::Literal(n));
            }
        }
        Err(())
    }

    fn operator_from_bits(bits: &[u8]) -> Result<PacketType, ()> {
        unimplemented!()
    }

    fn from_bits(bits: &[u8]) -> Result<PacketType, ()> {
        match from_bits(&bits[0..3]) {
            4 => PacketType::literal_from_bits(&bits[3..]),
            _ => PacketType::operator_from_bits(&bits[3..]),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

impl FromStr for Packet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = hex_to_bin(s)?;
        let version = from_bits(&bits[0..3]) as u8;
        let packet_type = PacketType::from_bits(&bits[3..])?;
        Ok(Packet {
            version,
            packet_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bin() {
        let bits = hex_to_bin("D2FE28").unwrap();
        assert_eq!(
            bits,
            vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
    }

    #[test]
    fn test_literal_packet_from_str() {
        // 110100101111111000101000
        let packet = Packet::from_str("D2FE28").unwrap();
        assert_eq!(packet.version, 6);
        assert_eq!(packet.packet_type, PacketType::Literal(2021));
    }
}
