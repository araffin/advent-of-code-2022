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

fn part2() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut total: u32 = 0;

    // Loop three lines per three lines
    for (line_num, current_line) in input.lines().enumerate().step_by(3) {
        // Get the next two lines
        let line_two = input.lines().nth(line_num + 1).unwrap();
        let line_three = input.lines().nth(line_num + 2).unwrap();

        // Create a set with characters for each line
        let firt_line_set: HashSet<char> = current_line.chars().collect();
        let second_line_set: HashSet<char> = line_two.chars().collect();
        let third_line_set: HashSet<char> = line_three.chars().collect();

        // Get the intersection of the three sets
        let intersection: HashSet<&char> = firt_line_set.intersection(&second_line_set).collect();
        // Convert the intersection from HashSet<&char> to HashSet<char>
        let intersection_owned: HashSet<char> = intersection.iter().map(|&x| x.clone()).collect();
        // Take the intersection of the previous intersection and the third set
        let final_intersection: HashSet<_> =
            intersection_owned.intersection(&third_line_set).collect();

        // There should be only one character in the intersection
        assert_eq!(final_intersection.len(), 1);
        // Retrieve the character from the intersection
        let letter = final_intersection.iter().next().unwrap().clone();
        total += get_priority(*letter) as u32;
    }

    // Print the total
    println!("{}", total);
}

fn main() {
    part1();
    part2();
}
