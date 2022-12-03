#![feature(array_chunks)]
use std::collections::HashSet;
use std::str::FromStr;

struct Rucksack {
    first_comp: HashSet<char>,
    second_comp: HashSet<char>,
    all: HashSet<char>,
}

impl Rucksack {
    fn duplicates(&self) -> impl Iterator<Item = &char> {
        self.first_comp.intersection(&self.second_comp)
    }
}

#[derive(Debug)]
enum Error {
    ParseRucksackError,
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let (one, two) = s.split_at(len / 2);

        Ok(Rucksack {
            first_comp: one.chars().collect(),
            second_comp: two.chars().collect(),
            all: s.chars().collect(),
        })
    }
}

fn parse(s: &str) -> Vec<Rucksack> {
    s.lines().map(|line| line.parse().unwrap()).collect()
}

const LOWERCASE_ALPHABET_START: u8 = b'a';
const UPPERCASE_ALPHABET_START: u8 = b'A';

fn priority(c: &char) -> u8 {
    let reset = match c {
        'a'..='z' => LOWERCASE_ALPHABET_START,
        'A'..='Z' => UPPERCASE_ALPHABET_START - 26,
        _ => unreachable!(),
    };
    *c as u8 - reset + 1
}

fn main() {
    let input = include_str!("../input.txt");
    let rucksacks = parse(input);
    let part1: u32 = rucksacks
        .iter()
        .map(|sack| {
            sack.duplicates()
                .fold(0, |memo, dupes| memo + priority(dupes) as u32)
        })
        .sum();
    println!("part1: {part1}");

    let part2: u32 = rucksacks
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            a.all
                .intersection(&b.all)
                .cloned()
                .collect::<HashSet<char>>()
                .intersection(&c.all)
                .next()
                .map(priority)
                .expect("couldn't find common") as u32
        })
        .sum();
    println!("part2: {part2}");
}

#[test]
fn chars_are_convertible() {
    assert_eq!(priority(&'a'), 1);
    assert_eq!(priority(&'A'), 27);
}
