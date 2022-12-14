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
        let (opponent, me): (&str, &str) = line.split_once(' ').expect("Could not split line");

        score += shape_scores[me] as usize;

        if me
            == from_utf8(&[(opponent.as_bytes()[0] + 23 - 65) % 26 + 65])
                .expect("Could not convert back to UTF-8")
        {
            score += result_scores["draw"] as usize;
        } else {
            score += result_scores[results[opponent][me]] as usize;
        }
    }

    score.to_string()
}
