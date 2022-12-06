use std::collections::HashSet;

fn find_unique(chars: &Vec<char>, marker: usize) -> usize {
    for idx in 0..chars.len() {
        let mut buffer: Vec<char> = Vec::new();
        for char in chars.iter().skip(idx).take(marker) {
            buffer.push(*char);
        }
        if buffer.into_iter().collect::<HashSet<char>>().len() == marker {
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
