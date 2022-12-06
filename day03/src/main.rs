fn calculate_priority(c: char) -> u32 {
    match c.is_ascii_lowercase() {
        true => 1 + c as u32 - 'a' as u32,
        _ => 27 + c as u32 - 'A' as u32,
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let part1 = input
        .lines()
        .map(|l| {
            let (h1, h2) = l.split_at(l.len() / 2);
            h1.chars().find(|s| h2.contains(*s)).unwrap()
        })
        .fold(0, |acc, c| acc + calculate_priority(c));

    let part2 = input
        .lines()
        .enumerate()
        .step_by(3)
        .filter_map(|(idx, _)| {
            lines[idx]
                .chars()
                .filter(|c| lines[idx + 1].contains(*c))
                .find(|c| lines[idx + 2].contains(*c))
        })
        .fold(0, |acc, c| acc + calculate_priority(c));

    println!("{part1}");
    println!("{part2}");
}
