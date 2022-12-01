fn main() {
    let input = include_str!("../input.txt");
    let elves = input.split("\n\n");
    let mut calories = Vec::<u32>::new();

    for elf in elves {
        let val = elf
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .sum::<u32>();
        calories.push(val);
    }

    calories.sort();
    println!("{}", calories.iter().max().unwrap());
    println!("{}", calories.iter().rev().take(3).sum::<u32>());
}
