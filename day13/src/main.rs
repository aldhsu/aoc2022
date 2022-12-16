use std::cmp::Ordering;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

#[derive(Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Num(u8),
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(list) => {
                // f.debug_tuple("List").field(arg0).finish()
                f.write_fmt(format_args!("{:?}", list))
            },
            Self::Num(num) => {
                f.write_fmt(format_args!("{}", num))
            },
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("unhandled ordering")
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => {
                for i in 0..a.len() {
                    let a = &a[i];
                    let Some(b) = b.get(i) else { return Some(Ordering::Greater) };
                    let result = a.partial_cmp(b);
                    if !matches!(result, Some(Ordering::Equal)) {
                        return result;
                    }
                }

                Some(if b.len() > a.len() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                })
            }
            (a @ Packet::List(_), Packet::Num(b)) => {
                a.partial_cmp(&Packet::List(vec![Packet::Num(*b)]))
            }
            (Packet::Num(a), b @ Packet::List(_)) => {
                Packet::List(vec![Packet::Num(*a)]).partial_cmp(b)
            }
            (Packet::Num(a), Packet::Num(b)) => Some(a.cmp(b)),
        }
    }
}

fn parse_list(s: &str) -> IResult<&str, Packet> {
    let (s, packets) = delimited(
        char('['),
        separated_list0(char(','), parse_packet),
        char(']'),
    )(s)?;
    Ok((s, Packet::List(packets)))
}

fn parse_num(s: &str) -> IResult<&str, Packet> {
    let (s, packet) = digit1(s)?;
    Ok((
        s,
        Packet::Num(packet.parse::<u8>().expect("Unable to parse num")),
    ))
}

fn parse_packet(s: &str) -> IResult<&str, Packet> {
    let (s, packet) = alt((parse_list, parse_num))(s)?;

    Ok((s, packet))
}

fn parse_packet_pair(s: &str) -> IResult<&str, (Packet, Packet)> {
    let (s, result) = separated_pair(parse_packet, newline, parse_packet)(s)?;

    Ok((s, result))
}

fn parse_input(s: &str) -> Vec<(Packet, Packet)> {
    let (_, items) =
        separated_list1(tag("\n\n"), parse_packet_pair)(s).expect("couldn't get items");

    items
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn part1(input: &str) -> usize {
    let items = parse_input(input);

    let indexes = items
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| {
            let not_greater = !matches!(left.partial_cmp(right), Some(Ordering::Greater));
            not_greater.then_some(i + 1)
        })
        .collect::<Vec<_>>();

    indexes.iter().sum()
}

fn part2(input: &str) -> usize {
    let items = parse_input(input);

    let mut packets = items
        .into_iter()
        .flat_map(|(a, b)| [a, b].into_iter())
        .chain(
            [
                Packet::List(vec![Packet::List(vec![Packet::Num(2)])]),
                Packet::List(vec![Packet::List(vec![Packet::Num(6)])]),
            ]
            .into_iter(),
        )
        .collect::<Vec<_>>();

    packets.sort();
    let two_packet = packets
        .iter()
        .position(|packet| packet == &Packet::List(vec![Packet::List(vec![Packet::Num(2)])]))
        .expect("couldn't find two");
    let six_packet = packets
        .iter()
        .position(|packet| packet == &Packet::List(vec![Packet::List(vec![Packet::Num(6)])]))
        .expect("coudln't find six");
    (two_packet + 1) * (six_packet + 1)
}

#[test]
fn it_works() {
    let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 140);
}

#[test]
fn cmp_works() {
    use Packet::*;

    let a = List(vec![List(vec![Num(1)]), List(vec![Num(2), Num(3), Num(4)])]);
    let b = List(vec![List(vec![Num(1)]), Num(4)]);
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Less))
}

#[test]
fn cmp_works2() {
    use Packet::*;
    let a = List(vec![Num(9)]);
    let b = List(vec![List(vec![Num(8), Num(7), Num(6)])]);
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater))
}
