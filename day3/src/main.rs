use std::collections::HashSet;

fn letter_position(letter: char) -> u8 {
    letter as u8 - ('a' as u8) + 1
}

fn get_priority(letter: char) -> u8 {
    // Convert letter to lowercase
    letter_position(letter.to_ascii_lowercase()) + 26 * (letter.is_uppercase() as u8)
}

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut total: u32 = 0;

    for line in input.lines() {
        // Get the length of the line
        let length = line.len();
        // Create a set with characters from the first half
        let first_half: HashSet<char> = line[..length / 2].chars().collect();
        // Create a set with characters from the second half
        let second_half: HashSet<char> = line[length / 2..].chars().collect();
        // Get the intersection of the two sets
        let intersection: HashSet<_> = first_half.intersection(&second_half).collect();
        // There should be only one character in the intersection
        assert_eq!(intersection.len(), 1);
        // Retrieve the character from the intersection
        let letter = intersection.iter().next().unwrap().clone();
        total += get_priority(*letter) as u32;
    }
    // Print the total
    println!("{}", total);
}

fn part2() {}

fn main() {
    part1();
    part2();
}
