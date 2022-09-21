use std::io;

use bytes::Bytes;

fn to_binary(c: char) -> &'static [u8] {
    match c {
        '0' => b"0000",
        '1' => b"0001",
        '2' => b"0010",
        '3' => b"0011",
        '4' => b"0100",
        '5' => b"0101",
        '6' => b"0110",
        '7' => b"0111",
        '8' => b"1000",
        '9' => b"1001",
        'A' => b"1010",
        'B' => b"1011",
        'C' => b"1100",
        'D' => b"1101",
        'E' => b"1110",
        'F' => b"1111",
        _ => unreachable!(),
    }
}

fn hex_to_bits(s: &str) -> Bytes {
    Bytes::from(s.chars().flat_map(to_binary).copied().collect::<Vec<u8>>())
}

#[test]
fn test_hex_to_bits() {
    assert_eq!(hex_to_bits("D2FE28"), "110100101111111000101000");
}

#[derive(Debug, PartialEq)]
enum Value {
    Literal(u32),
    Operation { kind: u32, on: Vec<Packet> },
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u32,
    value: Value,
}

fn decode_binary(bits: &[u8]) -> u32 {
    let mut value = 0;
    for b in bits {
        value = (value << 1) + (b - b'0') as u32
    }
    value
}

#[test]
fn test_decode_binary() {
    assert_eq!(decode_binary(b"0"), 0);
    assert_eq!(decode_binary(b"1"), 1);
    assert_eq!(decode_binary(b"10"), 2);
    assert_eq!(decode_binary(b"11"), 3);
    assert_eq!(decode_binary(b"100"), 4);
}

fn decode_literal(bits: &[u8]) -> (&[u8], Value) {
    let mut value = 0;
    let mut used = 0;
    for chunk in bits.chunks(5) {
        value = (value << 4) + decode_binary(&chunk[1..]);
        used += 5;

        if chunk[0] == b'0' {
            break;
        }
    }
    (&bits[used..], Value::Literal(value))
}

fn decode_operation(kind: u32, bits: &[u8]) -> (&[u8], Value) {
    // length type id
    if bits[0] == b'0' {
        let len = decode_binary(&bits[1..16]);
        let mut rem = &bits[16..(16 + len as usize)];
        let mut packets = vec![];
        while rem.len() > 6 {
            let packet: Packet;
            (rem, packet) = decode_packet(rem);
            packets.push(packet);
        }

        (
            &bits[(16 + len as usize)..],
            Value::Operation { kind, on: packets },
        )
    } else {
        let count = decode_binary(&bits[1..12]);
        let mut rem = &bits[12..];
        let mut packets = vec![];
        for _ in 0..count {
            let packet: Packet;
            (rem, packet) = decode_packet(rem);
            packets.push(packet);
        }

        (rem, Value::Operation { kind, on: packets })
    }
}

fn decode(bits: &[u8]) -> (&[u8], Packet) {
    decode_packet(bits)
}

fn decode_packet(bits: &[u8]) -> (&[u8], Packet) {
    let version = decode_binary(&bits[0..3]);
    let packet_type = decode_binary(&bits[3..6]);

    match packet_type {
        4 => {
            let (remainder, value) = decode_literal(&bits[6..]);
            (remainder, Packet { version, value })
        }
        _ => {
            let (remainder, value) = decode_operation(packet_type, &bits[6..]);
            (remainder, Packet { version, value })
        }
    }
}

#[test]
fn test_decode_packet_literal() {
    assert_eq!(
        decode(&hex_to_bits("D2FE28")),
        (
            b"000".as_slice(), // unused padding
            Packet {
                version: 6,
                value: Value::Literal(2021)
            }
        )
    )
}

#[test]
fn test_decode_packet_operator() {
    assert_eq!(
        decode(&hex_to_bits("38006F45291200")),
        (
            b"0000000".as_slice(),
            Packet {
                version: 1,
                value: Value::Operation {
                    kind: 6,
                    on: vec![
                        Packet {
                            version: 6,
                            value: Value::Literal(10)
                        },
                        Packet {
                            version: 2,
                            value: Value::Literal(20)
                        },
                    ]
                }
            }
        )
    );

    assert_eq!(
        decode(&hex_to_bits("EE00D40C823060")),
        (
            b"00000".as_slice(),
            Packet {
                version: 7,
                value: Value::Operation {
                    kind: 3,
                    on: vec![
                        Packet {
                            version: 2,
                            value: Value::Literal(1),
                        },
                        Packet {
                            version: 4,
                            value: Value::Literal(2)
                        },
                        Packet {
                            version: 1,
                            value: Value::Literal(3)
                        },
                    ],
                }
            }
        )
    );
}

fn walk_versions(p: &Packet) -> u32 {
    match &p.value {
        Value::Literal(_) => p.version,
        Value::Operation { on, .. } => p.version + &on.iter().map(|c| walk_versions(c)).sum(),
    }
}

fn sum_versions(s: &str) -> u32 {
    let (_, packet) = decode(&hex_to_bits(s));
    walk_versions(&packet)
}

#[test]
fn test_sum_versions() {
    assert_eq!(sum_versions("8A004A801A8002F478"), 16);
    assert_eq!(sum_versions("620080001611562C8802118E34"), 12);
    assert_eq!(sum_versions("C0015000016115A2E0802F182340"), 23);
    assert_eq!(sum_versions("A0016C880162017C3686B18A3D4780"), 31);
}

fn main() {
    let lines = io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let packet = &lines[0];
    println!("{}", sum_versions(&packet));
}
