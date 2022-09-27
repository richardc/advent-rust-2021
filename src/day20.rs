use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
struct Image {
    background: char,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
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

    for (y, line) in all[2..].iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                pixels.insert([x as i32, y as i32]);
            }
        }
    }

    let mut image = Image {
        background: '0',
        algorithm,
        pixels,
        x_min: 0,
        x_max: 0,
        y_min: 0,
        y_max: 0,
    };
    image.set_bounds();
    image
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
            if x < self.x_min || x > self.x_max || y < self.y_min || y > self.y_max {
                bits.push(self.background)
            } else if self.pixels.contains(&[x, y]) {
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
    let mut image = generate(include_str!("day20_example.txt"));
    assert_eq!(image.neighbour_string(0, 0), "000010010");
    assert_eq!(image.neighbour_string(2, 2), "000100010");

    // When out of bounds, we should use the background char, which defaults to 0
    assert_eq!(image.neighbour_string(-1000, -1000), "000000000");
    image.background = '1';
    assert_eq!(image.neighbour_string(-1000, -1000), "111111111");
}

impl Image {
    fn set_bounds(&mut self) {
        let (x_min, x_max) = self
            .pixels
            .iter()
            .map(|[x, _]| x)
            .minmax()
            .into_option()
            .unwrap();
        self.x_min = *x_min;
        self.x_max = *x_max;

        let (y_min, y_max) = self
            .pixels
            .iter()
            .map(|[_, y]| y)
            .minmax()
            .into_option()
            .unwrap();
        self.y_min = *y_min;
        self.y_max = *y_max;
    }
}

impl Image {
    fn step(&mut self) {
        let mut next = HashSet::new();

        for (x, y) in iproduct!(
            self.x_min - 1..=self.x_max + 1,
            self.y_min - 1..=self.y_max + 1
        ) {
            let index = usize::from_str_radix(&self.neighbour_string(x, y), 2).unwrap();
            if self.algorithm[index] == 1 {
                next.insert([x, y]);
            }
        }

        // calculate next background value
        let index = usize::from_str_radix(
            &self.neighbour_string(self.x_max + 1000, self.y_max + 1000),
            2,
        )
        .unwrap();
        self.background = if self.algorithm[index] == 1 { '1' } else { '0' };
        self.pixels = next;
        self.set_bounds();
    }
}

fn apply_steps(image: &Image, count: usize) -> usize {
    let mut image = (*image).clone();
    for _ in 0..count {
        image.step();
    }
    image.pixel_count()
}

#[test]
fn test_apply_steps() {
    let image = generate(include_str!("day20_example.txt"));
    assert_eq!(apply_steps(&image, 2), 35);
    assert_eq!(apply_steps(&image, 50), 3351);
}

#[aoc(day20, part1)]
fn enhance(image: &Image) -> usize {
    apply_steps(image, 2)
}

#[aoc(day20, part2)]
fn really_enhance(image: &Image) -> usize {
    apply_steps(image, 50)
}
