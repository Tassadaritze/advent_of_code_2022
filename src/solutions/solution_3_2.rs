pub fn solve(input: &str) -> String {
    const LOWERCASE_PRIORITY_OFFSET: u8 = 96;
    const UPPERCASE_PRIORITY_OFFSET: u8 = 38;

    let mut sum: usize = 0;
    for set in input.trim().split('\n').collect::<Vec<&str>>().chunks(3) {
        let mut common: Option<char> = None;
        'set_1: for item in set[0].chars() {
            for item_2 in set[1].chars() {
                if item == item_2 {
                    for item_3 in set[2].chars() {
                        if item_2 == item_3 {
                            common = Some(item);
                            break 'set_1;
                        }
                    }
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
