use std::collections::HashMap;

#[derive(Debug)]
struct Directory {
    parent: Option<usize>,
    children: HashMap<&'static str, usize>,
    size: usize,
}

impl Directory {
    fn new(parent: Option<usize>) -> Self {
        Self {
            parent,
            children: HashMap::new(),
            size: 0,
        }
    }

    pub fn total_size(&self, dirs: &Vec<Directory>) -> usize {
        self.size
            + self
                .children
                .values()
                .map(|idx| dirs[*idx].total_size(dirs))
                .sum::<usize>()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut dirs = vec![Directory::new(None)];
    let mut current = 0;

    for line in input.lines() {
        match line {
            "$ cd /" => current = 0,
            "$ cd .." => current = dirs[current].parent.unwrap(),
            "$ ls" => continue,
            _ if line.starts_with("$ cd ") => {
                current = dirs[current].children[line.strip_prefix("$ cd ").unwrap()];
            }
            _ if line.starts_with("dir ") => {
                let dir = dirs.len();
                dirs.push(Directory::new(Some(current)));
                dirs[current]
                    .children
                    .insert(line.strip_prefix("dir ").unwrap(), dir);
            }
            _ => {
                let (size, _) = line.split_once(' ').unwrap();
                dirs[current].size += size.parse::<usize>().unwrap();
            }
        }
    }

    let sum = dirs
        .iter()
        .filter(|d| d.total_size(&dirs) <= 100000)
        .fold(0, |acc, d| acc + d.total_size(&dirs));
    let deleted_size = dirs
        .iter()
        .filter_map(|d| {
            let size = d.total_size(&dirs);
            let needed_space = dirs[0].total_size(&dirs) - 40000000;
            if size >= needed_space {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    println!("{}", sum);
    println!("{}", deleted_size);
}
