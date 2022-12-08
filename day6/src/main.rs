use std::collections::HashSet;

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Convert to a vector of characters
    let characters: Vec<char> = input.chars().collect();

    // Process the input with a sliding window of 4 characters
    let n_unique_characters = 4;
    let mut starting_pos = n_unique_characters;
    for four_characters in characters.windows(n_unique_characters) {
        // Create a set from the characters in the window
        let set: HashSet<char> = four_characters.iter().cloned().collect();

        // Print the set
        // println!("{:?}", set);

        // 4 unique characters
        if set.len() == n_unique_characters {
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
    let n_unique_characters = 14;
    let mut starting_pos = n_unique_characters;
    for char_window in characters.windows(n_unique_characters) {
        // Create a set from the characters in the window
        let set: HashSet<&char> = char_window.iter().collect();

        // 14 unique characters
        if set.len() == n_unique_characters {
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
