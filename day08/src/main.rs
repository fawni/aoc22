#[derive(Debug)]
struct Tree {
    height: u32,
    x: usize,
    y: usize,
}

impl Tree {
    fn new(height: u32, x: usize, y: usize) -> Self {
        Self { height, x, y }
    }

    fn is_visible(&self, chars: &[u32], size: usize) -> bool {
        let by_x = |x: usize| chars[x + (1 + size) * self.y] < self.height;
        let by_y = |y: usize| chars[self.x + (1 + size) * y] < self.height;

        (0..self.x).all(by_x)
            || (self.x + 1..=size).all(by_x)
            || (0..self.y).all(by_y)
            || (self.y + 1..=size).all(by_y)
    }

    fn scenic_score(&self, chars: &[u32], size: usize) -> usize {
        let by_x = |&x: &usize| chars[x + (1 + size) * self.y] >= self.height;
        let by_y = |&y: &usize| chars[self.x + (1 + size) * y] >= self.height;

        (self.x - (0..self.x).rev().find(by_x).unwrap_or(0))
            * ((self.x + 1..=size).find(by_x).unwrap_or(size) - self.x)
            * (self.y - (0..self.y).rev().find(by_y).unwrap_or(0))
            * ((self.y + 1..=size).find(by_y).unwrap_or(size) - self.y)
    }
}

fn parse_trees(input: &str) -> Vec<Tree> {
    let mut trees: Vec<Tree> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let chars = line
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_string().parse::<u32>().unwrap());

        for (x, height) in chars.enumerate() {
            trees.push(Tree::new(height, x, y));
        }
    }

    trees
}

fn main() {
    let input = include_str!("../input.txt");
    let size = input.lines().count() - 1;
    let chars = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let trees = parse_trees(input);

    let sum = trees.iter().filter(|t| t.is_visible(&chars, size)).count();
    let scenic = trees
        .iter()
        .map(|t| t.scenic_score(&chars, size))
        .max()
        .unwrap();

    println!("{sum}");
    println!("{scenic}");
}
