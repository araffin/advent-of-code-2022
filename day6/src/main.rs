use std::collections::HashSet;

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Convert to a vector of characters
    let characters: Vec<char> = input.chars().collect();

    // Process the input with a sliding window of 4 characters
    let mut starting_pos = 4;
    for four_characters in characters.windows(4) {
        // Create a set from the characters in the window
        let set: HashSet<char> = four_characters.iter().cloned().collect();

        // Print the set
        // println!("{:?}", set);

        // 4 unique characters
        if set.len() == 4 {
            break;
        }
        starting_pos += 1;
    }
    println!("Starting pos: {}", starting_pos);
}

fn part2() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Convert to a vector of characters
    let characters: Vec<char> = input.chars().collect();

    // Process the input with a sliding window of 14 characters
    let mut starting_pos = 14;
    for char_window in characters.windows(14) {
        // Create a set from the characters in the window
        let set: HashSet<char> = char_window.iter().cloned().collect();

        // Print the set
        // println!("{:?}", set);

        // 4 unique characters
        if set.len() == 14 {
            break;
        }
        starting_pos += 1;
    }
    println!("Starting pos: {}", starting_pos);
}

fn main() {
    part1();
    part2();
}
