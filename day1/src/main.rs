// use std::str::FromStr;

// pub fn parse_lines<T: FromStr>(input: &str) -> Result<Vec<T>, String> {
//     input
//         .lines()
//         .enumerate()
//         .map(|(line_idx, line)| {
//             line.parse::<T>()
//                 .map_err(|_| format!("Line {}: Not a valid integer", line_idx + 1))
//         })
//         .collect()
// }

fn part1() {
    // Parse the input
    let part1_input = std::fs::read_to_string("input_part1.txt").unwrap();

    let mut max_calories = 0;
    let mut current_calories = 0;

    for line in part1_input.lines() {
        // Try to parse an int from the line
        // and catch the error otherwise
        match line.parse::<i32>() {
            Ok(n) => {
                current_calories += n;
            }
            Err(_) => {
                if current_calories >= max_calories {
                    max_calories = current_calories;
                }
                current_calories = 0;
                continue;
            }
        };
    }
    // Print the max_calories
    println!("max_calories={}", max_calories);
}

fn main() {
    part1();
}
