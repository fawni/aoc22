use std::collections::HashSet;

fn find_unique(chars: &Vec<char>, marker: usize) -> usize {
    for idx in 0..chars.len() {
        let buffer = chars
            .iter()
            .skip(idx)
            .take(marker)
            .collect::<HashSet<&char>>();
        if buffer.len() == marker {
            return idx + marker;
        }
    }
    unreachable!();
}

fn main() {
    let chars = include_str!("../input.txt")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();
    println!("{}", find_unique(&chars, 4));
    println!("{}", find_unique(&chars, 14));
}
