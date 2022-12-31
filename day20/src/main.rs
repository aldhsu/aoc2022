use anyhow::{Context, Result};
use std::collections::VecDeque;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");

    let part1 = part1(input)?;
    println!("part1: {part1}");

    let part2 = part2(input)?;
    println!("part2: {part2}");

    Ok(())
}

fn part1(s: &str) -> Result<i64> {
    let nums = s
        .lines()
        .map(|num| num.parse::<i64>().context("couldn't get num"))
        .collect::<Result<Vec<_>>>()?;
    let mut indices: VecDeque<usize> = (0..nums.len()).collect();

    for (old_index, val) in nums.iter().enumerate() {
        let current_index = indices
            .iter()
            .position(|other| other == &old_index)
            .context("couldn't find index")?;

        indices.remove(current_index);
        let new_index = ((current_index as i64 + val).rem_euclid(indices.len() as i64)) as usize;
        indices.insert(new_index, old_index);
    }

    let original_zero = nums
        .iter()
        .position(|num| num == &0)
        .context("couldn't find zero")?;
    let zero_pos = indices
        .iter()
        .position(|idx| idx == &original_zero)
        .context("couldn't find new zero pos")?;

    Ok([1000, 2000, 3000]
        .iter()
        .map(|thousand| {
            let original_index = indices[(thousand + zero_pos) % indices.len()];
            nums[original_index]
        })
        .sum())
}

const KEY: i64 = 811589153;

fn part2(s: &str) -> Result<i64> {
    let nums = s
        .lines()
        .map(|num| {
            num.parse::<i64>()
                .context("couldn't get num")
                .map(|i| i * KEY)
        })
        .collect::<Result<Vec<_>>>()?;

    let mut indices: VecDeque<usize> = (0..nums.len()).collect();

    for _ in 0..10 {
        for (old_index, val) in nums.iter().enumerate() {
            let current_index = indices
                .iter()
                .position(|other| other == &old_index)
                .context("couldn't find index")?;

            indices.remove(current_index);
            let new_index =
                ((current_index as i64 + val).rem_euclid(indices.len() as i64)) as usize;
            indices.insert(new_index, old_index);
        }
    }

    let original_zero = nums
        .iter()
        .position(|num| num == &0)
        .context("couldn't find zero")?;
    let zero_pos = indices
        .iter()
        .position(|idx| idx == &original_zero)
        .context("couldn't find new zero pos")?;

    Ok([1000, 2000, 3000]
        .iter()
        .map(|thousand| {
            let original_index = indices[(thousand + zero_pos) % indices.len()];
            nums[original_index]
        })
        .sum())
}

#[test]
fn part1_works() {
    let input = r#"1
2
-3
3
-2
0
4"#;
    assert_eq!(part1(input).unwrap(), 3)
}

#[test]
fn part2_works() {
    let input = r#"1
2
-3
3
-2
0
4"#;
    assert_eq!(part2(input).unwrap(), 1623178306)
}
