fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut register_value: i32 = 1;
    let mut n_cycles: i32 = 0;
    let mut total_signal_strength: i32 = 0;

    for line in input.lines() {
        let (cycles, new_value) = match line {
            "noop" => (1, 0),
            _ => {
                let value = line.split_once(" ").unwrap().1.parse::<i32>().unwrap();
                (2, value)
            }
        };
        for _ in 0..cycles {
            n_cycles += 1;
            if (n_cycles + 20) % 40 == 0 {
                total_signal_strength += register_value * n_cycles;
            }
        }
        register_value += new_value;
        // println!("{}: {} + {} = {}", n_cycles, register_value - new_value, new_value, register_value);
    }
    println!("Part 1: {}", total_signal_strength);
}

fn main() {
    part1();
}
