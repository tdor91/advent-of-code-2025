use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/input.txt");
    let points = input.lines().map(Point::from).collect::<Vec<_>>();

    let mut distances = Vec::new();

    for (i, a) in points.iter().enumerate() {
        for b in &points[i + 1..] {
            distances.push(((a, b), a.distance_to(b)));
        }
    }

    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("part 1: {}", part1(&distances));
    println!("part 2: {}", part2(&points, &distances));
}

fn part1(distances: &Vec<((&Point, &Point), f64)>) -> usize {
    let mut network = Network::new();

    for i in 0..1000 {
        let ((a, b), _) = distances[i];
        network.connect(a, b);
    }

    let mut sizes: Vec<_> = network.circuits.iter().map(|c| c.len()).collect();
    sizes.sort_by_key(|x| std::cmp::Reverse(*x));

    sizes.iter().take(3).product()
}

fn part2(points: &Vec<Point>, distances: &Vec<((&Point, &Point), f64)>) -> usize {
    let mut network = Network::new();

    let mut i = 0;

    loop {
        let ((a, b), _) = distances[i];
        let connections = network.connect(a, b);

        if connections == points.len() {
            break a.x * b.x;
        }

        i += 1;
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn from(line: &str) -> Self {
        let mut parts = line.split(',');

        Point {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        }
    }

    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x.abs_diff(other.x) as f64;
        let dy = self.y.abs_diff(other.y) as f64;
        let dz = self.z.abs_diff(other.z) as f64;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

struct Network {
    circuits: Vec<HashSet<Point>>,
}

impl Network {
    fn new() -> Self {
        Network { circuits: vec![] }
    }

    fn connect(&mut self, a: &Point, b: &Point) -> usize {
        let index_a = self.circuits.iter().position(|c| c.contains(a));
        let index_b = self.circuits.iter().position(|c| c.contains(b));

        match (index_a, index_b) {
            (Some(ia), Some(ib)) if ia == ib => {}

            (Some(ia), None) => {
                self.circuits[ia].insert(b.clone());
            }

            (None, Some(ib)) => {
                self.circuits[ib].insert(a.clone());
            }

            (None, None) => {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(a.clone());
                new_circuit.insert(b.clone());
                self.circuits.push(new_circuit);
            }

            (Some(ia), Some(ib)) => {
                // remove larger index first to avoid shifting
                let (first, second) = (ia.max(ib), ia.min(ib));
                let c1 = self.circuits.remove(first);
                let c2 = self.circuits.remove(second);

                let mut combined = c1;
                combined.extend(c2);
                self.circuits.push(combined);
            }
        }

        // return the size of the biggest circuit
        self.circuits.iter().map(|c| c.len()).max().unwrap()
    }
}
