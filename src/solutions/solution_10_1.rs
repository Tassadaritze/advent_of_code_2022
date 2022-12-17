pub fn solve(input: &str) -> String {
    let mut cycle: usize = 1;
    let mut register: isize = 1;
    let mut sum: isize = 0;

    for line in input.split('\n') {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            sum += register * cycle as isize;
        }

        let mut line_iter = line.split(' ');
        let (instruction, value) = (
            line_iter.next().unwrap(),
            line_iter.next().unwrap_or_default(),
        );

        match instruction {
            "noop" => cycle += 1,
            "addx" => {
                cycle += 1;
                if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                    sum += register * cycle as isize;
                }
                register += value.parse::<isize>().unwrap();
                cycle += 1;
            }
            _ => panic!(),
        }
    }

    sum.to_string()
}
