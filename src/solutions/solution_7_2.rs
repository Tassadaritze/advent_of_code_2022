use std::collections::VecDeque;

struct Dir {
    name: String,
    contents: Vec<Dir>,
    size: usize,
    is_directory: bool,
}

impl Dir {
    fn new(name: &str, is_directory: bool, size: usize) -> Self {
        Dir {
            name: name.to_string(),
            contents: Vec::new(),
            size,
            is_directory,
        }
    }

    fn add_file(&mut self, name: &str, size: &str) {
        self.contents
            .push(Dir::new(name, false, size.parse().unwrap()));
    }

    fn add_dir(&mut self, name: &str) {
        self.contents.push(Dir::new(name, true, 0));
    }

    fn get_size(&mut self) -> usize {
        let mut sum: usize = 0;

        for item in self.contents.iter_mut() {
            sum += if item.is_directory {
                item.get_size()
            } else {
                item.size
            };
        }

        self.size = sum;

        sum
    }

    fn get_deletion_candidates(&self, threshold: usize) -> Vec<&Dir> {
        let mut candidates: Vec<&Dir> = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(dir) = queue.pop_front() {
            if dir.is_directory && dir.size >= threshold {
                candidates.push(dir);
            }
            for item in &dir.contents {
                queue.push_back(item);
            }
        }

        candidates
    }
}

pub fn solve(input: &str) -> String {
    const REQUIRED_SPACE: u32 = 70_000_000 - 30_000_000;

    let mut dir: Dir = Dir::new("/", true, 0);
    let mut current_path: Vec<&str> = Vec::new();

    for line in input.split('\n') {
        match line.starts_with('$') {
            true => {
                let mut line_iter = line.split(' ');
                let (_, command, option): (_, &str, &str) = (
                    line_iter.next().unwrap(),
                    line_iter.next().unwrap(),
                    line_iter.next().unwrap_or_default(),
                );

                if command == "cd" && option != "/" {
                    match option {
                        ".." => {
                            current_path.pop();
                        }
                        option => {
                            current_path.push(option);
                        }
                    };
                }
            }
            false => {
                let mut line_iter = line.split(' ');
                let (kind, name): (&str, &str) =
                    (line_iter.next().unwrap(), line_iter.next().unwrap());

                let mut dir = &mut dir;
                for item in current_path.iter() {
                    dir = dir.contents.iter_mut().find(|el| el.name == *item).unwrap();
                }

                match kind {
                    "dir" => dir.add_dir(name),
                    size => dir.add_file(name, size),
                };
            }
        }
    }

    println!("{}", dir.get_size());
    let missing_space = dir.size - REQUIRED_SPACE as usize;
    println!(
        "We are missing {} - {} = {}.",
        dir.size, REQUIRED_SPACE, missing_space
    );

    let mut candidates = dir.get_deletion_candidates(missing_space);
    candidates.sort_by(|el, el2| el.size.partial_cmp(&el2.size).unwrap());
    candidates[0].size.to_string()
}
