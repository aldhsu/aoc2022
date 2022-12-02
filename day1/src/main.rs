struct Elf {
    foods: Vec<u32>,
}

impl Elf {
    fn total_calories(&self) -> u32 {
        self.foods.iter().sum()
    }
}

fn parse(s: &str) -> Vec<Elf> {
    s.split("\n\n").map(|elf| {
        let foods = elf.lines().map(|food| {
            food.parse::<u32>().unwrap()
        }).collect();
        Elf {
            foods
        }
    }).collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let mut elves = parse(input);
    let max_elf = elves.iter().max_by(|a, b| a.total_calories().cmp(&b.total_calories())).unwrap();
    println!("max_elf: {}", max_elf.total_calories());

    elves.sort_by_key(|a| a.total_calories());
    let three_total = elves.iter().rev().take(3).fold(0, |memo, elf| memo + elf.total_calories());
    println!("three_elves: {three_total}");
}
