fn answer(stacks: Vec<Vec<char>>) -> String {
    stacks.iter().map(|v| v.last().unwrap()).collect::<String>()
}

fn main() {
    let input = include_str!("../input.txt");
    let (boxes, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in boxes.lines().rev().skip(1) {
        for (i, c) in line
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| !c.is_whitespace())
        {
            if i >= stacks.len() {
                stacks.push(Vec::new());
            }
            stacks[i].push(c);
        }
    }
    let mut stacks2 = stacks.clone();

    for instruction in instructions.lines().filter(|s| !s.is_empty()) {
        let v = instruction.split_whitespace().collect::<Vec<&str>>();
        let (amount, from, to) = (
            v[1].parse().unwrap(),
            v[3].parse::<usize>().unwrap() - 1,
            v[5].parse::<usize>().unwrap() - 1,
        );

        for _ in 0..amount {
            let item = stacks[from].pop().unwrap();
            stacks[to].push(item);
        }

        let len = stacks2[from].len();
        let items = stacks2[from].drain(len - amount..).collect::<Vec<char>>();
        for item in items {
            stacks2[to].push(item);
        }
    }

    println!("{}", answer(stacks));
    println!("{}", answer(stacks2));
}
