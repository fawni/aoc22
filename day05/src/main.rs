use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let instructions = input.split("\n\n").collect::<Vec<&str>>()[1];

    // it is what it is bro idc
    let mut stacks = HashMap::from([
        (1, vec!['B', 'Q', 'C']),
        (2, vec!['R', 'Q', 'W', 'Z']),
        (3, vec!['B', 'M', 'R', 'L', 'V']),
        (4, vec!['C', 'Z', 'H', 'V', 'T', 'W']),
        (5, vec!['D', 'Z', 'H', 'B', 'N', 'V', 'G']),
        (6, vec!['H', 'N', 'P', 'C', 'J', 'F', 'V', 'Q']),
        (7, vec!['D', 'G', 'T', 'R', 'W', 'Z', 'S']),
        (8, vec!['C', 'G', 'M', 'N', 'B', 'W', 'Z', 'P']),
        (9, vec!['N', 'J', 'B', 'M', 'W', 'Q', 'F', 'P']),
    ]);
    let mut stacks2 = stacks.clone();

    for instruction in instructions.lines().filter(|s| !s.is_empty()) {
        let v = instruction.split_whitespace().collect::<Vec<&str>>();
        let (amount, from, to) = (
            v[1].parse().unwrap(),
            v[3].parse().unwrap(),
            v[5].parse().unwrap(),
        );

        // part 1
        for _ in 0..amount {
            let (mut stack_from, mut stack_to) = (stacks[&from].clone(), stacks[&to].clone());
            stack_to.insert(stack_to.len(), stack_from.pop().unwrap());

            stacks.insert(from, stack_from);
            stacks.insert(to, stack_to);
        }

        // part 2
        let lookahead = stacks2[&from].clone();
        let to_be_moved = lookahead
            .iter()
            .rev()
            .take(amount)
            .rev()
            .collect::<Vec<&char>>();
        for item in to_be_moved {
            let (mut stack_from, mut stack_to) = (stacks2[&from].clone(), stacks2[&to].clone());
            stack_from.pop();
            stack_to.insert(stack_to.len(), *item);

            stacks2.insert(from, stack_from);
            stacks2.insert(to, stack_to);
        }
    }

    let (mut part1, mut part2) = (String::new(), String::new());
    for i in 1..=9 {
        part1.push(*stacks[&i].last().unwrap());
        part2.push(*stacks2[&i].last().unwrap());
    }
    println!("{part1}\n{part2}");
}
