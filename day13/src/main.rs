fn compare(line1: &str, line2: &str) -> bool {
    let mut a = line1.chars();
    let mut b = line2.chars();
    let mut idx = 0;
    // println!("Comparing {} to {}", line1, line2);

    loop {
        idx += 1;

        match (a.next(), b.next()) {
            (Some('['), Some('[')) => continue,
            (Some(']'), Some(']')) => continue,
            // Comparing list to value
            (Some('['), Some(b)) if b.is_ascii_digit() => {
                // convert value to list
                // and concatenate the list to the rest of the string
                let new_string = format!("[{}]{}", b, &line2[idx..]);
                return compare(&line1[idx - 1..], &new_string);
            }
            (Some(a), Some('[')) if a.is_ascii_digit() => {
                // convert value to list
                // and concatenate the list to the rest of the string
                let new_string = format!("[{}]{}", a, &line1[idx..]);
                return compare(&new_string, &line2[idx - 1..]);
            }

            (Some(a), Some(b)) if a.is_ascii_digit()  && b.is_ascii_digit() => match (a.to_digit(10), b.to_digit(10)) {
                (Some(a), Some(b)) if a == b => continue,
                (Some(a), Some(b)) => return a < b,
                (None, None) => continue,
                _ => return false,
            },
            // right run out of values
            (Some(a), Some(']')) if a == '[' || a.is_ascii_digit() => return false,
            // left run out of values
            (Some(']'), Some(b)) if b == '[' || b.is_ascii_digit() => return true,
            // left run out of values
            (None, Some(_)) => return true,
            // right run out of values
            (Some(_), None) => return false,
            (None, None) => return true,
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
        }
    }
    println!("Total: {}", total);
}

fn main() {
    part1();
}
