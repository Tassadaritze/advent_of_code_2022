pub fn solve(input: &str) -> String {
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in input.split('\n') {
        map.push(
            line.chars()
                .map(|el| el.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }

    let mut best_score: usize = 1;
    for (y, line) in map.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if y == 0 || y == map.len() - 1 || x == 0 || x == line.len() - 1 {
                continue;
            }

            println!("Checking {} at x:{},y:{}...", val, x, y);

            let mut score: usize = 1;

            // left/right
            let mut left = map[y].iter().take(x).rev().enumerate().peekable();
            while let Some((distance, other)) = left.next() {
                if other >= val {
                    println!("To the left {} blocks it!", other);
                    score *= distance + 1;
                    break;
                }
                if left.peek().is_none() {
                    score *= distance + 1;
                }
            }
            let mut right = map[y].iter().skip(x + 1).enumerate().peekable();
            while let Some((distance, other)) = right.next() {
                if other >= val {
                    println!("To the right {} blocks it!", other);
                    score *= distance + 1;
                    break;
                }
                if right.peek().is_none() {
                    score *= distance + 1;
                }
            }

            // up/down
            let mut up = map.iter().take(y).rev().enumerate().peekable();
            while let Some((distance, other)) = up.next() {
                if &other[x] >= val {
                    println!("Above {} blocks it!", &other[x]);
                    score *= distance + 1;
                    break;
                }
                if up.peek().is_none() {
                    score *= distance + 1;
                }
            }
            let mut down = map.iter().skip(y + 1).enumerate().peekable();
            while let Some((distance, other)) = down.next() {
                if &other[x] >= val {
                    println!("Below {} blocks it!", &other[x]);
                    score *= distance + 1;
                    break;
                }
                if down.peek().is_none() {
                    score *= distance + 1;
                }
            }

            println!("Score is {}!", score);

            if score > best_score {
                best_score = score;
            }
        }
    }

    for line in map {
        println!("{:?}", line);
    }

    best_score.to_string()
}
