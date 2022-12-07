const OFFSET: u8 = b'a';
fn main() {
    let input = include_str!("../input.txt");
    let part1 = find_marker(input, 4).expect("couldn't find 4 char unique sequence");
    println!("part1: {part1}");

    let part2 = find_marker(input, 14).expect("couldn't find 4 char unique sequence");
    println!("part2: {part2}");
}

fn find_marker(s: &str, window_length: usize) -> Option<usize> {
    let c = s.chars().collect::<Vec<_>>();
    c.windows(window_length)
        .enumerate()
        .find_map(|(idx, window)| {
            let mut alphabet = [false; 26];
            for c in window.iter() {
                let letter = (*c as u8 - OFFSET) as usize;
                if alphabet[letter] {
                    return None;
                }
                alphabet[letter] = true;
            }
            Some(idx)
        })
        .map(|idx| idx + window_length)
}

#[test]
fn find_marker_works() {
    let s = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(find_marker(s, 4), Some(5));
}
