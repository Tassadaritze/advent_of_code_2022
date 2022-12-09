pub fn solve(input: &str) -> String {
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut top_3: [usize; 3] = [0; 3];
    for elf in elves {
        let mut total: usize = 0;
        for item in elf.trim().split('\n') {
            total += item.parse::<usize>().expect("Could not parse as usize");
        }

        let mut lowest: (i8, usize) = (-1, 0); // (index, value)
        for (i, value) in top_3.iter_mut().enumerate() {
            if lowest.0 < 0 || *value <= lowest.1 {
                lowest = (i as i8, *value);
            };
        }
        if total > lowest.1 {
            top_3[lowest.0 as usize] = total;
        }
    }

    let mut sum: usize = 0;
    for value in top_3 {
        sum += value;
    }

    sum.to_string()
}
