fn main() {
    let input = include_str!("../input/input.txt");

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let numbers: Vec<Vec<usize>> = input.lines().filter_map(parse_numbers).collect();
    let operations = parse_operations(input.lines().last().unwrap());

    let mut result = 0;

    for i in 0..numbers[0].len() {
        let operands: Vec<_> = numbers.iter().map(|line| line[i]).collect();
        result += operations[i](&operands);
    }

    result
}

fn part2(input: &str) -> usize {
    let numbers = parse_rtl(&input);
    let operations = parse_operations(input.lines().last().unwrap());

    let mut result = 0;

    for i in 0..numbers.len() {
        let val = operations[i](&numbers[i]);
        result += val;
    }

    result
}

fn parse_rtl(input: &str) -> Vec<Vec<usize>> {
    let lines: Vec<_> = input.lines().collect();
    let (operations_def, numbers_def) = lines.split_last().unwrap();

    let mut widths: Vec<usize> = operations_def
        .split(|c| c == '+' || c == '*')
        .map(|x| x.len())
        .skip(1)
        .collect();

    // fix last column of input
    *widths.last_mut().unwrap() += 1;

    let mut blocks: Vec<Vec<String>> = Vec::new();
    let mut start = 0;

    for &width in &widths {
        let end = start + width;

        let block: Vec<String> = numbers_def
            .iter()
            .map(|line| line[start..end].to_string())
            .collect();

        blocks.push(block);
        start = end + 1;
    }

    let mut result: Vec<Vec<usize>> = vec![];

    for block in blocks {
        let mut numbers: Vec<usize> = vec![];

        for i in (0..block[0].len()).rev() {
            let num = block
                .iter()
                .filter_map(|b| b.chars().nth(i))
                .collect::<String>()
                .trim()
                .parse::<usize>()
                .unwrap();
            
            numbers.push(num);
        }

        result.push(numbers);
    }

    result
}

fn parse_numbers(line: &str) -> Option<Vec<usize>> {
    let numbers: Vec<_> = line
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    (!numbers.is_empty()).then_some(numbers)
}

fn parse_operations(line: &str) -> Vec<fn(&[usize]) -> usize> {
    line.split_whitespace()
        .filter_map(|c| match c {
            "+" => Some(add as fn(&[usize]) -> usize),
            "*" => Some(multiply as fn(&[usize]) -> usize),
            _ => None,
        })
        .collect()
}

fn add(numbers: &[usize]) -> usize {
    numbers.iter().sum()
}

fn multiply(numbers: &[usize]) -> usize {
    numbers.iter().product()
}
