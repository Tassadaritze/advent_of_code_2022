use std::collections::VecDeque;

#[derive(Debug)]
enum Operation {
    Add(),
    Multiply(),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    operation_value: Option<usize>,
    test_divisor: u8,
    if_true: u8,
    if_false: u8,
    inspect_count: usize,
}

impl Monkey {
    fn new(
        items: VecDeque<usize>,
        operation: Operation,
        operation_value: Option<usize>,
        test_divisor: u8,
        if_true: u8,
        if_false: u8,
    ) -> Self {
        Monkey {
            items,
            operation,
            operation_value,
            test_divisor,
            if_true,
            if_false,
            inspect_count: 0,
        }
    }

    fn add(&mut self, item: usize, lcm: usize) -> usize {
        self.inspect_count += 1;
        item + match self.operation_value {
            Some(val) => val,
            None => item,
        } % lcm
    }

    fn multiply(&mut self, item: usize, lcm: usize) -> usize {
        self.inspect_count += 1;
        item * match self.operation_value {
            Some(val) => val,
            None => item,
        } % lcm
    }

    fn test(&self, item: usize) -> u8 {
        if item % self.test_divisor as usize == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

pub fn solve(input: &str) -> String {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey in input.split("\n\n") {
        let mut line = monkey.split('\n').skip(1);

        let items: VecDeque<usize> = line.next().unwrap()[18..]
            .split(", ")
            .map(|el| el.parse().unwrap())
            .collect();

        let op = line.next().unwrap();
        let operation = if op.contains('+') {
            Operation::Add()
        } else if op.contains('*') {
            Operation::Multiply()
        } else {
            panic!()
        };
        let operation_value: Option<usize> = match &op[25..] == "old" {
            true => None,
            false => Some(op[25..].parse().unwrap()),
        };

        let test_divisor: u8 = line.next().unwrap()[21..].parse().unwrap();

        let if_true: u8 = line.next().unwrap()[29..].parse().unwrap();

        let if_false: u8 = line.next().unwrap()[30..].parse().unwrap();

        monkeys.push(Monkey::new(
            items,
            operation,
            operation_value,
            test_divisor,
            if_true,
            if_false,
        ));
    }

    let mut lcm: usize = 1;
    for monkey in monkeys.iter() {
        lcm *= monkey.test_divisor as usize;
    }

    for round in 0_u16..10000 {
        println!("Round {round}:");
        for i in 0..monkeys.len() {
            if monkeys[i].items.is_empty() {
                continue;
            }

            while let Some(item) = monkeys[i].items.pop_front() {
                let item = match monkeys[i].operation {
                    Operation::Add() => monkeys[i].add(item, lcm),
                    Operation::Multiply() => monkeys[i].multiply(item, lcm),
                };

                let next_monkey = monkeys[i].test(item) as usize;
                monkeys[next_monkey].items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|el1, el2| el2.inspect_count.cmp(&el1.inspect_count));

    println!("{:#?}", monkeys);

    (monkeys[0].inspect_count * monkeys[1].inspect_count).to_string()
}
