#[derive(Debug)]
struct Heightmap {
    map: Vec<Vec<u8>>,
}

impl From<&[&str]> for Heightmap {
    fn from(input: &[&str]) -> Self {
        Heightmap {
            map: input
                .iter()
                .map(|x| x.chars().map(|c| c as u8 - b'0').collect())
                .collect(),
        }
    }
}

impl Heightmap {
    fn neighbours(&self, x: usize, y: usize) -> Vec<u8> {
        let mut res = vec![];
        if y > 0 {
            // above
            res.push(self.map[x][y - 1]);
        }
        if x > 0 {
            // left
            res.push(self.map[x - 1][y]);
        }
        if y < self.map[0].len() - 1 {
            // below
            res.push(self.map[x][y + 1]);
        }
        if x < self.map.len() - 1 {
            // right
            res.push(self.map[x + 1][y]);
        }
        res
    }

    fn low_points(&self) -> Vec<u8> {
        let mut low = vec![];
        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                let val = self.map[x][y];
                if self.neighbours(x, y).iter().all(|&x| val < x) {
                    low.push(val)
                }
            }
        }
        low
    }

    fn basin_sizes(&self) -> Vec<u32> {
        fn basin(m: &mut Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
            m[x][y] = 9;
            let mut c = 1;
            if x > 0 && m[x - 1][y] != 9 {
                c += basin(m, x - 1, y);
            }
            if x < m.len() - 1 && m[x + 1][y] != 9 {
                c += basin(m, x + 1, y);
            }
            if y > 0 && m[x][y - 1] != 9 {
                c += basin(m, x, y - 1);
            }
            if y < m[0].len() - 1 && m[x][y + 1] != 9 {
                c += basin(m, x, y + 1);
            }
            c
        }

        let mut m = self.map.clone();
        let mut sizes = vec![];
        for x in 0..m.len() - 1 {
            for y in 0..m[x].len() - 1 {
                if m[x][y] != 9 {
                    sizes.push(basin(&mut m, x, y))
                }
            }
        }
        sizes
    }
}

#[test]
fn test_low_points() {
    let example = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    let input = example.trim().split('\n').collect::<Vec<_>>();
    let slice: &[&str] = &input;
    let map = Heightmap::from(slice);
    assert_eq!(map.low_points(), [1, 0, 5, 5]);
    assert_eq!(risk_level(&map), 15);
    assert_eq!(map.basin_sizes(), [3, 9, 14, 9]);
}

fn risk_level(heightmap: &Heightmap) -> u32 {
    heightmap.low_points().iter().map(|&x| x as u32 + 1).sum()
}

use itertools::Itertools;

fn biggest_basins(map: &Heightmap) -> u32 {
    map.basin_sizes()
        .into_iter()
        .sorted()
        .rev()
        .take(3)
        .product()
}

use std::io;

fn main() {
    let lines = io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let input = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    let slice: &[&str] = &input;

    let map = Heightmap::from(slice);
    println!("{}", risk_level(&map));
    println!("{}", biggest_basins(&map));
}
