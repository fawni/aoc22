use std::ops::RangeInclusive;

fn range_contains(r: &RangeInclusive<u32>, start: u32, end: u32) -> bool {
    r.contains(&start) && r.contains(&end)
}

fn range_overlaps(r: &RangeInclusive<u32>, start: u32, end: u32) -> bool {
    r.contains(&start) || r.contains(&end)
}

fn main() {
    let input = include_str!("../input.txt");
    let (mut contains, mut overlaps) = (0, 0);
    for line in input.lines() {
        let v = line
            .split(',')
            .flat_map(|s| s.split('-'))
            .flat_map(|s| s.parse::<u32>())
            .collect::<Vec<u32>>();
        let r1 = v[0]..=v[1];
        let r2 = v[2]..=v[3];
        if range_contains(&r1, v[2], v[3]) || range_contains(&r2, v[0], v[1]) {
            contains += 1
        }
        if range_overlaps(&r1, v[2], v[3]) || range_overlaps(&r2, v[0], v[1]) {
            overlaps += 1
        }
    }
    println!("{contains}");
    println!("{overlaps}");
}
