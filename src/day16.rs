#![allow(unused)]
use itertools::Itertools;
use std::{str::FromStr, usize};

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
    value: usize,
    length: usize,
}

impl LiteralPacket {
    fn new(value: usize, length: usize) -> LiteralPacket {
        LiteralPacket { value, length }
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

    fn eval(&self) -> usize {
        self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
enum OperatorPacket {
    TotalLength(TotalLengthPacket),
    CountSubpackets(CountSubpacketsPacket),
}

impl OperatorPacket {
    const BIT_CONSUMPTION: usize = 1;
    fn from_bits(bits: &[u8]) -> Result<Self, ()> {
        match bits[0] {
            0 => Ok(OperatorPacket::TotalLength(TotalLengthPacket::from_bits(
                &bits[Self::BIT_CONSUMPTION..],
            )?)),
            1 => Ok(OperatorPacket::CountSubpackets(
                CountSubpacketsPacket::from_bits(&bits[Self::BIT_CONSUMPTION..])?,
            )),
            _ => Err(()),
        }
    }

    fn len(&self) -> usize {
        match self {
            OperatorPacket::TotalLength(p) => p.len() + Self::BIT_CONSUMPTION,
            OperatorPacket::CountSubpackets(p) => p.len() + Self::BIT_CONSUMPTION,
        }
    }

    fn sum_packet_versions(&self) -> usize {
        match self {
            OperatorPacket::TotalLength(p) => p.sum_packet_versions(),
            OperatorPacket::CountSubpackets(p) => p.sum_packet_versions(),
        }
    }

    fn eval(&self) -> Vec<usize> {
        match self {
            OperatorPacket::TotalLength(p) => p.eval(),
            OperatorPacket::CountSubpackets(p) => p.eval(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct CountSubpacketsPacket {
    count: usize,
    subpackets: Vec<Packet>,
}

impl CountSubpacketsPacket {
    fn new(count: usize, subpackets: Vec<Packet>) -> CountSubpacketsPacket {
        CountSubpacketsPacket { count, subpackets }
    }

    fn from_bits(bits: &[u8]) -> Result<CountSubpacketsPacket, ()> {
        let n_subpackets = from_bits(&bits[..11]) as usize;
        let mut subpackets = Vec::with_capacity(n_subpackets);
        let mut data = &bits[11..];
        for _ in 0..n_subpackets {
            let packet = Packet::from_bits(data)?;
            data = &data[packet.len()..];
            subpackets.push(packet);
        }

        Ok(CountSubpacketsPacket::new(n_subpackets, subpackets))
    }

    fn len(&self) -> usize {
        self.subpackets.iter().map(|p| p.len()).sum::<usize>() + 11
    }

    fn sum_packet_versions(&self) -> usize {
        self.subpackets
            .iter()
            .map(|p| p.sum_packet_versions())
            .sum::<usize>()
    }

    fn eval(&self) -> Vec<usize> {
        self.subpackets.iter().map(|p| p.eval()).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TotalLengthPacket {
    length: usize,
    subpackets: Vec<Packet>,
}

impl TotalLengthPacket {
    fn new(length: usize, subpackets: Vec<Packet>) -> TotalLengthPacket {
        TotalLengthPacket { length, subpackets }
    }

    fn from_bits(bits: &[u8]) -> Result<TotalLengthPacket, ()> {
        let length = from_bits(&bits[0..15]) as usize;
        let mut data = &bits[15..length + 15];
        let mut packets = Vec::new();
        while !data.is_empty() {
            let packet = Packet::from_bits(data)?;
            data = &data[packet.len()..];
            packets.push(packet);
        }
        Ok(TotalLengthPacket::new(length, packets))
    }

    fn len(&self) -> usize {
        self.length + 15
    }

    fn sum_packet_versions(&self) -> usize {
        self.subpackets
            .iter()
            .map(|p| p.sum_packet_versions())
            .sum::<usize>()
    }

    fn eval(&self) -> Vec<usize> {
        self.subpackets.iter().map(|p| p.eval()).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PacketType {
    Literal(LiteralPacket),
    Sum(OperatorPacket),
    Product(OperatorPacket),
    Minimum(OperatorPacket),
    Maximum(OperatorPacket),
    GreaterThan(OperatorPacket),
    LessThan(OperatorPacket),
    Equal(OperatorPacket),
}

impl PacketType {
    const BIT_CONSUMPTION: usize = 3;

    fn from_bits(bits: &[u8]) -> Result<PacketType, ()> {
        match from_bits(&bits[0..Self::BIT_CONSUMPTION]) {
            0 => Ok(Self::Sum(OperatorPacket::from_bits(
                &bits[Self::BIT_CONSUMPTION..],
            )?)),
            1 => Ok(Self::Product(OperatorPacket::from_bits(
                &bits[Self::BIT_CONSUMPTION..],
            )?)),
            2 => Ok(Self::Minimum(OperatorPacket::from_bits(
                &bits[Self::BIT_CONSUMPTION..],
            )?)),
            3 => Ok(Self::Maximum(OperatorPacket::from_bits(
                &bits[Self::BIT_CONSUMPTION..],
            )?)),
            4 => Ok(PacketType::Literal(LiteralPacket::from_bits(
                &bits[Self::BIT_CONSUMPTION..],
            )?)),
            5 => Ok(PacketType::GreaterThan(OperatorPacket::from_bits(
                &bits[Self::BIT_CONSUMPTION..],
            )?)),
            6 => Ok(PacketType::LessThan(OperatorPacket::from_bits(
                &bits[Self::BIT_CONSUMPTION..],
            )?)),
            7 => Ok(PacketType::Equal(OperatorPacket::from_bits(
                &bits[Self::BIT_CONSUMPTION..],
            )?)),

            _ => Err(()),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Literal(l) => l.len() + Self::BIT_CONSUMPTION,
            Self::Sum(o)
            | Self::Product(o)
            | Self::Minimum(o)
            | Self::Maximum(o)
            | Self::GreaterThan(o)
            | Self::LessThan(o)
            | Self::Equal(o) => o.len() + Self::BIT_CONSUMPTION,
        }
    }

    fn sum_packet_versions(&self) -> usize {
        match self {
            PacketType::Literal(l) => 0,
            Self::Sum(o)
            | Self::Product(o)
            | Self::Minimum(o)
            | Self::Maximum(o)
            | Self::GreaterThan(o)
            | Self::LessThan(o)
            | Self::Equal(o) => o.sum_packet_versions(),
        }
    }

    fn eval(&self) -> usize {
        match self {
            PacketType::Literal(l) => l.value,
            Self::Sum(o) => o.eval().iter().sum::<usize>(),
            Self::Product(o) => o.eval().iter().product::<usize>(),
            Self::Minimum(o) => *o.eval().iter().min().unwrap(),
            Self::Maximum(o) => *o.eval().iter().max().unwrap(),
            Self::GreaterThan(o) => (o.eval()[0] > o.eval()[1]) as usize,
            Self::LessThan(o) => (o.eval()[0] < o.eval()[1]) as usize,
            Self::Equal(o) => (o.eval()[0] == o.eval()[1]) as usize,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

impl Packet {
    const BIT_CONSUMPTION: usize = 3;

    fn new(version: u8, packet_type: PacketType) -> Self {
        Self {
            version,
            packet_type,
        }
    }

    fn from_bits(bits: &[u8]) -> Result<Self, ()> {
        let version = from_bits(&bits[0..Self::BIT_CONSUMPTION]) as u8;
        let packet_type = PacketType::from_bits(&bits[Self::BIT_CONSUMPTION..])?;
        Ok(Self::new(version, packet_type))
    }

    fn len(&self) -> usize {
        Self::BIT_CONSUMPTION + self.packet_type.len()
    }

    fn sum_packet_versions(&self) -> usize {
        self.version as usize + self.packet_type.sum_packet_versions()
    }

    fn eval(&self) -> usize {
        self.packet_type.eval()
    }
}

impl FromStr for Packet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = hex_to_bin(s)?;
        Self::from_bits(&bits)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    const INPUT_FILE: &str = "input/day16.txt";

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
            PacketType::LessThan(OperatorPacket::TotalLength(TotalLengthPacket::new(
                27,
                vec![p1, p2]
            )))
        );
    }

    #[test]
    fn test_operator_packet_count_subpackets_from_str() {
        let packet = Packet::from_str("EE00D40C823060").unwrap();
        assert_eq!(packet.version, 7);
        assert_eq!(
            packet.packet_type,
            PacketType::Maximum(OperatorPacket::CountSubpackets(CountSubpacketsPacket::new(
                3,
                vec![
                    Packet::from_bits(&[0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1]).unwrap(),
                    Packet::from_bits(&[1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0]).unwrap(),
                    Packet::from_bits(&[0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1]).unwrap(),
                ]
            )))
        );
    }

    #[test]
    fn test_packet_version_sum() {
        let packet = Packet::from_str("8A004A801A8002F478").unwrap();
        assert_eq!(packet.sum_packet_versions(), 16);

        let packet = Packet::from_str("620080001611562C8802118E34").unwrap();
        assert_eq!(packet.sum_packet_versions(), 12);

        let packet = Packet::from_str("C0015000016115A2E0802F182340").unwrap();
        assert_eq!(packet.sum_packet_versions(), 23);

        let packet = Packet::from_str("A0016C880162017C3686B18A3D4780").unwrap();
        assert_eq!(packet.sum_packet_versions(), 31);
    }

    #[test]
    fn part_1() {
        let input = read_to_string(INPUT_FILE).unwrap();
        let packet = Packet::from_str(&input).unwrap();
        println!("Part 1: {}", packet.sum_packet_versions());
    }

    #[test]
    fn test_part_2() {
        let packet = Packet::from_str("C200B40A82").unwrap();
        assert_eq!(packet.eval(), 3);

        let packet = Packet::from_str("04005AC33890").unwrap();
        assert_eq!(packet.eval(), 54);

        let packet = Packet::from_str("880086C3E88112").unwrap();
        assert_eq!(packet.eval(), 7);

        let packet = Packet::from_str("CE00C43D881120").unwrap();
        assert_eq!(packet.eval(), 9);

        let packet = Packet::from_str("D8005AC2A8F0").unwrap();
        assert_eq!(packet.eval(), 1);

        let packet = Packet::from_str("F600BC2D8F").unwrap();
        assert_eq!(packet.eval(), 0);

        let packet = Packet::from_str("9C005AC2F8F0").unwrap();
        assert_eq!(packet.eval(), 0);

        let packet = Packet::from_str("9C0141080250320F1802104A08").unwrap();
        assert_eq!(packet.eval(), 1);
    }

    #[test]
    fn part_2() {
        let input = read_to_string(INPUT_FILE).unwrap();
        let packet = Packet::from_str(&input).unwrap();
        println!("Part 2: {}", packet.eval());
    }
}
