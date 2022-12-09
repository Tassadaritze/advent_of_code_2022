pub fn solve(input: &str) -> String {
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut highest: usize = 0;
    for elf in elves {
        let mut total: usize = 0;
        for item in elf.trim().split('\n') {
            total += item.parse::<usize>().expect("Could not parse as usize");
        }
        if total > highest {
            highest = total;
        };
    }

    highest.to_string()
}
