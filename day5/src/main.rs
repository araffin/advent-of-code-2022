use regex::Regex;

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    // read the first line and get its length
    let line_length = input.lines().nth(0).unwrap().len();

    // Compute the number of stacks
    // each stack is three characters wide
    // and there is a space bewteen each stack
    let num_stacks = line_length / 4 + 1; // Euclidian division

    println!("Number of stacks: {}", num_stacks);

    // Find the max stack size
    let mut max_stack_size = 0;
    for line in input.lines() {
        if line.is_empty() {
            max_stack_size -= 1;
            break;
        }
        max_stack_size += 1;
    }

    // Each stack is a vector of characters
    let mut stacks: Vec<Vec<char>> = Vec::new();

    // Initialize the stacks
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    // Fill the stacks
    for i in 0..max_stack_size {
        let line = input.lines().nth(max_stack_size - i - 1).unwrap();
        for j in 0..num_stacks {
            let index = j * 3 + j + 1;
            match line.chars().nth(index) {
                Some(c) if c.is_alphanumeric() => stacks[j].push(c),
                _ => continue,
            }
        }
    }

    // println!("Stacks: {:?}", stacks);
    let pattern = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    // Read and execute the instructions
    // Start from max_stack_size + 2
    for line in input.lines().skip(max_stack_size + 2) {
        let captures = pattern.captures(line).unwrap();
        let num_to_move = captures[1].parse::<usize>().unwrap();
        let start_stack = captures[2].parse::<usize>().unwrap();
        let finish_stack = captures[3].parse::<usize>().unwrap();

        // Move the top num_to_move characters from start_stack to finish_stack
        for _ in 0..num_to_move {
            let character = stacks[start_stack - 1].pop().unwrap();
            stacks[finish_stack - 1].push(character);
        }

        // println!(
        //     "Moving {} from stack {} to stack {}",
        //     num_to_move, start_stack, finish_stack
        // );
    }

    // Print the top of each stack
    print!("Top of each stack: ");
    for stack in stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
}

fn part2() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    // read the first line and get its length
    let line_length = input.lines().nth(0).unwrap().len();

    // Compute the number of stacks
    // each stack is three characters wide
    // and there is a space bewteen each stack
    let num_stacks = line_length / 4 + 1; // Euclidian division

    println!("Number of stacks: {}", num_stacks);

    // Find the max stack size
    let mut max_stack_size = 0;
    for line in input.lines() {
        if line.is_empty() {
            max_stack_size -= 1;
            break;
        }
        max_stack_size += 1;
    }

    // Each stack is a vector of characters
    let mut stacks: Vec<Vec<char>> = Vec::new();

    // Initialize the stacks
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    // Fill the stacks
    for i in 0..max_stack_size {
        let line = input.lines().nth(max_stack_size - i - 1).unwrap();
        for j in 0..num_stacks {
            let index = j * 3 + j + 1;
            match line.chars().nth(index) {
                Some(c) if c.is_alphanumeric() => stacks[j].push(c),
                _ => continue,
            }
        }
    }

    // println!("Stacks: {:?}", stacks);
    let pattern = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    // Read and execute the instructions
    // Start from max_stack_size + 2
    for line in input.lines().skip(max_stack_size + 2) {
        let captures = pattern.captures(line).unwrap();
        let num_to_move = captures[1].parse::<usize>().unwrap();
        let start_stack = captures[2].parse::<usize>().unwrap();
        let finish_stack = captures[3].parse::<usize>().unwrap();

        // Move the top num_to_move characters from start_stack to finish_stack
        // while keeping the original order
        let mut temp_stack: Vec<char> = Vec::new();
        for _ in 0..num_to_move {
            let character = stacks[start_stack - 1].pop().unwrap();
            temp_stack.push(character);
        }
        for _ in 0..num_to_move {
            let character = temp_stack.pop().unwrap();
            stacks[finish_stack - 1].push(character);
        }
    }

    // Print the top of each stack
    print!("Top of each stack: ");
    for stack in stacks.iter() {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();

    // Create a string from a vector of characters
    // let mut final_string: String = String::new();
    // for stack in stacks.iter() {
    //     let character = stack[stack.len() - 1];
    //     final_string.push(character);
    // }

    // println!("Final string: {}", final_string);
}
fn main() {
    part1();
    part2();
}
