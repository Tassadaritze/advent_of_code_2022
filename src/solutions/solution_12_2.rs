use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Node {
    elevation: u8,
    children: Vec<[i8; 2]>,
    is_end: bool,
}

pub fn solve(input: &str) -> String {
    const HEIGHT_OFFSET: u8 = 97;

    let mut nodes: HashMap<[i8; 2], Node> = HashMap::new();
    let mut starts: Vec<[i8; 2]> = Vec::new();

    for (y, line) in input.split('\n').enumerate() {
        for (x, char) in line.chars().enumerate() {
            nodes.insert(
                [x as i8, y as i8],
                Node {
                    elevation: match char {
                        'S' => 0,
                        'E' => 25,
                        _ => char as u8 - HEIGHT_OFFSET,
                    },
                    children: Vec::new(),
                    is_end: char == 'E',
                },
            );
            if char == 'S' || char == 'a' {
                starts.push([x as i8, y as i8]);
            }
        }
    }

    {
        // would like to know a better way to do this if there is one
        let clone = nodes.clone();
        for ([x, y], node) in nodes.iter_mut() {
            for offset in [-1_i8, 1] {
                for offset_coord in ['x', 'y'] {
                    let x_offset = if offset_coord == 'x' { offset } else { 0 };
                    let y_offset = if offset_coord == 'y' { offset } else { 0 };
                    match clone.get(&[x + x_offset, y + y_offset]) {
                        Some(other) => {
                            if node.elevation + 1 >= other.elevation {
                                node.children.push([x + x_offset, y + y_offset])
                            }
                        }
                        None => continue,
                    }
                }
            }
        }
    }

    println!("{:?}", nodes);

    struct Path([i8; 2], Option<Rc<Path>>);

    let mut path_lengths: Vec<u16> = Vec::new();
    for start in starts {
        let mut end: Option<Path> = None;

        let mut explored = HashSet::from([start]);
        let mut queue = VecDeque::new();

        queue.push_back(Path(start, None));

        while let Some(Path(coords, path)) = queue.pop_front() {
            let node = nodes.get(&coords).unwrap();
            if node.is_end {
                end = Some(Path(coords, path));
                break;
            };

            let path = Rc::new(Path(coords, path.clone()));

            for coords in &node.children {
                if explored.insert(*coords) {
                    queue.push_back(Path(*coords, Some(path.clone())));
                }
            }
        }

        let mut path_length: u16 = 0;
        let mut path = match end {
            Some(path) => Rc::new(path),
            None => continue,
        };
        while path.0 != start {
            path = path.1.as_ref().unwrap().clone();
            path_length += 1;
        }

        path_lengths.push(path_length);
    }

    path_lengths.sort();
    println!("{:?}", path_lengths);

    path_lengths[0].to_string()
}
