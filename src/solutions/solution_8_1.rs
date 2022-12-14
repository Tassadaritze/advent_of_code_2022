pub fn solve(input: &str) -> String {
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in input.split('\n') {
        map.push(
            line.chars()
                .map(|el| el.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }

    let mut visible: usize = 0;
    for (y, line) in map.iter().enumerate() {
        'main: for (x, val) in line.iter().enumerate() {
            if y == 0 || y == map.len() - 1 || x == 0 || x == line.len() - 1 {
                visible += 1;
                continue;
            }

            println!("Checking {} at x:{},y:{}...", val, x, y);

            // left/right
            let mut left = map[y].iter().take(x).peekable();
            while let Some(other) = left.next() {
                if other >= val {
                    println!("To the left {} blocks it!", other);
                    break;
                }
                if left.peek().is_none() {
                    visible += 1;
                    continue 'main;
                }
            }
            let mut right = map[y].iter().skip(x + 1).peekable();
            while let Some(other) = right.next() {
                if other >= val {
                    println!("To the right {} blocks it!", other);
                    break;
                }
                if right.peek().is_none() {
                    visible += 1;
                    continue 'main;
                }
            }

            // up/down
            let mut up = map.iter().take(y).peekable();
            while let Some(other) = up.next() {
                if &other[x] >= val {
                    println!("Above {} blocks it!", &other[x]);
                    break;
                }
                if up.peek().is_none() {
                    visible += 1;
                    continue 'main;
                }
            }
            let mut down = map.iter().skip(y + 1).peekable();
            while let Some(other) = down.next() {
                if &other[x] >= val {
                    println!("Below {} blocks it!", &other[x]);
                    break;
                }
                if down.peek().is_none() {
                    visible += 1;
                    continue 'main;
                }
            }
        }
    }

    for line in map {
        println!("{:?}", line);
    }

    visible.to_string()
}
