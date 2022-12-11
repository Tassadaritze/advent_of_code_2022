use std::collections::VecDeque;

pub fn solve(input: &str) -> String {
    let mut buffer: VecDeque<char> = VecDeque::new();
    let mut pos: u16 = 1;

    for char in input.chars() {
        if let Some(el) = buffer.iter().enumerate().find(|(_, el)| *el == &char) {
            buffer.drain(..el.0 + 1);
        }

        buffer.push_back(char);

        if buffer.len() > 13 {
            break;
        }

        pos += 1;
    }

    println!("{:?}", buffer);

    pos.to_string()
}
