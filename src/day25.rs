struct Cucumbers;

#[aoc_generator(day25)]
fn generate(_input: &str) -> Cucumbers {
    Cucumbers {}
}

#[aoc(day25, part1)]
fn safe_iteration(_cucumbers: &Cucumbers) -> usize {
    0
}

#[test]
fn test_safe_iteration() {
    assert_eq!(
        safe_iteration(&generate(include_str!("day25_example.txt"))),
        58
    );
}
