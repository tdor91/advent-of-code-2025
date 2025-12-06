// 123 328  51 64 
//  45 64  387 23 
//   6 98  215 314
// *   +   *   +  

fn main() {
    let input = include_str!("../input/input.txt");

    let numbers: Vec<Vec<usize>> = input.lines().filter_map(parse_numbers).collect();
    let operations = parse_operations(input.lines().last().unwrap());

    let mut result = 0;

    for i in 0..numbers[0].len() {
        result += operations[i](&get_operands(&numbers, i));
    }

    println!("part 1: {}", result);
}

fn get_operands(numbers: &[Vec<usize>], i: usize) -> Vec<usize> {
    numbers.iter().map(|line| line[i]).collect()
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
