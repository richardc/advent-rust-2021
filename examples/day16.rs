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
        dbg!(len);
        let (rem, on) = decode_packet(&bits[16..(16 + len) as usize]);
        return (
            rem,
            Value::Operation {
                kind: kind,
                on: vec![on],
            },
        );
    } else {
        let count = decode_binary(&bits[1..12]);
    }

    (
        bits,
        Value::Operation {
            kind: kind,
            on: vec![],
        },
    )
}

fn decode(bits: &[u8]) -> (&[u8], Packet) {
    decode_packet(&bits)
}

fn decode_packet(bits: &[u8]) -> (&[u8], Packet) {
    let version = decode_binary(&bits[0..3]);
    let packet_type = decode_binary(&bits[3..6]);

    match packet_type {
        4 => {
            let (remainder, value) = decode_literal(&bits[6..]);
            (
                remainder,
                Packet {
                    version: version,
                    value: value,
                },
            )
        }
        _ => {
            let (remainder, value) = decode_operation(packet_type, &bits[6..]);
            (
                remainder,
                Packet {
                    version: version,
                    value: value,
                },
            )
        }
    }
}

#[test]
fn test_decode_packet_literal() {
    assert_eq!(
        decode(&hex_to_bits("D2FE28")),
        (
            b"000".as_slice(),
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
            b"".as_slice(),
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
    )
}

fn main() {}
