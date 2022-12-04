use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Range {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy)]
enum Error {
    ParseRangeError,
    ParsePairError,
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 39-41
        let (start, end) = s.split_once('-').ok_or(Error::ParseRangeError)?;
        let start = start.parse().map_err(|_| Error::ParseRangeError)?;
        let end = end.parse().map_err(|_| Error::ParseRangeError)?;
        Ok(Range { start, end })
    }
}

impl Range {
    fn overlaps(&self, other: &Self) -> bool {
        match self.start.cmp(&other.start) {
            std::cmp::Ordering::Less => {
                !matches!(self.end.cmp(&other.start), std::cmp::Ordering::Less)
            }
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Greater => {
                !matches!(self.start.cmp(&other.end), std::cmp::Ordering::Greater)
            }
        }
    }

    fn within(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }
}

struct Pair {
    first: Range,
    second: Range,
}
impl Pair {
    fn is_contained(&self) -> bool {
        self.first.within(&self.second) || self.second.within(&self.first)
    }

    fn is_overlapped(&self) -> bool {
        self.first.overlaps(&self.second) || self.second.overlaps(&self.first)
    }
}

impl FromStr for Pair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(',').ok_or(Error::ParsePairError)?;
        let first = first.parse()?;
        let second = second.parse()?;

        Ok(Pair { first, second })
    }
}

fn parse(input: &str) -> Result<Vec<Pair>, Error> {
    input.lines().map(|line| line.parse()).collect()
}

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");
    let pairs = parse(input)?;
    let part1 = pairs.iter().filter(|pair| pair.is_contained()).count();
    println!("part1: {part1}");

    let part2 = pairs.iter().filter(|pair| pair.is_overlapped()).count();
    println!("part2: {part2}");
    Ok(())
}
