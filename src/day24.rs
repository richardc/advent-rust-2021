enum Argument {
    Register(char),
    Value(i32),
}

enum Instruction {
    Inp(Argument),
    Add(Argument, Argument),
    Mul(Argument, Argument),
    Div(Argument, Argument),
    Mod(Argument, Argument),
    Eql(Argument, Argument),
}

#[aoc_generator(day24)]
fn generate(input: &str) -> Vec<Instruction> {
    vec![]
}

#[aoc(day24, part1)]
fn largest_model_number(input: &[Instruction]) -> usize {
    0
}
