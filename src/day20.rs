use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
struct Image {
    algorithm: [u8; 512],
    pixels: HashSet<[i32; 2]>,
}

impl Image {
    fn pixel_count(&self) -> usize {
        self.pixels.len()
    }
}

fn to_bit(c: char) -> u8 {
    match c {
        '#' => 1,
        '.' => 0,
        _ => unreachable!(),
    }
}

#[aoc_generator(day20)]
fn generate(input: &str) -> Image {
    let all = input.lines().collect_vec();
    let algorithm = all[0].chars().map(to_bit).collect_vec().try_into().unwrap();
    let mut pixels = HashSet::new();

    for (y, line) in (&all[2..]).iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                pixels.insert([x as i32, y as i32]);
            }
        }
    }

    Image { algorithm, pixels }
}

#[test]
fn test_generate() {
    let image = generate(include_str!("day20_example.txt"));
    assert_eq!(image.algorithm[0..4], [0, 0, 1, 0]);
    assert_eq!(image.pixels.len(), 10);
}

#[test]
fn test_image_pixel_count() {
    let image = generate(include_str!("day20_example.txt"));
    assert_eq!(image.pixel_count(), 10);
}

use itertools::iproduct;

impl Image {
    fn neighbour_string(&self, x: i32, y: i32) -> String {
        let mut bits = String::with_capacity(9);
        for (y, x) in iproduct!((y - 1)..=(y + 1), (x - 1)..=(x + 1)) {
            if self.pixels.contains(&[x, y]) {
                bits.push('1');
            } else {
                bits.push('0');
            }
        }
        bits
    }
}

#[test]
fn test_neighbour_string() {
    let image = generate(include_str!("day20_example.txt"));
    assert_eq!(image.neighbour_string(-1000, -1000), "000000000");
    assert_eq!(image.neighbour_string(0, 0), "000010010");
    assert_eq!(image.neighbour_string(2, 2), "000100010");
}

impl Image {
    fn step(&mut self) {
        let (&x_min, &x_max) = self
            .pixels
            .iter()
            .map(|[x, _]| x)
            .minmax()
            .into_option()
            .unwrap();
        let (&y_min, &y_max) = self
            .pixels
            .iter()
            .map(|[_, y]| y)
            .minmax()
            .into_option()
            .unwrap();
        let mut next = HashSet::new();

        for (x, y) in iproduct!(x_min - 1..=x_max + 1, y_min - 1..=y_max + 1) {
            let index = usize::from_str_radix(&self.neighbour_string(x, y), 2).unwrap();
            if self.algorithm[index] == 1 {
                next.insert([x, y]);
            }
        }

        self.pixels = next;
    }
}

fn apply_steps(image: &Image, count: usize) -> usize {
    let mut image = (*image).clone();
    for _ in 0..count {
        image.step();
    }
    image.pixel_count()
}

#[aoc(day20, part1)]
fn two_step(image: &Image) -> usize {
    apply_steps(image, 2)
}

#[test]
fn test_two_step() {
    let image = generate(include_str!("day20_example.txt"));
    assert_eq!(two_step(&image), 35);
}
