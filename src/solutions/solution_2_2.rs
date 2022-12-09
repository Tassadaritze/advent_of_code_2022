use std::collections::HashMap;
use std::str::from_utf8;

pub fn solve(input: &str) -> String {
    let shape_scores: HashMap<&str, u8> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let result_scores: HashMap<&str, u8> = HashMap::from([("lose", 0), ("draw", 3), ("win", 6)]);
    let results: HashMap<&str, HashMap<&str, &str>> = HashMap::from([
        ("A", HashMap::from([("Y", "win"), ("Z", "lose")])),
        ("B", HashMap::from([("X", "lose"), ("Z", "win")])),
        ("C", HashMap::from([("X", "win"), ("Y", "lose")])),
    ]);

    let mut score: usize = 0;
    for line in input.trim().split('\n') {
        let (opponent, goal): (&str, &str) = line.split_once(' ').expect("Could not split line");

        let opponent_bytes = &[(opponent.as_bytes()[0] + 23 - 65) % 26 + 65];
        let converted_opponent =
            from_utf8(opponent_bytes).expect("Could not convert back to UTF-8");
        let me = match goal {
            "Y" => converted_opponent,
            "X" | "Z" => {
                let goal = if goal == "X" { "lose" } else { "win" };
                results[opponent]
                    .iter()
                    .find_map(|(key, val)| if val == &goal { Some(key) } else { None })
                    .expect("Could not get losing/winning move")
            }
            _ => panic!("Could not match goal for the round"),
        };

        score += shape_scores[me] as usize;

        if me == converted_opponent {
            score += result_scores["draw"] as usize;
        } else {
            score += result_scores[results[opponent][me]] as usize;
        }
    }

    score.to_string()
}
