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

#[derive(Debug, Clone, PartialEq)]
struct LiteralPacket {
    data: usize,
    length: usize,
}

impl LiteralPacket {
    fn new(data: usize, length: usize) -> LiteralPacket {
        LiteralPacket { data, length }
    }

    fn from_bits(bits: &[u8]) -> Result<LiteralPacket, ()> {
        let mut n: usize = 0;
        let chunk_size = 5;
        let mut group_count = 0;
        for mut group in &bits.iter().chunks(5) {
            let last_group = match group.next() {
                Some(0) => true,
                Some(1) => false,
                _ => return Err(()),
            };
            for &bit in group {
                n = n * 2 + bit as usize;
            }
            group_count += 1;
            if last_group {
                return Ok(LiteralPacket::new(n, group_count * chunk_size));
            }
        }
        Err(())
    }

    fn len(&self) -> usize {
        self.length
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LengthTypeId {
    TotalLength,
    CountSubpackets,
}

#[derive(Debug, Clone, PartialEq)]
enum PacketType {
    Literal(LiteralPacket),
    Operator((LengthTypeId, Vec<Packet>)),
}

impl PacketType {
    fn literal_from_bits(bits: &[u8]) -> Result<PacketType, ()> {
        Ok(PacketType::Literal(LiteralPacket::from_bits(bits)?))
    }

    fn operator_from_bits(bits: &[u8]) -> Result<PacketType, ()> {
        match bits[0] {
            0 => Self::operator_total_length_from_bits(&bits[1..]),
            1 => unimplemented!(),
            _ => return Err(()),
        }
    }

    fn operator_total_length_from_bits(bits: &[u8]) -> Result<PacketType, ()> {
        let length_type_id = LengthTypeId::TotalLength;
        let total_lenght = from_bits(&bits[0..15]) as usize;
        let mut data = &bits[15..total_lenght + 15];
        let mut packets = Vec::new();
        while data.len() > 0 {
            let packet = Packet::from_bits(&data)?;
            data = &data[packet.len()..];
            packets.push(packet);
        }
        Ok(PacketType::Operator((length_type_id, packets)))
    }

    fn from_bits(bits: &[u8]) -> Result<PacketType, ()> {
        match from_bits(&bits[0..3]) {
            4 => Self::literal_from_bits(&bits[3..]),
            _ => Self::operator_from_bits(&bits[3..]),
        }
    }

    fn len(&self) -> usize {
        match self {
            PacketType::Literal(l) => l.len() + 3,
            PacketType::Operator((_, packets)) => {
                packets.iter().map(|p| p.len()).sum::<usize>() + 3
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

impl Packet {
    fn new(version: u8, packet_type: PacketType) -> Self {
        Self {
            version,
            packet_type,
        }
    }

    fn from_bits(bits: &[u8]) -> Result<Self, ()> {
        let version = from_bits(&bits[0..3]) as u8;
        let packet_type = PacketType::from_bits(&bits[3..])?;
        Ok(Self::new(version, packet_type))
    }

    fn len(&self) -> usize {
        3 + self.packet_type.len()
    }
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
        assert_eq!(
            packet.packet_type,
            PacketType::Literal(LiteralPacket::new(2021, 15))
        );
    }

    #[test]
    fn test_operator_packet_total_length_from_str() {
        let p1 = Packet::from_bits(&[1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0]).unwrap();
        let p2 = Packet::from_bits(&[0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0]).unwrap();
        let packet = Packet::from_str("38006F45291200").unwrap();

        assert_eq!(packet.version, 1);
        assert_eq!(
            packet.packet_type,
            PacketType::Operator((LengthTypeId::TotalLength, vec![p1, p2]))
        );
    }
}
