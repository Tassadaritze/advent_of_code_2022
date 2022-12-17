use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Sub};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Coords {
    x: isize,
    y: isize,
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Coords {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Coords {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn solve(input: &str) -> String {
    let movement: HashMap<&str, Coords> = HashMap::from([
        ("L", Coords { x: -1, y: 0 }),
        ("R", Coords { x: 1, y: 0 }),
        ("D", Coords { x: 0, y: -1 }),
        ("U", Coords { x: 0, y: 1 }),
    ]);

    let mut head = Coords { x: 0, y: 0 };
    let mut tail = Coords { x: 0, y: 0 };
    let mut visited: HashSet<Coords> = HashSet::from([Coords { x: 0, y: 0 }]);

    for line in input.split('\n') {
        let mut line_iter = line.split(' ');
        let (direction, count): (&str, u8) = (
            line_iter.next().unwrap(),
            line_iter.next().unwrap().parse().unwrap(),
        );

        for _ in 0..count {
            head += *movement.get(direction).unwrap();

            let diff = head - tail;
            if diff.x.abs() > 1 || diff.y.abs() > 1 {
                tail = match diff.x.abs() > diff.y.abs() {
                    true => {
                        head + if diff.x.is_positive() {
                            Coords { x: -1, y: 0 }
                        } else {
                            Coords { x: 1, y: 0 }
                        }
                    }
                    false => {
                        head + if diff.y.is_positive() {
                            Coords { x: 0, y: -1 }
                        } else {
                            Coords { x: 0, y: 1 }
                        }
                    }
                }
            }

            visited.insert(tail);
        }
    }

    visited.len().to_string()
}
