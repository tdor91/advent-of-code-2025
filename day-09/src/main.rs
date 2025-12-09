use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/input.txt");

    let coords = input.lines().filter_map(parse).collect::<Vec<_>>();

    println!("part 1: {}", find_largest(&coords));
}

fn parse(line: &str) -> Option<(usize, usize)> {
    if let Some((x, y)) = line.split_once(',') {
        return Some((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
    }

    None
}

fn find_largest(coords: &[(usize, usize)]) -> usize {
    let mut sizes = HashMap::new();

    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let dx = coords[i].0.abs_diff(coords[j].0) + 1;
            let dy = coords[i].1.abs_diff(coords[j].1) + 1;

            sizes.insert((i, j), dx * dy);
        }
    }

    let max = sizes.iter().max_by_key(|&(_, size)| size).unwrap();

    max.1.clone()
}
