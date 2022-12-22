use std::{collections::BTreeSet, fmt::Display};

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input.trim());
    println!("part1: {part1}");
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

#[derive(Default)]
struct Map {
    board: Board,
    jets: Vec<Jet>,
    cursor: usize,
    piece_cursor: usize,
    highest: usize,
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
        let mut piece: Vec<Coord> =
            Self::PIECE_ORDER[self.piece_cursor].appear_at((2, self.highest + 3));
        loop {
            let offset: Offset = (&self.jets[self.cursor]).into();
            // dbg!((&self.jets[self.cursor]), self.cursor);
            self.cursor = (self.cursor + 1) % self.jets.len();

            if let Some(new) = move_horizontal(&piece, offset, &self.board) {
                // dbg!("moved", piece);
                piece = new;
            };

            if let Some(new) = move_down(&piece, &self.board) {
                piece = new;
            } else {
                break;
            }
        }

        self.piece_cursor = (self.piece_cursor + 1) % Self::PIECE_ORDER.len();
        self.highest = piece
            .iter()
            .map(|(_, y)| y + 1)
            .max()
            .unwrap_or(0)
            .max(self.highest);
        self.board.extend(piece);
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

    for _ in 0..2022 {
        map.drop_rock();
    }

    map.highest
}

#[test]
fn test_part1() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part1(input), 3068)
}
