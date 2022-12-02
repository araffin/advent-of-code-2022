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

fn part2() {}

fn main() {
    part1();
    part2();
}
