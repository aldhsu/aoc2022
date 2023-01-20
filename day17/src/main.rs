#![feature(map_try_insert)]
#![feature(let_chains)]

use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
};

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input.trim());
    println!("part1: {part1}");

    let part2 = part2(input.trim());
    println!("part2: {part2}");
}

#[derive(Debug, Copy, Clone)]
enum Piece {
    Long,
    Plus,
    El,
    Eye,
    Box,
}

impl Piece {
    fn appear_at(&self, (x, y): Coord) -> Vec<Coord> {
        let mut coords = match self {
            Piece::Long => {
                vec![(0, 0), (1, 0), (2, 0), (3, 0)]
            }
            Piece::Plus => {
                vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)]
            }
            Piece::El => {
                vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)]
            }
            Piece::Eye => {
                vec![(0, 3), (0, 2), (0, 1), (0, 0)]
            }
            Piece::Box => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        };

        for coord in coords.iter_mut() {
            coord.0 += x;
            coord.1 += y;
        }

        coords
    }
}

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

impl From<char> for Jet {
    fn from(value: char) -> Self {
        match value {
            '>' => Jet::Right,
            '<' => Jet::Left,
            _ => unreachable!(),
        }
    }
}

impl From<&Jet> for Offset {
    fn from(value: &Jet) -> Self {
        match value {
            Jet::Left => (-1, 0),
            Jet::Right => (1, 0),
        }
    }
}

type Coord = (usize, usize);

#[derive(Debug, Hash, Eq, PartialEq, Default)]
struct Identifier {
    piece: usize,
    jet: usize,
    height_change: usize,
    lines: u64,
}

#[derive(Default)]
struct Map {
    board: Board,
    jets: Vec<Jet>,
    cursor: usize,
    piece_cursor: usize,
    highest: usize,
    cache: HashMap<Identifier, (usize, usize)>,
    move_num: usize,
    found_cycle: Option<((usize, usize), (usize, usize))>,
}

type Board = BTreeSet<Coord>;
type Offset = (isize, isize);

fn update_coord((x, y): Coord, (o_x, o_y): Offset) -> Option<Coord> {
    let x = x.checked_add_signed(o_x)?;
    if x > 6 {
        return None;
    }
    Some((x, y.checked_add_signed(o_y)?))
}

fn update_coords(coords: &[Coord], offset: Offset) -> Option<Vec<Coord>> {
    coords
        .iter()
        .map(|coord| update_coord(*coord, offset))
        .collect()
}

fn move_horizontal(coords: &[Coord], offset: Offset, board: &Board) -> Option<Vec<Coord>> {
    let new_coords = update_coords(coords, offset)?;
    if new_coords.iter().any(|coord| board.contains(coord)) {
        return None;
    }
    Some(new_coords)
}

fn move_down(coords: &[Coord], board: &Board) -> Option<Vec<Coord>> {
    let new_coords = update_coords(coords, (0, -1))?;
    if new_coords.iter().any(|coord| board.contains(coord)) {
        return None;
    }
    Some(new_coords)
}

impl Map {
    const PIECE_ORDER: [Piece; 5] = [Piece::Long, Piece::Plus, Piece::El, Piece::Eye, Piece::Box];

    fn new(s: &str) -> Map {
        Self {
            jets: s.chars().map(Jet::from).collect(),
            ..Default::default()
        }
    }

    fn drop_rock(&mut self) {
        self.move_num += 1;

        let mut piece: Vec<Coord> =
            Self::PIECE_ORDER[self.piece_cursor].appear_at((2, self.highest + 3));
        loop {
            let offset: Offset = (&self.jets[self.cursor]).into();
            self.cursor = (self.cursor + 1) % self.jets.len();

            if let Some(new) = move_horizontal(&piece, offset, &self.board) {
                piece = new;
            };

            if let Some(new) = move_down(&piece, &self.board) {
                piece = new;
            } else {
                break;
            }
        }

        let old_highest = self.highest;
        self.highest = piece
            .iter()
            .map(|(_, y)| y + 1)
            .max()
            .unwrap_or(0)
            .max(self.highest);
        self.board.extend(piece);

        let id = Identifier {
            piece: self.piece_cursor,
            jet: self.cursor,
            lines: self.get_last_lines(),
            height_change: self.highest - old_highest,
        };

        if let Some(old) =
            self.cache.get(&id) && self.found_cycle.is_none()
        {
            self.found_cycle = Some(((self.move_num, self.highest), *old));
        } else {
            self.cache.insert(id, (self.move_num, self.highest));
        }

        self.piece_cursor = (self.piece_cursor + 1) % Self::PIECE_ORDER.len();
    }

    fn get_last_lines(&self) -> u64 {
        let mut line = 0u64;
        for y in (self.highest.saturating_sub(8))..=self.highest {
            for x in 0..7 {
                line <<= 1;
                if self.board.contains(&(x, y)) {
                    line += 1;
                }
            }
        }
        line
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..=self.highest).rev() {
            for x in 0..7 {
                if self.board.contains(&(x, y)) {
                    write!(f, "@")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        write!(f, "finished")
    }
}

fn part1(s: &str) -> usize {
    let mut map = Map::new(s);

    for i in 0..2022 {
        map.drop_rock();
        if let Some(((end_move, end_height), (start_move, start_height))) = map.found_cycle {
            if i % 100 == 0 {
                let cycle_height = end_height - start_height;
                let cycle_length = end_move - start_move;
                let cycle_count = (map.move_num - start_move) / cycle_length;
                let partial_cycle = (map.move_num - start_move) % cycle_length;
                let mut nmap = Map::new(s);
                for _ in 0..partial_cycle + start_move {
                    nmap.drop_rock();
                    if nmap.move_num == start_move + 2 {
                    }
                }
                let offset = dbg!(nmap.highest - start_height);
                dbg!(map.highest - (cycle_count * cycle_height + start_height + offset));
                println!();
            }
        }
    }

    map.highest
}

fn part2(s: &str) -> usize {
    let mut map = Map::new(s);

    while map.found_cycle.is_none() {
        map.drop_rock();
    }

    let Some(((end_move, end_height), (start_move, start_height))) = map.found_cycle else { panic!("should have something")};
    let cycle_length = end_move - start_move;
    let height_change = end_height - start_height;

    let mut height_total = 0;
    let mut move_goal = 1_000_000_000_000;
    move_goal -= end_move;
    height_total += end_height;
    let full_cycles = move_goal / (cycle_length);
    height_total += full_cycles * height_change;

    let partial_cycle = move_goal % cycle_length;
    let mut map = Map::new(s);
    for _ in 0..partial_cycle + start_move {
        map.drop_rock()
    }
    let partial_height = map.highest - start_height;

    height_total + partial_height
}

#[test]
fn test_part1() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part1(input), 3068)
}

#[test]
fn test_part2() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part2(input), 1514285714288)
}
