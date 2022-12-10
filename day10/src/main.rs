fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut register_value: i32 = 1;
    let mut n_cycles: i32 = 0;
    let mut total_signal_strength: i32 = 0;

    for line in input.lines() {
        let (cycles, register_offset) = match line {
            "noop" => (1, 0),
            _ => {
                let value = line.split_once(' ').unwrap().1.parse::<i32>().unwrap();
                (2, value)
            }
        };
        for _ in 0..cycles {
            n_cycles += 1;
            if (n_cycles + 20) % 40 == 0 {
                total_signal_strength += register_value * n_cycles;
            }
        }
        register_value += register_offset;
        // println!("{}: {} + {} = {}", n_cycles, register_value - new_value, new_value, register_value);
    }
    println!("Part 1: {}", total_signal_strength);
}

fn part2() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut register_value: i32 = 1;
    let mut n_cycles: i32 = 0;
    let sprite = "###.....................................";
    let line_length = 40;

    for line in input.lines() {
        let (cycles, register_offset) = match line {
            "noop" => (1, 0),
            _ => {
                let value = line.split_once(' ').unwrap().1.parse::<i32>().unwrap();
                (2, value)
            }
        };
        for _ in 0..cycles {
            // cursor position
            let cursor_position = n_cycles % line_length;
            let offset = register_value - 1;
            // use modulo to wrap around (-3 % 40 = 37)
            let sprite_idx = ((cursor_position - offset % line_length) + line_length) % line_length;
            let current_pixel = sprite.chars().nth(sprite_idx as usize).unwrap();

            print!("{}", current_pixel);

            n_cycles += 1;

            if n_cycles % line_length == 0 {
                println!()
            }
        }
        register_value += register_offset;
    }
}

fn main() {
    part1();
    part2();
}
