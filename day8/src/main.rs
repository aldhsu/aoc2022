use std::{collections::HashSet, str::FromStr};

use anyhow::{Context, Error, Result};

struct Map {
    inner: Vec<Vec<u8>>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner: Result<Vec<Vec<u8>>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Ok(c.to_digit(10).context("couldn't convert to digit")? as u8))
                    .collect::<Result<Vec<_>>>()
            })
            .collect();

        Ok(Self { inner: inner? })
    }
}

fn visible_in_row<'a>(
    (direction, position): (char, usize),
    mut items: impl Iterator<Item = (usize, &'a u8)>,
) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let (idx, mut tallest_seen) = items.next().expect("should have an item");

    fn into_coord(direction: char, idx: usize, position: usize) -> (usize, usize) {
        match direction {
            'x' => (position, idx),
            'y' => (idx, position),
            _ => panic!("unhandled direction"),
        }
    }

    result.push(into_coord(direction, idx, position));

    for (idx, item) in items {
        if tallest_seen < item {
            result.push(into_coord(direction, idx, position));
            tallest_seen = item
        }
    }

    result
}

impl Map {
    fn visible_trees(&self) -> Result<usize> {
        let y_len = self.inner.len();
        let x_len = self
            .inner
            .first()
            .map(|first| first.len())
            .context("couldn't get x len")?;

        let mut seen = HashSet::new();

        for y in 0..y_len {
            let row = &self.inner[y];
            //left forwards
            let forwards = visible_in_row(('y', y), row.iter().enumerate());
            seen.extend(forwards);
            // right backwards
            let backwards = visible_in_row(('y', y), row.iter().enumerate().rev());
            seen.extend(backwards);
        }

        for x in 0..x_len {
            let row = &self.inner.iter().map(|row| &row[x]).enumerate();
            //left forwards
            let forwards = visible_in_row(('x', x), row.clone());
            seen.extend(forwards);
            //right backwards
            let backwards = visible_in_row(('x', x), row.clone().rev());
            seen.extend(backwards);
        }

        Ok(seen.len())
    }

    fn most_scenic(&self) -> Result<usize> {
        let y_len = self.inner.len();
        let x_len = self
            .inner
            .first()
            .map(|first| first.len())
            .context("couldn't get x len")?;

        let mut max = 0;

        for x in 0..x_len {
            for y in 0..y_len {
                let result = calculate_score((x, y), (x_len, y_len), &self.inner);
                if result > max {
                    max = result;
                }
            }
        }

        Ok(max)
    }
}

fn calculate_score((x, y): (usize, usize), (_, y_len): (usize, usize), map: &[Vec<u8>]) -> usize {
    let tree_height = map[y][x];
    [
        visible_from_tree(tree_height, map[y][x..].iter().skip(1)), // right
        visible_from_tree(tree_height, map[y][..x].iter().rev()), //left
        visible_from_tree(
            tree_height,
            map.iter().rev().skip(y_len - y).map(|line| &line[x]),
        ), // up
        visible_from_tree(tree_height, map.iter().skip(y + 1).map(|line| &line[x])), // down
    ]
    .iter()
    .product()
}

fn visible_from_tree<'a>(tree_height: u8, items: impl Iterator<Item = &'a u8>) -> usize {
    let mut result = 0;

    for item in items {
        result += 1;
        if tree_height <= *item {
            break;
        }
    }

    result
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let map: Map = input.parse()?;
    let part1 = map.visible_trees()?;
    println!("part1: {part1}");
    let part2 = map.most_scenic()?;
    println!("part2: {part2}");
    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let input = r#"30373
25512
65332
33549
35390"#;
    let map: Map = input.parse()?;
    let part1 = map.visible_trees()?;
    assert_eq!(part1, 21);
    Ok(())
}

#[test]
fn calculate_score_works() -> Result<()> {
    let input = r#"30373
25512
65332
33549
35390"#;
    let map: Map = input.parse()?;
    assert_eq!(calculate_score((2, 1), (5, 5), &map.inner), 4);
    assert_eq!(calculate_score((2, 3), (5, 5), &map.inner), 8);
    Ok(())
}

#[test]
fn part2_works() -> Result<()> {
    let input = r#"30373
25512
65332
33549
35390"#;
    let map: Map = input.parse()?;
    assert_eq!(map.most_scenic().unwrap(), 8);
    Ok(())
}
