fn compare_digits(line1: &str, line2: &str, idx: usize) -> Option<bool> {
    let next_character_1 = line1.chars().clone().nth(idx);
    let next_character_2 = line2.chars().clone().nth(idx);

    // Special case to deal with 10
    match (next_character_1, next_character_2) {
        (Some(a), Some(b)) if a.is_ascii_digit() ^ b.is_ascii_digit() => {
            return Some(b.is_ascii_digit())
        }
        _ => None,
    }
}

fn compare(line1: &str, line2: &str) -> bool {
    let mut line1_chars = line1.chars();
    let mut line2_chars = line2.chars();
    let mut idx: usize = 0;
    // println!("Comparing {} to {}", line1, line2);

    loop {
        idx += 1;

        // println!("Comparing {} to {}", &line1[idx..], &line2[idx..]);

        match (line1_chars.next(), line2_chars.next()) {
            // Comparing list to value
            (Some('['), Some(b)) if b.is_ascii_digit() => {
                // convert value to list
                // and concatenate the list to the rest of the string
                let next_character = line2.chars().clone().nth(idx);
                if next_character == Some('0') {
                    let new_string = format!("[{}0]{}", b, &line2[idx + 1..]);
                    return compare(&line1[idx - 1..], &new_string);
                }

                let new_string = format!("[{}]{}", b, &line2[idx..]);
                return compare(&line1[idx - 1..], &new_string);
            }
            (Some(a), Some('[')) if a.is_ascii_digit() => {
                // convert value to list
                // and concatenate the list to the rest of the string
                let next_character = line1.chars().clone().nth(idx);
                if next_character == Some('0') {
                    let new_string = format!("[{}0]{}", a, &line1[idx + 1..]);
                    return compare(&new_string, &line2[idx - 1..]);
                }

                let new_string = format!("[{}]{}", a, &line1[idx..]);
                return compare(&new_string, &line2[idx - 1..]);
            }

            (Some(a), Some(b)) if a.is_ascii_digit() && b.is_ascii_digit() => {
                match (a.to_digit(10).unwrap(), b.to_digit(10).unwrap()) {
                    (a_num @ 1, b_num) | (a_num, b_num @ 1) => {
                        match compare_digits(line1, line2, idx) {
                            Some(result) => return result,
                            None => {
                                if a_num == b_num {
                                    continue;
                                } else {
                                    return a_num < b_num;
                                }
                            }
                        }
                    }
                    (a_num, b_num) if a_num == b_num => match compare_digits(line1, line2, idx) {
                        Some(result) => return result,
                        None => continue,
                    },
                    (a_num, b_num) => {
                        match compare_digits(line1, line2, idx) {
                            Some(result) => return result,
                            None => (),
                        }
                        return a_num < b_num;
                    }
                }
            }
            // right run out of values
            (Some(a), Some(']')) if a == '[' || a.is_ascii_digit() => return false,
            (Some(a), Some('[')) if a.is_ascii_digit() => return false,
            (Some(a), Some(',')) if a.is_ascii_digit() => return false,

            // left run out of values
            (Some(']'), Some(b)) if b == '[' || b.is_ascii_digit() => return true,
            (Some(']'), Some(b)) if b.is_ascii_digit() => return true,
            (Some(','), Some(b)) if b.is_ascii_digit() => return true,

            // left run out of values
            (None, Some(_)) => return true,
            // right run out of values
            (Some(_), None) => return false,
            (None, None) => panic!("Invalid input"),
            // (Some('['), Some('[')) => continue,
            // (Some(']'), Some(']')) => continue,
            _ => continue,
        }
    }
}

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut total = 0;

    for (pair_idx, lines) in input.split("\n\n").enumerate() {
        let line1 = lines.split('\n').next().unwrap();
        let line2 = lines.split('\n').nth(1).unwrap();
        let in_order = compare(line1, line2);
        // println!("Pair {} in order: {}", pair_idx + 1, in_order);

        if in_order {
            total += pair_idx + 1;
            println!("Pair {} in order: {}", pair_idx + 1, in_order);
        }
    }
    println!("Total: {}", total);
    panic!("Careful, this code is buggy, shoudln't solve it character by character")
}

fn main() {
    part1();
}
