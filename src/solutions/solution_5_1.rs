use std::collections::HashMap;

pub fn solve(input: &str) -> String {
    let mut stacks: HashMap<&str, Vec<&str>> = HashMap::new();

    for (num, part) in input.split("\n\n").enumerate() {
        match num {
            0 => {
                let mut rows = part.split('\n').rev();

                let row_1 = rows.next().unwrap();
                for stack in row_1.trim().split("   ") {
                    stacks.insert(stack, Vec::new());
                }

                for row in rows {
                    for (i, actual_crate) in row.match_indices(|c| char::is_ascii_uppercase(&c)) {
                        let stack_i = ((i + 3) / 4).to_string();
                        stacks
                            .get_mut(stack_i.as_str())
                            .expect("Could not get stack")
                            .push(actual_crate);
                    }
                }
            }
            1 => {
                for row in part.trim().split('\n') {
                    let command = row[5..].split(' ').step_by(2).collect::<Vec<&str>>();
                    for _ in 0..command[0].parse::<u8>().unwrap() {
                        let popped = stacks.get_mut(command[1]).unwrap().pop().unwrap();
                        stacks.get_mut(command[2]).unwrap().push(popped);
                    }
                }
            }
            _ => panic!(),
        }
    }

    let mut output = String::new();
    for i in 1..stacks.len() + 1 {
        output += stacks[i.to_string().as_str()].last().unwrap();
    }

    output
}
