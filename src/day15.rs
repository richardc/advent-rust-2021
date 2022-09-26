use itertools::Itertools;
use ndarray::prelude::*;
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug)]
struct Puzzle {
    map: Array2<u8>,
}

impl FromIterator<String> for Puzzle {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let lines = iter.into_iter().collect::<Vec<_>>();
        let x = lines[0].len();
        let y = lines.len();
        Puzzle {
            map: Array::from_shape_vec(
                (y, x),
                lines
                    .iter()
                    .flat_map(|s| s.chars().map(|c| c as u8 - b'0'))
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(usize, usize);

#[derive(PartialEq, Eq)]
struct State {
    cost: usize,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Puzzle {
    fn adjacent(&self, Point(x, y): Point) -> Vec<Point> {
        let mut adj = vec![];
        if x > 1 {
            adj.push(Point(x - 1, y));
        }

        if x < self.map.dim().0 - 1 {
            adj.push(Point(x + 1, y));
        }

        if y > 1 {
            adj.push(Point(x, y - 1));
        }

        if y < self.map.dim().1 - 1 {
            adj.push(Point(x, y + 1));
        }

        adj
    }

    fn embiggen(&self) -> Self {
        let (x, y) = self.map.dim();
        let repeat = 5;
        let mut new_map = Array::zeros((x * repeat, y * repeat));
        (0..repeat).cartesian_product(0..repeat).for_each(|(i, j)| {
            let mut into = new_map.slice_mut(s!(i * x..(i + 1) * x, j * y..(j + 1) * y,));
            into.assign(&(self.map.clone() + i as u8 + j as u8));
        });
        // Wrap 9 to 1
        new_map.iter_mut().for_each(|x| {
            while *x > 9 {
                *x -= 9
            }
        });
        Puzzle { map: new_map }
    }

    fn shortest_path(&self) -> usize {
        let start = Point(0, 0);
        let end = Point(self.map.dim().0 - 1, self.map.dim().1 - 1);

        // Dijkstra's algorithm - from the manpage for std::collections::binaryheap
        let mut dist = Array::from_elem(self.map.dim(), usize::MAX);
        dist[[start.0, start.1]] = 0;

        let mut queue = BinaryHeap::new();
        queue.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = queue.pop() {
            if position == end {
                return cost;
            }

            if cost > dist[[position.0, position.1]] {
                continue;
            }

            for location in self.adjacent(position) {
                let next = State {
                    cost: cost + self.map[[location.0, location.1]] as usize,
                    position: location,
                };

                if next.cost < dist[[location.0, location.1]] {
                    dist[[location.0, location.1]] = next.cost;
                    queue.push(next);
                }
            }
        }
        unreachable!()
    }
}

#[test]
fn test_puzzle() {
    let example = r#"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    let lines = example.trim().split('\n').map(|s| s.to_string());
    let puzzle = Puzzle::from_iter(lines);
    assert_eq!(puzzle.map.dim(), (10, 10));

    assert_eq!(puzzle.map[[0, 0]], 1);
    assert_eq!(puzzle.map[[0, 9]], 2);
    assert_eq!(puzzle.shortest_path(), 40);

    let example_embiggened = r#"
11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479
"#;

    let example_embiggened =
        Puzzle::from_iter(example_embiggened.trim().split('\n').map(|s| s.to_string()));

    let embiggened = puzzle.embiggen();
    assert_eq!(embiggened.map, example_embiggened.map);
    assert_eq!(embiggened.map.dim(), (50, 50));
    assert_eq!(embiggened.map[[10, 0]], 2);
    assert_eq!(embiggened.shortest_path(), 315);
}

#[test]
fn test_puzzle_embiggen() {
    let puzzle = Puzzle::from_iter([String::from("18")]);
    assert_eq!(puzzle.map.dim(), (1, 2));
    assert_eq!(puzzle.map.slice(s!(0, ..)), aview1(&[1, 8]));

    let embiggened = puzzle.embiggen();
    assert_eq!(embiggened.map.dim(), (5, 10));
    assert_eq!(
        embiggened.map.slice(s!(0..2, ..)),
        aview2(&[
            [1, 8, 2, 9, 3, 1, 4, 2, 5, 3],
            [2, 9, 3, 1, 4, 2, 5, 3, 6, 4]
        ])
    );
}

#[aoc_generator(day15)]
fn generate(input: &str) -> Puzzle {
    Puzzle::from_iter(input.lines().map(|x| x.to_string()))
}

#[aoc(day15, part1)]
fn shortest_path(p: &Puzzle) -> usize {
    p.shortest_path()
}

#[aoc(day15, part2)]
fn shortest_path_expanded(p: &Puzzle) -> usize {
    p.embiggen().shortest_path()
}
