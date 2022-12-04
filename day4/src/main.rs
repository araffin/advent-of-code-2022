fn is_contained(low1: u32, high1: u32, low2: u32, high2: u32) -> bool {
    (low1 >= low2 && high1 <= high2) || (low2 >= low1 && high2 <= high1)
}

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut total: u32 = 0;

    for line in input.lines() {
        let (first_elf, second_elf) = line.split_once(',').unwrap();
        let (low1, high1) = first_elf.split_once('-').unwrap();
        let (low2, high2) = second_elf.split_once('-').unwrap();
        // Parse the numbers into a int tuple
        let (low1, high1, low2, high2) = (
            low1.parse::<u32>().unwrap(),
            high1.parse::<u32>().unwrap(),
            low2.parse::<u32>().unwrap(),
            high2.parse::<u32>().unwrap(),
        );
        total += is_contained(low1, high1, low2, high2) as u32;
    }
    // Print the total
    println!("{}", total);
}

fn part2() {}

fn main() {
    part1();
    part2();
}
