pub fn solve(input: &str) -> String {
    let mut buffer: [Option<char>; 3] = [None; 3];
    let mut pos: u16 = 1;

    for (i, char) in input.chars().take(3).enumerate() {
        buffer[i] = Some(char);
        pos += 1;
    }

    'char_loop: for char in input.chars().skip(3) {
        for prev_char in buffer.into_iter().flatten() {
            if char == prev_char {
                buffer.rotate_left(1);
                buffer[2] = Some(char);
                pos += 1;
                continue 'char_loop;
            }
        }

        for prev_char in buffer[0..2].iter().flatten() {
            if buffer[2].unwrap() == *prev_char {
                buffer.rotate_left(1);
                buffer[2] = Some(char);
                pos += 1;
                continue 'char_loop;
            }
        }

        if buffer[0].unwrap() == buffer[1].unwrap() {
            buffer.rotate_left(1);
            buffer[2] = Some(char);
            pos += 1;
            continue 'char_loop;
        }

        break;
    }

    pos.to_string()
}
