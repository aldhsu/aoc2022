use std::str::FromStr;

use anyhow::{Error, Result};

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let part1 = part1(input)?;
    println!("part1: {part1}");
    Ok(())
}

#[test]
fn part1_works() {
    let input = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;
    assert_eq!(part1(input).unwrap(), "2=-1=0");
}
fn part1(s: &str) -> Result<String> {
    let sum = s
        .lines()
        .map(|line| line.parse::<Snafu>().unwrap())
        .sum::<isize>();
    Ok(Snafu::from(sum).source)
}

#[derive(Debug)]
struct Snafu {
    source: String,
    num: isize,
}

#[test]
fn snafu_parses() {
    assert_eq!("1".parse::<Snafu>().unwrap().num, 1);
    assert_eq!("2".parse::<Snafu>().unwrap().num, 2);
    assert_eq!("1=".parse::<Snafu>().unwrap().num, 3);
    assert_eq!("1-".parse::<Snafu>().unwrap().num, 4);
    assert_eq!("10".parse::<Snafu>().unwrap().num, 5);
    assert_eq!("11".parse::<Snafu>().unwrap().num, 6);
    assert_eq!("12".parse::<Snafu>().unwrap().num, 7);
    assert_eq!("2=".parse::<Snafu>().unwrap().num, 8);
    assert_eq!("2-".parse::<Snafu>().unwrap().num, 9);
    assert_eq!("20".parse::<Snafu>().unwrap().num, 10);
    assert_eq!("1=0".parse::<Snafu>().unwrap().num, 15);
    assert_eq!("1-0".parse::<Snafu>().unwrap().num, 20);
    assert_eq!("1=11-2".parse::<Snafu>().unwrap().num, 2022);
    assert_eq!("1-0---0".parse::<Snafu>().unwrap().num, 12345);
    assert_eq!("1121-1110-1=0".parse::<Snafu>().unwrap().num, 314159265);
}

impl FromStr for Snafu {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let place = 5isize.pow(i as u32);
                match c {
                    '1' => place,
                    '2' => place * 2,
                    '0' => 0,
                    '-' => -place,
                    '=' => -place * 2,
                    _ => unreachable!(),
                }
            })
            .sum();
        Ok(Self {
            source: s.into(),
            num,
        })
    }
}

#[test]
fn can_translate_back() {
    // assert_eq!(Snafu::from(1isize).source, "1");
    // assert_eq!(Snafu::from(2isize).source, "2");
    // assert_eq!(Snafu::from(3isize).source, "1=");
    // assert_eq!(Snafu::from(4isize).source, "1-");
    // assert_eq!(Snafu::from(5isize).source, "10");
    // assert_eq!(Snafu::from(6isize).source, "11");
    // assert_eq!(Snafu::from(7isize).source, "12");
    // assert_eq!(Snafu::from(8isize).source, "2=");
    // assert_eq!(Snafu::from(9isize).source, "2-");
    // assert_eq!(Snafu::from(10isize).source, "20");
    // assert_eq!(Snafu::from(15isize).source, "1=0");
    // assert_eq!(Snafu::from(20isize).source, "1-0");
    // assert_eq!(Snafu::from(2022isize).source, "1=11-2");
    // assert_eq!(Snafu::from(12345isize).source, "1-0---0");
    // assert_eq!(Snafu::from(314159265isize).source, "1121-1110-1=0");
    assert_eq!(
        Snafu::from(30638862852576isize).source,
        "2=01-0-2-0=-0==-1=01"
    );
}

impl From<isize> for Snafu {
    fn from(mut value: isize) -> Self {
        let mut source: Vec<i8> = vec![0; (value.ilog10() * 2 + 2) as usize];

        fn add_sources(vec: &mut Vec<i8>, other: impl Iterator<Item = i8> + Clone) {
            let mut carry = 0;

            let last_place = other.size_hint().1.unwrap();
            for (source, adder) in vec.iter_mut().zip(other) {
                let mut new = *source + adder + carry;
                carry = 0;

                if new > 2 {
                    new -= 5;
                    carry = 1;
                }
                *source = new;
            }

            if carry == 1 {
                add_sources(
                    vec,
                    std::iter::repeat(0)
                        .take(last_place)
                        .chain(std::iter::once(1)),
                )
            }
        }

        fn convert(source: &[i8]) -> String {
            source
                .iter()
                .rev()
                .skip_while(|x| *x == &0)
                .map(|item| match item {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => panic!("{item}"),
                })
                .collect::<String>()
        }

        let mut places: u32 = (value.ilog10() as f32 * 1.4 + 1f32) as u32;
        while value != 0 {
            let pow = 5usize.pow(places);
            let pow_down = 5usize.pow(places.saturating_sub(1));

            let val = [
                pow * 2,
                pow,
                pow.saturating_sub(pow_down),
                pow.saturating_sub(pow_down * 2),
            ]
            .into_iter()
            .enumerate()
            .find_map(|(i, candidate)| {
                (candidate as isize <= value).then_some((i, candidate as isize))
            });

            if let Some((i, val)) = val {
                value -= val;
                let places = places as usize;
                match i {
                    0 => add_sources(
                        &mut source,
                        std::iter::repeat(0).take(places).chain(std::iter::once(2)),
                    ),
                    1 => add_sources(
                        &mut source,
                        std::iter::repeat(0).take(places).chain(std::iter::once(1)),
                    ),
                    2 => add_sources(
                        &mut source,
                        std::iter::repeat(0)
                            .take(places.saturating_sub(1))
                            .chain([-1, 1].into_iter()),
                    ),
                    3 => add_sources(
                        &mut source,
                        std::iter::repeat(0)
                            .take(places.saturating_sub(1))
                            .chain([-2, 1].into_iter()),
                    ),
                    _ => unreachable!(),
                }
            } else {
                places -= 1;
            }
        }

        let source = convert(&source);

        Snafu { source, num: value }
    }
}

impl std::iter::Sum<Snafu> for isize {
    fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Self {
        iter.map(|s| s.num).sum()
    }
}
