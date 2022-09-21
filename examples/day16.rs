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
    Bytes::from(
        s.chars()
            .map(to_binary)
            .flatten()
            .map(|&x| x)
            .collect::<Vec<u8>>(),
    )
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

fn decode_literal(bits: &[u8]) -> Value {
    let mut value = 0;
    for chunk in bits.chunks(5) {
        value = (value << 4) + decode_binary(&chunk[1..]);

        if chunk[0] == b'0' {
            break;
        }
    }
    Value::Literal(value)
}

fn decode_operation(kind: u32, bits: &[u8]) -> Value {
    // length type id
    if bits[0] == b'0' {
        let len = decode_binary(&bits[1..16]);
        dbg!(len);
        return Value::Operation {
            kind: kind,
            on: vec![decode_packet(&bits[16..(16 + len as usize)])],
        };
    } else {
        let count = decode_binary(&bits[1..12]);
    }
    Value::Operation {
        kind: kind,
        on: vec![],
    }
}

fn decode(hex: &str) -> Packet {
    let bits = hex_to_bits(hex);
    decode_packet(&bits)
}

fn decode_packet(bits: &[u8]) -> Packet {
    let version = decode_binary(&bits[0..3]);
    let packet_type = decode_binary(&bits[3..6]);

    match packet_type {
        4 => Packet {
            version: version,
            value: decode_literal(&bits[6..]),
        },
        _ => Packet {
            version: version,
            value: decode_operation(packet_type, &bits[6..]),
        },
    }
}

#[test]
fn test_decode_packet_literal() {
    assert_eq!(
        decode("D2FE28"),
        Packet {
            version: 6,
            value: Value::Literal(2021)
        }
    )
}

#[test]
fn test_decode_packet_operator() {
    assert_eq!(
        decode("38006F45291200"),
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
}

fn main() {}
