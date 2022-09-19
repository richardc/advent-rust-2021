use std::collections::HashMap;
use std::io;

#[derive(Default, Debug)]
struct Map {
    paths: HashMap<String, Vec<String>>,
}

impl From<Vec<&str>> for Map {
    fn from(links: Vec<&str>) -> Self {
        let mut map = Map::default();
        for link in links {
            if let Some((from, to)) = link.split_once('-') {
                map.paths
                    .entry(from.to_string())
                    .and_modify(|c| c.push(to.to_string()))
                    .or_insert(vec![to.to_string()]);

                map.paths
                    .entry(to.to_string())
                    .and_modify(|c| c.push(from.to_string()))
                    .or_insert(vec![from.to_string()]);
            }
        }
        map
    }
}

impl Map {
    fn walk(&self, next: &String, from: Vec<String>) -> usize {
        let mut count = 0;
        for exit in self.paths.get(next).unwrap() {
            if exit == "end" {
                count += 1;
                continue;
            }

            if exit == "start" {
                continue;
            }

            if *exit == exit.to_ascii_lowercase() && from.contains(&exit) {
                continue;
            }

            let mut path = from.clone();
            path.push(next.to_string());
            count += self.walk(exit, path);
        }
        count
    }

    fn count_paths(&self) -> usize {
        self.walk(&"start".to_string(), vec![])
    }
}

#[test]
fn test_paths() {
    let example = r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;

    let input = example.trim().split('\n').collect::<Vec<_>>();
    let map = Map::from(input);

    assert_eq!(map.count_paths(), 10);

    let example = r#"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;

    let input = example.trim().split('\n').collect::<Vec<_>>();
    let map = Map::from(input);

    assert_eq!(map.count_paths(), 19);

    let example = r#"
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"#;

    let input = example.trim().split('\n').collect::<Vec<_>>();
    let map = Map::from(input);

    assert_eq!(map.count_paths(), 226);
}

fn main() {
    let lines = io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let input = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();

    let map = Map::from(input);

    println!("{}", map.count_paths())
}
