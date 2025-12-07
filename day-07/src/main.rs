use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input/input.txt");
    let lines = input.lines().collect::<Vec<_>>();

    let mut splitter = vec![];
    let mut start_position = (0, 0);

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'S' => start_position = (col, row),
                '^' => splitter.push((col, row)),
                _ => {}
            }
        }
    }

    println!(
        "part 1: {}",
        count_splits(&start_position, lines.len(), &splitter)
    );

    println!(
        "part 2: {}",
        count_worlds(&start_position, lines.len(), &splitter, &mut HashMap::new())
    );
}

fn count_splits(start: &(usize, usize), max: usize, splitter: &Vec<(usize, usize)>) -> usize {
    let mut pool = HashSet::new();
    pool.insert(*start);

    let mut current_level = 0;
    let mut visited_splitters = 0;

    while current_level < max {
        let mut next = HashSet::new();

        for c in pool {
            let n = (c.0, c.1 + 1);

            if splitter.iter().any(|x| *x == n) {
                next.insert((n.0 - 1, n.1));
                next.insert((n.0 + 1, n.1));
                visited_splitters += 1;
            } else {
                next.insert(n);
            }
        }

        pool = next;
        current_level += 1;
    }

    visited_splitters
}

fn count_worlds(
    start: &(usize, usize),
    max: usize,
    splitter: &[(usize, usize)],
    state: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(worlds) = state.get(start) {
        return *worlds;
    }

    let next = (start.0, start.1 + 1);

    if next.1 >= max {
        return 1;
    }

    let worlds;

    if splitter.contains(&next) {
        worlds = count_worlds(&(next.0 - 1, next.1), max, splitter, state)
            + count_worlds(&(next.0 + 1, next.1), max, splitter, state);

        state.insert(next, worlds);
    } else {
        worlds = count_worlds(&next, max, splitter, state);
        state.insert(next, worlds);
    }

    worlds
}
