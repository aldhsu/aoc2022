use anyhow::{Context, Error, Result};
use petgraph::algo::astar;
use petgraph::prelude::*;
use petgraph::Graph;
use rayon::prelude::*;
use std::collections::HashSet;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct Map {
    inner: Graph<i32, ()>,
    start: Coord,
    end: Coord,
    x_max: usize,
    starting_points: Vec<Coord>,
}

type Coord = (isize, isize);

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let y_max = s.lines().count();
        let x_max = s
            .lines()
            .next()
            .map(|line| line.chars().count())
            .context("couldn't get first line")?;
        let start = s
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .find_map(|(x, c)| (c == 'S').then_some((x as isize, y as isize)))
            })
            .context("couldn't find start")?;
        let end = s
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .find_map(|(x, c)| (c == 'E').then_some((x as isize, y as isize)))
            })
            .context("couldn't find start")?;

        let map: HashMap<(isize, isize), char> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    let c = match c {
                        'E' => 'z',
                        'S' => 'a',
                        _ => c,
                    };
                    ((x as isize, y as isize), c)
                })
            })
            .collect();

        let starting_points = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    let c = match c {
                        'E' => 'z',
                        'S' => 'a',
                        _ => c,
                    };
                    (c == 'a').then(|| (x as isize, y as isize))
                })
            })
            .collect();

        let edges = map
            .iter()
            .flat_map(|(coord, c)| {
                [(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .iter()
                    .filter_map(|offset| {
                        let new = (coord.0 + offset.0, coord.1 + offset.1);
                        let Some(other) = map.get(&new) else { return None };
                        let diff = (*c as u8) as i8 - ((*other as u8) as i8);
                        (diff >= -1).then_some({
                            (coord_into_u32(coord, x_max), coord_into_u32(&new, x_max))
                        })
                    })
            })
            .collect::<Vec<_>>();

        let inner = Graph::<i32, ()>::from_edges(edges);

        Ok(Self {
            inner,
            start,
            end,
            x_max,
            starting_points,
        })
    }
}

fn coord_into_u32(coord: &Coord, x_max: usize) -> u32 {
    (coord.0 as usize + coord.1 as usize * x_max) as u32
}

impl Map {
    fn find_shortest_path(&self) -> Option<u32> {
        let start = self.coord_into_u32(&self.start);
        let end = self.coord_into_u32(&self.end);
        let (step, path) = astar(
            &self.inner,
            start.into(),
            |finish| finish == end.into(),
            |_| 1,
            |_| 1,
        )?;
        // let mut step_map = HashMap::new();
        // let mut y_max = 0;

        // for (i, p) in path.iter().enumerate() {
        //     let coord = self.u32_into_coord(p.index() as u32);
        //     if coord.1 > y_max {
        //         y_max = coord.1
        //     };
        //     step_map.insert(coord, i);
        // }
        //
        // for y in 0..y_max + 1 {
        //     for x in 0..self.x_max {
        //         if let Some(count) = step_map.get(&(x as isize, y)) {
        //             print!("{:02} ", count);
        //         } else {
        //             print!(".. ");
        //         }
        //     }
        //     println!()
        // }
        //
        Some(step)
    }

    fn find_shortest_path_all_a(&self) -> Option<u32> {
        let end = self.coord_into_u32(&self.end);

        let steps = self
            .starting_points
            .par_iter()
            .filter_map(|start| {
                let Some((step, _)) = astar(
                &self.inner,
                self.coord_into_u32(start).into(),
                |finish| finish == end.into(),
                |_| 1,
                |_| 1,
            ) else { return None };
                Some(step)
            })
            .collect::<Vec<_>>();

        steps.into_iter().min()
    }

    fn coord_into_u32(&self, coord: &Coord) -> u32 {
        coord_into_u32(coord, self.x_max)
    }

    fn u32_into_coord(&self, num: u32) -> Coord {
        let y = num / self.x_max as u32;
        let x = num % self.x_max as u32;
        (x as isize, y as isize)
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let map = input.parse::<Map>()?;
    let part1 = map.find_shortest_path().context("Couldn't get path")?;
    println!("part1: {part1}");
    let part2 = map
        .find_shortest_path_all_a()
        .context("Couldn't get path for part2")?;
    println!("part2: {part2}");
    Ok(())
}

#[test]
fn it_works() {
    let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
    let map = input.parse::<Map>().unwrap();
    let part1 = map.find_shortest_path();
    assert_eq!(part1, Some(31));
}
