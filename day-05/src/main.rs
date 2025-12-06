use std::cmp;

fn main() {
    let input = include_str!("../input/input.txt");

    let (ranges, ids) = input.lines().filter_map(Input::from).fold(
        (Vec::new(), Vec::new()),
        |(mut ranges, mut ids), x| {
            match x {
                Input::IdRange(start, end) => ranges.push((start, end)),
                Input::Id(value) => ids.push(value),
            }
            (ranges, ids)
        },
    );

    let result = ids
        .iter()
        .filter(|&&id| ranges.iter().any(|(start, end)| id >= *start && id <= *end))
        .count();

    println!("part 1: {}", result);

    let result: usize = merge_ranges(&ranges).iter().map(|x| x.1 - x.0 + 1).sum();

    println!("part 2: {}", result);
}

fn merge_ranges(ranges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut ranges = ranges.to_vec();
    ranges.sort_by_key(|x| x.0);

    let mut result: Vec<(usize, usize)> = vec![];

    for i in 0..ranges.len() {
        let start = ranges[i].0;
        let mut end = ranges[i].1;

        if let Some(last) = result.last()
            && last.1 >= end
        {
            continue;
        }

        for j in (i + 1)..ranges.len() {
            if ranges[j].0 <= end {
                end = cmp::max(end, ranges[j].1);
            }
        }

        result.push((start, end));
    }

    result
}

enum Input {
    IdRange(usize, usize),
    Id(usize),
}

impl Input {
    fn from(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }

        if let Some((start, end)) = s.split_once('-') {
            let start = start.parse::<usize>().ok()?;
            let end = end.parse::<usize>().ok()?;
            Some(Input::IdRange(start, end))
        } else {
            Some(Input::Id(s.parse::<usize>().ok()?))
        }
    }
}
