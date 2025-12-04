fn main() {
    let input = include_str!("../input/input.txt");
    let mut matrix = Matrix::parse(input);

    println!("part 1: {}", part1(&matrix));
    println!("part 2: {}", part2(&mut matrix));
}

fn part1(matrix: &Matrix) -> usize {
    matrix.filter_cells(Matrix::is_accessible).iter().count()
}

fn part2(matrix: &mut Matrix) -> usize {
    let mut result = 0;

    loop {
        let count = matrix.remove_accessibles();

        if count == 0 {
            return result;
        }

        result += count;
    }
}

struct Matrix {
    data: Vec<Vec<bool>>,
}

impl Matrix {
    fn parse(input: &str) -> Self {
        let mut data = vec![];

        for line in input.lines() {
            let row = line.chars().map(|c| c == '@').collect::<Vec<bool>>();
            data.push(row);
        }

        Matrix { data }
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.data.get(y)?.get(x).copied()
    }

    fn filter_cells<F>(&self, func: F) -> Vec<(usize, usize)>
    where
        F: Fn(usize, usize, &Matrix) -> bool,
    {
        let mut result = Vec::<(usize, usize)>::new();

        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if func(x, y, self) {
                    result.push((x, y));
                }
            }
        }

        result
    }

    fn remove_accessibles(&mut self) -> usize {
        let mut count = 0;

        for (x, y) in self.filter_cells(Matrix::is_accessible) {
            self.data[y][x] = false;
            count += 1;
        }

        count
    }

    fn is_accessible(x: usize, y: usize, matrix: &Matrix) -> bool {
        if let Some(true) = matrix.get(x, y) {
            let neighbor_cells = Matrix::neighbours(x, y);
            let n = neighbor_cells
                .iter()
                .map(|c| matrix.get(c.0, c.1).unwrap_or(false))
                .filter(|b| *b)
                .count();

            return n < 4;
        }

        false
    }

    // returning an iterator would be better...
    fn neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut out = Vec::new();

        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && ny >= 0 {
                    out.push((nx as usize, ny as usize));
                }
            }
        }

        out
    }
}
