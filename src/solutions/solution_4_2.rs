pub fn solve(input: &str) -> String {
    let mut count: usize = 0;
    for pair in input.trim().split('\n') {
        let mut assignments = pair.split(',');
        let (first, second) = (assignments.next().unwrap(), assignments.next().unwrap());

        let mut range = first.split('-');
        let (first_lower, first_upper): (u8, u8) = (
            range.next().unwrap().parse().unwrap(),
            range.next().unwrap().parse().unwrap(),
        );

        let mut range = second.split('-');
        let (second_lower, second_upper): (u8, u8) = (
            range.next().unwrap().parse().unwrap(),
            range.next().unwrap().parse().unwrap(),
        );

        if first_upper >= second_lower && first_lower <= second_upper {
            count += 1;
        }
    }

    count.to_string()
}
