use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operations: Vec<String>,
    divisible_by: usize,
    next_monkeys: Vec<usize>,
    n_inspections: usize,
}

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Number of rounds to simulate
    let n_rounds = 20;

    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey_lines in input.split("\n\n") {
        let mut line_iter = monkey_lines.lines().skip(1);
        // Parse the entry
        let items: VecDeque<usize> = line_iter
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();
        let operations: Vec<String> = line_iter
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .skip(1)
            .map(|s| s.to_string())
            .collect();

        let divisible_by = line_iter
            .next()
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();

        let next_monkeys: Vec<usize> = line_iter
            .take(2)
            .map(|line| {
                line.split_once("monkey ")
                    .unwrap()
                    .1
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();

        monkeys.push(Monkey {
            items,
            operations,
            divisible_by,
            next_monkeys,
            n_inspections: 0,
        });
    }
    // println!("{:?}", monkeys);

    for _ in 0..n_rounds {
        for i in 0..monkeys.len() {
            // Copy the current monkey to be able
            // to modify the list later
            let monkey = monkeys[i].clone();

            for item in monkey.items {
                monkeys.get_mut(i).unwrap().n_inspections += 1;

                let value = match monkey.operations[1].parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => item,
                };

                let mut new_value = match monkey.operations[0].as_str() {
                    "+" => item + value,
                    "*" => item * value,
                    _ => panic!("Unknown operation"),
                };
                // Divide by 3 and floor to the nearest integer
                // (here integer division)
                new_value /= 3;
                let next_monkey = if new_value % monkey.divisible_by == 0 {
                    monkey.next_monkeys[0]
                } else {
                    monkey.next_monkeys[1]
                };
                // Move the item to the next monkey
                // we need to remove it from the current monkey first
                monkeys.get_mut(i).unwrap().items.pop_front();
                monkeys
                    .get_mut(next_monkey)
                    .unwrap()
                    .items
                    .push_back(new_value);
            }
        }
    }

    // for (idx, monkey) in monkeys.iter().enumerate() {
    //     println!("{} {:?} {}", idx, monkey.items, monkey.n_inspections);
    // }

    let mut n_inspections: Vec<usize> = monkeys.iter().map(|m| m.n_inspections).collect();
    n_inspections.sort();
    // Multiply the top two numbers
    let monkey_business: usize = n_inspections.iter().rev().take(2).product();

    println!("Part 1: {}", monkey_business);
}

fn part2() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Number of rounds to simulate
    let n_rounds = 10000;

    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey_lines in input.split("\n\n") {
        let mut line_iter = monkey_lines.lines().skip(1);
        // Parse the entry
        let items: VecDeque<usize> = line_iter
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();
        let operations: Vec<String> = line_iter
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .skip(1)
            .map(|s| s.to_string())
            .collect();

        let divisible_by = line_iter
            .next()
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();

        let next_monkeys: Vec<usize> = line_iter
            .take(2)
            .map(|line| {
                line.split_once("monkey ")
                    .unwrap()
                    .1
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();

        monkeys.push(Monkey {
            items,
            operations,
            divisible_by,
            next_monkeys,
            n_inspections: 0,
        });
    }
    // println!("{:?}", monkeys);

    let modulus = monkeys.iter().map(|m| m.divisible_by).product::<usize>();

    for _ in 0..n_rounds {
        for i in 0..monkeys.len() {
            // Copy the current monkey to be able
            // to modify the list later
            let monkey = monkeys[i].clone();

            for item in monkey.items {
                monkeys.get_mut(i).unwrap().n_inspections += 1;

                let value = match monkey.operations[1].parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => item,
                };

                let mut new_value = match monkey.operations[0].as_str() {
                    "+" => item + value,
                    "*" => item * value,
                    _ => panic!("Unknown operation"),
                };
                // Use modulus to avoid overflow
                new_value %= modulus;

                let next_monkey = if new_value % monkey.divisible_by == 0 {
                    monkey.next_monkeys[0]
                } else {
                    monkey.next_monkeys[1]
                };
                // Move the item to the next monkey
                // we need to remove it from the current monkey first
                monkeys.get_mut(i).unwrap().items.pop_front();
                monkeys
                    .get_mut(next_monkey)
                    .unwrap()
                    .items
                    .push_back(new_value);
            }
        }
    }

    // for (idx, monkey) in monkeys.iter().enumerate() {
    //     println!("{} {:?} {}", idx, monkey.items, monkey.n_inspections);
    // }

    let mut n_inspections: Vec<usize> = monkeys.iter().map(|m| m.n_inspections).collect();
    n_inspections.sort();
    // Multiply the top two numbers
    let monkey_business: usize = n_inspections.iter().rev().take(2).product();

    println!("Part 2: {}", monkey_business);
}

fn main() {
    part1();
    part2();
}
