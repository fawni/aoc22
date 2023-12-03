struct Knot {
    hx: i32,
    hy: i32,
    tx: i32,
    ty: i32,
}

impl Knot {
    fn move_tail(mut self) -> Self {
        // self.tx = self.tx + 1;
        self
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let moves = input
        .lines()
        .map(|l| {
            let (d, a) = l.split_once(' ').unwrap();
            let amount = a.parse::<u32>().unwrap();
            match d {
                "L" => (-1, 0, amount),
                "R" => (1, 0, amount),
                "D" => (0, -1, amount),
                _ => (0, 1, amount),
            }
        })
        .collect::<Vec<(i32, i32, u32)>>();
}
