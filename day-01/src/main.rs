fn main() {
    let input = include_str!("../input/input.txt");
    let moves: Vec<_> = input.lines().map(parse).collect();
    
    println!("Part 1: {}", part1(&moves));
    println!("Part 2: {}", part2(&moves));
}

fn part1(moves: &Vec<Move>) -> usize {
    let mut zeros = 0;
    let mut dial = 50;

    for m in moves {
        dial = match m {
            Move::Left { c } => (dial - *c as i32) % 100,
            Move::Right { c } => (dial + *c as i32) % 100,
        };

        if dial == 0 {
            zeros += 1;
        }
    }

    zeros
}

fn part2(moves: &Vec<Move>) -> usize {
    let mut zeros = 0;
    let mut dial = 50;

    for m in moves {
        let (steps, step_value) = match m {
            Move::Left { c } => (*c, -1),
            Move::Right { c } => (*c, 1),
        };

        // I did not get the correct result by calculation so I am iterating it instead 🤮
        for _ in 0..steps {
            dial = (dial + step_value) % 100;

            if dial == 0 {
                zeros += 1;
            }
        }
    }

    zeros
}

fn parse(line: &str) -> Move {
    let (dir, count) = line.split_at(1);
    let c = count.parse::<usize>().expect("invalid count");

    match dir {
        "L" => Move::Left { c },
        "R" => Move::Right { c },
        _ => panic!("invalid direction"),
    }
}

#[derive(Debug)]
enum Move {
    Left { c: usize },
    Right { c: usize },
}
