use std::str::FromStr;

pub fn parse_lines<T: FromStr>(input: &str) -> Result<Vec<T>, String> {
    input
        .lines()
        .enumerate()
        .map(|(line_idx, line)| {
            line.parse::<T>()
                .map_err(|_| format!("Line {}: Not a valid integer", line_idx + 1))
        })
        .collect()
}

fn part1(input: &str) {
    // Parse the input
    let parsed_input = parse_lines::<u32>(&input).unwrap();

    let n_lines = parsed_input.len();

    println!("{} lines", n_lines);

    // Iterate over the lines
    let mut total_increase = 0;
    for line in 1..n_lines {
        if parsed_input[line] >= parsed_input[line - 1] {
            total_increase += 1;
        }
    }
    // Print the total_increase
    println!("{}", total_increase);
}

fn main() {
    let part1_input = include_str!("../input.txt");
    part1(part1_input);
}
