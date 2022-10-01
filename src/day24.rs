use std::collections::HashMap;

type Register = char;
type Value = i64;

#[derive(Debug, Clone, Copy)]
enum Argument {
    Register(char),
    Value(Value),
}

impl From<&str> for Argument {
    fn from(input: &str) -> Self {
        if "wxyz".contains(input) {
            Argument::Register(input.chars().next().unwrap())
        } else {
            Argument::Value(input.parse().unwrap())
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inp(Register),
    Add(Register, Argument),
    Mul(Register, Argument),
    Div(Register, Argument),
    Mod(Register, Argument),
    Eql(Register, Argument),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut parts = input.split_ascii_whitespace();
        let keyword = parts.next().unwrap();
        let dest = parts.next().unwrap().chars().next().unwrap();
        match keyword {
            "inp" => Instruction::Inp(dest),
            _ => {
                let src = Argument::from(parts.next().unwrap());
                match keyword {
                    "add" => Instruction::Add(dest, src),
                    "mul" => Instruction::Mul(dest, src),
                    "div" => Instruction::Div(dest, src),
                    "mod" => Instruction::Mod(dest, src),
                    "eql" => Instruction::Eql(dest, src),
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Alu {
    w: Value,
    x: Value,
    y: Value,
    z: Value,
    instructions: Vec<Instruction>,
}

impl Alu {
    fn new(instructions: &[Instruction]) -> Self {
        Alu {
            w: 0,
            x: 0,
            y: 0,
            z: 0,

            instructions: instructions.to_vec(),
        }
    }

    fn fetch(&self, src: Argument) -> Value {
        match src {
            Argument::Value(v) => v,
            Argument::Register(r) => match r {
                'w' => self.w,
                'x' => self.x,
                'y' => self.y,
                'z' => self.z,
                _ => unreachable!(),
            },
        }
    }

    fn store(&mut self, dest: Register, value: Value) {
        match dest {
            'w' => self.w = value,
            'x' => self.x = value,
            'y' => self.y = value,
            'z' => self.z = value,
            _ => unreachable!(),
        }
    }

    fn reset(&mut self) {
        self.w = 0;
        self.x = 0;
        self.y = 0;
        self.z = 0;
    }

    fn run(&mut self, input: &[Value]) -> bool {
        let mut input = input.iter();

        for instruction in self.instructions.clone() {
            match instruction {
                Instruction::Inp(dest) => self.store(dest, *input.next().unwrap()),
                Instruction::Add(dest, src) => {
                    self.store(dest, self.fetch(Argument::Register(dest)) + self.fetch(src))
                }
                Instruction::Mul(dest, src) => {
                    self.store(dest, self.fetch(Argument::Register(dest)) * self.fetch(src))
                }
                Instruction::Div(dest, src) => {
                    let denominator = self.fetch(src);
                    if denominator == 0 {
                        return false;
                    }
                    self.store(dest, self.fetch(Argument::Register(dest)) / denominator)
                }
                Instruction::Mod(dest, src) => {
                    let numerator = self.fetch(Argument::Register(dest));
                    let denominator = self.fetch(src);
                    if numerator < 0 || denominator <= 0 {
                        return false;
                    }
                    self.store(dest, numerator % denominator)
                }
                Instruction::Eql(dest, src) => self.store(
                    dest,
                    if self.fetch(Argument::Register(dest)) == self.fetch(src) {
                        1
                    } else {
                        0
                    },
                ),
            };
        }
        true
    }
}

#[test]
fn test_alu_example1() {
    let mut alu = Alu::new(&generate("inp x\nmul x -1"));
    alu.run(&[42]);
    assert_eq!(alu.x, -42);
}

#[test]
fn test_alu_example2() {
    let mut alu = Alu::new(&generate("inp z\ninp x\nmul z 3\neql z x"));
    alu.run(&[1, 3]);
    assert_eq!(alu.x, 3);
    assert_eq!(alu.z, 1);

    alu.run(&[3, 3]);
    assert_eq!(alu.x, 3);
    assert_eq!(alu.z, 0);
}

#[test]
fn test_alu_example3() {
    let mut alu = Alu::new(&generate(include_str!("day24_example3.txt")));
    assert!(alu.run(&[0b0000]));
    assert_eq!([alu.w, alu.x, alu.y, alu.z], [0, 0, 0, 0]);

    alu.reset();
    assert!(alu.run(&[0b0001]));
    assert_eq!([alu.w, alu.x, alu.y, alu.z], [0, 0, 0, 1]);

    alu.reset();
    assert!(alu.run(&[0b1001]));
    assert_eq!([alu.w, alu.x, alu.y, alu.z], [1, 0, 0, 1]);
}

fn find_model_number(program: &[Instruction], keep: fn(Value, Value) -> bool) -> Value {
    let programs = program.split(|i| matches!(i, Instruction::Inp(_))).skip(1);

    let mut digits: HashMap<Value, Value> = HashMap::new();
    digits.insert(0, 0);

    for (_p, program) in programs.enumerate() {
        let mut new_digits: HashMap<Value, Value> = HashMap::new();
        let mut alu = Alu::new(program);
        for candidate in 1..=9 {
            for (z, max) in &digits {
                alu.reset();
                // All the subprograms start: inp w
                alu.z = *z;
                alu.w = candidate;
                alu.run(&[]);

                let val = max * 10 + candidate;
                if keep(*new_digits.get(&alu.z).unwrap_or(&0), val) {
                    new_digits.insert(alu.z, val);
                }
            }
        }
        digits = new_digits;
        // println!("{} digits checked, {} entries", _p, digits.len());
    }

    digits[&0]
}
#[aoc_generator(day24)]
fn generate(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

#[aoc(day24, part1)]
fn largest_model_number(program: &[Instruction]) -> Value {
    find_model_number(program, |old, new| old < new)
}

#[aoc(day24, part2)]
fn smallest_model_number(program: &[Instruction]) -> Value {
    find_model_number(program, |old, new| old > new || old == 0)
}
