fn main() {
    let input = include_str!("../input/input.txt");
    let banks: Vec<_> = input.lines().collect();

    println!("part 1: {}", part1(&banks));
    println!("part 2: {}", part2(&banks));
}

fn part1(banks: &[&str]) -> usize {
    banks.iter().map(|b| max_joltage_2(b)).sum()
}

fn part2(banks: &[&str]) -> usize {
    banks.iter().map(|b| max_joltage_n(b, 12)).sum()
}

fn max_joltage_2(bank: &str) -> usize {
    let chars = bank.chars().collect::<Vec<char>>();

    let max = *chars.iter().max().unwrap();
    let index_max = chars.iter().position(|&c| c == max).unwrap();

    if index_max == chars.len() - 1 {
        let first = chars[..index_max].iter().max().unwrap();
        return format!("{}{}", first, max).parse().unwrap();
    } else {
        let second = chars[index_max + 1..].iter().max().unwrap();
        return format!("{}{}", max, second).parse().unwrap();
    }
}

fn max_joltage_n(bank: &str, n: usize) -> usize {
    let chars = bank.chars().collect::<Vec<char>>();

    let mut result = Vec::new();

    let mut id = 0;
    
    for slot in 0..n {
        let missing_slots = n - slot - 1;
        let end_with_buffer = chars.len() - missing_slots;
        
        let mut max = chars[id];
        
        for i in (id + 1)..end_with_buffer {
            if chars[i] > max {
                max = chars[i];
                id = i;
            }
        }

        result.push(max);
        id += 1;
    }

    String::from_iter(result).parse().unwrap()
}
