pub fn solve(input: &str) -> String {
    let mut cycle: usize = 1;
    let mut register: isize = 1;
    let mut out: [[&str; 40]; 6] = [[""; 40]; 6];

    for line in input.split('\n') {
        out[(cycle - 1) / 40][(cycle - 1) % 40] =
            if (register - 1..=register + 1).contains(&(((cycle - 1) % 40) as isize)) {
                "#"
            } else {
                "."
            };

        let mut line_iter = line.split(' ');
        let (instruction, value) = (
            line_iter.next().unwrap(),
            line_iter.next().unwrap_or_default(),
        );

        match instruction {
            "noop" => cycle += 1,
            "addx" => {
                cycle += 1;
                out[(cycle - 1) / 40][(cycle - 1) % 40] =
                    if (register - 1..=register + 1).contains(&(((cycle - 1) % 40) as isize)) {
                        "#"
                    } else {
                        "."
                    };
                register += value.parse::<isize>().unwrap();
                cycle += 1;
            }
            _ => panic!(),
        }
    }

    let mut out_str = String::new();
    for row in out {
        for char in row {
            out_str += char;
        }
        out_str += "\n";
    }

    out_str.to_string()
}
