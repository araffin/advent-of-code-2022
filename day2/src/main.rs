fn convert_to_score(string: &str) -> i32 {
    match string {
        "A" | "X" => 1, // rock
        "B" | "Y" => 2, // paper
        "C" | "Z" => 3, // scissors
        _ => panic!("Invalid input"),
    }
}

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut total_score = 0;

    for line in input.lines() {
        let (player1, player2) = line.split_once(" ").unwrap();
        let score = match (convert_to_score(player1), convert_to_score(player2)) {
            // Player 1 wins
            (1, p2 @ 3) | (2, p2 @ 1) | (3, p2 @ 2) => p2,
            // Player 2 wins
            (3, p2 @ 1) | (1, p2 @ 2) | (2, p2 @ 3) => p2 + 6,
            // Draw
            (p1, p2) if p1 == p2 => p2 + 3,
            _ => panic!("Invalid input"),
        };
        total_score += score;
    }
    println!("Total score: {}", total_score);
}

fn part2() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut total_score = 0;

    for line in input.lines() {
        let (player1, player2) = line.split_once(" ").unwrap();

        let score = match (convert_to_score(player1), player2) {
            // We need to lose
            (player1, "X") => {
                match player1 {
                    // rock against paper
                    2 => 1,
                    // paper against scissors
                    3 => 2,
                    // scissors against rock
                    1 => 3,
                    _ => panic!("Invalid input"),
                }
            }
            // We need to end with a draw
            (player1, "Y") => {
                // We play the same as player 1
                player1 + 3
            }
            // We need to win
            (player1, "Z") => {
                let player2 = match player1 {
                    // paper against rock
                    1 => 2,
                    // scissors against paper
                    2 => 3,
                    // rock against scissors
                    3 => 1,
                    _ => panic!("Invalid input"),
                };
                player2 + 6
            }

            _ => panic!("Invalid input"),
        };

        total_score += score;
    }
    println!("Total score part2: {}", total_score);
}

fn main() {
    part1();
    part2();
}
