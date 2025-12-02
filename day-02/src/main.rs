fn main() {
    let input = include_str!("../input/input.txt");
    let ranges = parse(&input);

    println!("Part 1: {}", part1(&ranges));
    println!("Part 2: {}", part2(&ranges));
}

fn part1(ranges: &Vec<Range>) -> usize {
    sum_invalid(ranges, is_invalid_id1)
}

fn part2(ranges: &Vec<Range>) -> usize {
    sum_invalid(ranges, is_invalid_id2)
}

fn sum_invalid<F>(ranges: &Vec<Range>, is_invalid: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut result = 0;

    for range in ranges {
        for i in range.start..=range.end {
            if is_invalid(i) {
                result += i;
            }
        }
    }

    result
}

fn is_invalid_id1(val: usize) -> bool {
    let s = val.to_string();
    let parts = s.split_at(s.len() / 2);
    
    parts.0 == parts.1
}

fn is_invalid_id2(val: usize) -> bool {
    let chars: Vec<char> = val.to_string().chars().collect();

    let mut size = 1;

    while size <= chars.len() / 2 {
        let parts = chars
            .chunks(size)
            .map(|chunk| chunk.iter().collect())
            .collect::<Vec<String>>();

        let all_equal = parts
            .windows(2)
            .all(|w| w[0] == w[1]);

        if all_equal {
            return true;
        }

        size += 1;
    }

    false
}

fn parse(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|def| {
            let mut parts = def.split('-');

            let start = parts
                .next()
                .expect("invalid definition")
                .parse()
                .expect("invalid start of range");

            let end = parts
                .next()
                .expect("invalid definition")
                .parse()
                .expect("invalid end of range");

            Range { start, end }
        })
        .collect()
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}
