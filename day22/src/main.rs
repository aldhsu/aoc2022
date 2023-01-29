mod parser;
mod types;
use types::*;
use parser::parse_instructions;

use std::collections::HashMap;
use anyhow::{Result, Context};


fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let part1 = part1(input)?;
    println!("part1: {part1}");

    let part2 = part2(input)?;
    println!("part2: {part2}");

    Ok(())
}

fn part1(s: &str) -> Result<usize> {
    let (map, instructions) = s.split_once("\n\n").context("couldn't get parts")?;
    let map = map.parse::<Map>()?;
    let (_, instructions) = parse_instructions(instructions).unwrap();

    let mut cursor = Cursor {
        position: map.starting_point,
        facing: Facing::Right,
        history: vec![(map.starting_point, Facing::Right)],
    };

    for ins in &instructions {
        cursor.apply_instruction(ins, &map);
    }

    let history = cursor
        .history
        .iter()
        .cloned()
        .collect::<HashMap<Coord, Facing>>();

    Ok(cursor.score())
}

fn part2(s: &str) -> Result<usize> {
    let (map, instructions) = s.split_once("\n\n").context("couldn't get parts")?;
    let map = map.parse::<Cube>()?;
    let (_, instructions) = parse_instructions(instructions).unwrap();

    let mut cursor = CubeCursor {
        coord: map.starting_point,
        facing: Facing::Right,
        history: vec![(map.starting_point, Facing::Right)],
    };

    for ins in &instructions {
        cursor.apply_instruction(ins, &map);
    }

    Ok(cursor.score())
}


// debugging code to print map
    // for (y, line) in map.inner.iter().enumerate() {
    //     for (x, tile) in line.iter().enumerate() {
    //         match tile {
    //             Tile::Open => {
    //                 if let Some(facing) = history.get(&(x, y)) {
    //                     match facing {
    //                         Facing::Up => print!("^"),
    //                         Facing::Right => print!(">"),
    //                         Facing::Left => print!("<"),
    //                         Facing::Down => print!("v"),
    //                     }
    //                 } else {
    //                     print!(".");
    //                 };
    //             }
    //             Tile::Wall => print!("#"),
    //             Tile::None => print!(" "),
    //         }
    //     }
    //     println!()
    // }
