pub fn solve(input: &str) -> String {
    const LOWERCASE_PRIORITY_OFFSET: u8 = 96;
    const UPPERCASE_PRIORITY_OFFSET: u8 = 38;

    let mut sum: usize = 0;
    for rucksack in input.trim().split('\n') {
        let mut common: Option<char> = None;
        'half_1: for item in rucksack.chars().take(rucksack.len() / 2) {
            for item_2 in rucksack.chars().skip(rucksack.len() / 2) {
                if item == item_2 {
                    common = Some(item);
                    break 'half_1;
                }
            }
        }

        match common {
            Some(common) => {
                let mut b: [u8; 1] = [0];
                common.encode_utf8(&mut b);
                sum += b[0] as usize
                    - match common.is_ascii_uppercase() {
                        true => UPPERCASE_PRIORITY_OFFSET,
                        false => LOWERCASE_PRIORITY_OFFSET,
                    } as usize
            }
            None => panic!("Could not find a common item"),
        }
    }

    sum.to_string()
}
