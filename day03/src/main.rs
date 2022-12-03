use itertools::Itertools;

fn calculate_priority(c: char) -> u32 {
    match c.is_ascii_lowercase() {
        true => c as u32 - 96,
        _ => c as u32 - 38,
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let chunks = input.lines().chunks(3);

    let part1 = input
        .lines()
        .map(|l| {
            let (h1, h2) = l.split_at(l.len() / 2);
            h1.chars().find(|s| h2.contains(*s)).unwrap()
        })
        .fold(0, |acc, c| acc + calculate_priority(c));

    let part2 = chunks
        .into_iter()
        .filter_map(|chunk| {
            let v = chunk.collect::<Vec<&str>>();
            v[0].chars()
                .filter(|c| v[1].contains(*c))
                .find(|c| v[2].contains(*c))
        })
        .fold(0, |acc, c| acc + calculate_priority(c));

    println!("{part1}");
    println!("{part2}");
}
