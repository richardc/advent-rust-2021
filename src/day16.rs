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

type Number = u64;

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Literal(Number),
    Operation { kind: Number, on: Vec<Packet> },
}

#[derive(Debug, PartialEq, Clone)]
struct Packet {
    version: Number,
    value: Value,
}

fn decode_binary(bits: &[u8]) -> Number {
    let mut value = 0;
    for b in bits {
        value = (value << 1) + (b - b'0') as Number
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

fn decode_operation(kind: Number, bits: &[u8]) -> (&[u8], Value) {
    // length type id
    if bits[0] == b'0' {
        // next 15 bits are a length, parse up to length worth of subpackets
        let len = decode_binary(&bits[1..16]);
        let mut rem = &bits[16..(16 + len as usize)];
        let mut packets = vec![];
        // at least a version+kind+4 (smallest value) left to parse
        while rem.len() > 10 {
            let packet: Packet;
            (rem, packet) = decode_packet(rem);
            packets.push(packet);
        }

        (
            &bits[(16 + len as usize)..],
            Value::Operation { kind, on: packets },
        )
    } else {
        // next 11 bits are a count, parse count subpackets
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

fn walk_versions(p: &Packet) -> Number {
    match &p.value {
        Value::Literal(_) => p.version,
        Value::Operation { on, .. } => p.version + on.iter().map(walk_versions).sum::<Number>(),
    }
}

#[aoc(day16, part1)]
fn sum_versions(s: &str) -> Number {
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

fn eval_list(on: &[Packet]) -> impl Iterator<Item = Number> + '_ {
    on.iter().map(|p| {
        if let Value::Literal(v) = eval(p.value.clone()) {
            v
        } else {
            unreachable!()
        }
    })
}

fn eval(v: Value) -> Value {
    match v {
        Value::Literal(_) => v,
        Value::Operation { kind, on, .. } => match kind {
            // sum
            0 => Value::Literal(eval_list(&on).sum()),
            // product
            1 => Value::Literal(eval_list(&on).product()),
            // min
            2 => Value::Literal(eval_list(&on).min().unwrap()),
            // max
            3 => Value::Literal(eval_list(&on).max().unwrap()),
            // greater-than
            5 => {
                let mut values = eval_list(&on);
                let first = values.next().unwrap();
                let second = values.next().unwrap();
                Value::Literal(if first > second { 1 } else { 0 })
            }
            // 6 => (), // less-than
            6 => {
                let mut values = eval_list(&on);
                let first = values.next().unwrap();
                let second = values.next().unwrap();
                Value::Literal(if first < second { 1 } else { 0 })
            }
            // 7 => (), // equals
            7 => {
                let mut values = eval_list(&on);
                let first = values.next().unwrap();
                let second = values.next().unwrap();
                Value::Literal(if first == second { 1 } else { 0 })
            }
            _ => unreachable!(),
        },
    }
}

impl Packet {
    #[allow(dead_code)] // used to construct eval tests
    fn literal(n: Number) -> Self {
        Packet {
            version: 42,
            value: Value::Literal(n),
        }
    }
}

#[test]
fn test_eval() {
    assert_eq!(
        eval(Value::Operation {
            kind: 0,
            on: vec![
                Packet::literal(1),
                Packet::literal(1),
                Packet::literal(1),
                Packet::literal(1)
            ],
        }),
        Value::Literal(4),
        "sum(1,1,1,1) == 4"
    );

    assert_eq!(
        eval(Value::Operation {
            kind: 0,
            on: vec![Packet::literal(128)],
        }),
        Value::Literal(128),
        "sum(128) == 128"
    );

    assert_eq!(
        eval(Value::Operation {
            kind: 1,
            on: vec![
                Packet::literal(2),
                Packet::literal(3),
                Packet::literal(4),
                Packet::literal(5)
            ]
        }),
        Value::Literal(120),
        "product(2,3,4,5) == 120"
    );

    assert_eq!(
        eval(Value::Operation {
            kind: 2,
            on: vec![Packet::literal(200)],
        }),
        Value::Literal(200),
        "product(200) == 200"
    );
}

#[aoc(day16, part2)]
fn eval_wrapped(s: &str) -> Number {
    let bits = hex_to_bits(s);
    let (rem, packet) = decode(&bits);
    assert!(rem.iter().all(|&c| c == b'0'), "Leftover unparsed!");
    if let Value::Literal(v) = eval(packet.value) {
        return v;
    }
    unreachable!();
}

#[test]
fn test_eval_wrapped() {
    assert_eq!(eval_wrapped("C200B40A82"), 3, "1+2 == 3");
    assert_eq!(eval_wrapped("04005AC33890"), 54, "6*9 == 54");
    assert_eq!(eval_wrapped("880086C3E88112"), 7, "min(7,8,9) == 7");
    assert_eq!(eval_wrapped("CE00C43D881120"), 9, "max(7,8,9) == 9");
    assert_eq!(eval_wrapped("D8005AC2A8F0"), 1, "5 < 15 == 1");
    assert_eq!(eval_wrapped("F600BC2D8F"), 0, "5 > 15 == 0");
    assert_eq!(eval_wrapped("9C005AC2F8F0"), 0, "5 != 15 == 0");
    assert_eq!(
        eval_wrapped("9C0141080250320F1802104A08"),
        1,
        "(1 + 3) == (2 * 2)"
    );
}
