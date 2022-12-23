use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input);
    println!("part1: {part1}");
}

type Coord = (isize, isize);
type Offset = (isize, isize);

const OFFSETS: [Offset; 2] = [(-1, 0), (1, 0)];

fn part1(s: &str) -> usize {
    let mut z_collection: BTreeMap<isize, BTreeSet<Coord>> = BTreeMap::new();
    let mut x_collection: BTreeMap<isize, BTreeSet<Coord>> = BTreeMap::new();
    let mut y_collection: BTreeMap<isize, BTreeSet<Coord>> = BTreeMap::new();

    for line in s.lines() {
        let mut iter = line.splitn(3, ',');
        let x = iter
            .next()
            .expect("couldn't get x")
            .parse::<isize>()
            .expect("couldn't parse x");
        let y = iter
            .next()
            .expect("couldn't get y")
            .parse::<isize>()
            .expect("couldn't parse y");
        let z = iter
            .next()
            .expect("couldn't get z")
            .parse::<isize>()
            .expect("couldn't parse z");

        z_collection.entry(z).or_default().insert((x, y));
        x_collection.entry(x).or_default().insert((y, z));
        y_collection.entry(y).or_default().insert((z, x));
    }

    fn neighbour_coord((x, y): &Coord, (ox, oy): Offset) -> Option<Coord> {
        Some((x.checked_add(ox)?, y.checked_add(oy)?))
    }

    fn find_open_faces(collection: &BTreeMap<isize, BTreeSet<Coord>>) -> usize {
        let mut total = 0;
        for coords in collection.values() {
            for coord in coords {
                for offset in OFFSETS {
                    let Some(neighbour) = neighbour_coord(coord, offset) else { continue };
                    if !coords.contains(&neighbour) {
                        total += 1;
                    } else {
                        dbg!(neighbour);
                    }
                }
            }
        }
        total
    }

    find_open_faces(&z_collection) + find_open_faces(&x_collection) + find_open_faces(&y_collection)
}

#[test]
fn part1_works() {
    let input = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;
    assert_eq!(part1(input), 64);
    let input = r#"1,1,1
2,2,2"#;
    assert_eq!(part1(input), 12);

    let input = r#"0,0,0
1,0,0"#;
    assert_eq!(part1(input), 10);
}
