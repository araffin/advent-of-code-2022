use std::collections::HashSet;

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Convert to a vector of characters
    let characters: Vec<char> = input.chars().collect();
    // let inter = tst.chars().collect::<Vec<char>>();

    // Process the input with a sliding window of 4 characters
    // set by 4 characters

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

fn main() {
    part1();
    // part2();
}
