use std::collections::HashMap;

type Point = (usize, usize);

fn is_accessible(current_height: u8, (x, y): Point, height_map: &[Vec<u8>]) -> bool {
    let neighbor_height = height_map[x][y];
    // neighbor is accessible if the height difference is at most 1
    // or lower
    // (neighbor_height as i8 - current_height as i8) <= 1
    // Note we go from start to the end, so the condition is the opposite
    (current_height as i8 - neighbor_height as i8) <= 1
}

fn neighbors((x, y): Point, height_map: &Vec<Vec<u8>>) -> Vec<Point> {
    let mut neighbors = Vec::new();
    let current_height = height_map[x][y];

    if x > 0 && is_accessible(current_height, (x - 1, y), height_map) {
        neighbors.push((x - 1, y));
    }
    if x < height_map.len() - 1 && is_accessible(current_height, (x + 1, y), height_map) {
        neighbors.push((x + 1, y));
    }
    if y > 0 && is_accessible(current_height, (x, y - 1), height_map) {
        neighbors.push((x, y - 1));
    }
    if y < height_map[0].len() - 1 && is_accessible(current_height, (x, y + 1), height_map) {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn part1_and_2() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let height_map: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(line_idx, line)| {
            line.chars()
                .enumerate()
                // convert letter to number
                // note the offset doesn't matter
                .map(|(column, c)| match c {
                    'S' => {
                        start = (line_idx, column);
                        b'a'
                    }
                    'E' => {
                        end = (line_idx, column);
                        b'z'
                    }
                    _ => c as u8,
                })
                .collect()
        })
        .collect();

    // To make part2 easier
    // start from the end
    let (start, end) = (end, start);

    // Dijkstra's algorithm
    // Note: we would need to use a priority queue for A*
    let mut queue = Vec::new();
    queue.push(start);

    let mut came_from: HashMap<Point, Option<Point>> = HashMap::new();
    let mut cost_so_far: HashMap<Point, u64> = HashMap::new();
    came_from.insert(start, None);
    cost_so_far.insert(start, 0);

    while !queue.is_empty() {
        let current = queue.pop().unwrap();

        // Do not do early stopping for finding the shortest path
        // if current == end {
        //     break;
        // }

        for next in neighbors(current, &height_map) {
            let new_cost = cost_so_far.get(&current).unwrap() + 1;

            if cost_so_far.get(&next).unwrap_or(&std::u64::MAX) > &new_cost {
                cost_so_far.insert(next, new_cost);
                // we could try A* for the priority
                // let priority = new_cost + heuristic(next, end);
                queue.push(next);
                came_from.insert(next, Some(current));
            }
        }
    }

    // Print the length of the shortest path
    println!("Part1: {}", cost_so_far.get(&end).unwrap());

    // Post process: find the shortest path
    // to any lowest point (b'a' to end)
    let mut shortest_path = std::u64::MAX;
    for point in cost_so_far.keys() {
        if height_map[point.0][point.1] == b'a' {
            let path_length = cost_so_far.get(point).unwrap();
            if path_length < &shortest_path {
                shortest_path = *path_length;
            }
        }
    }
    println!("Part2: {}", shortest_path);
}

fn main() {
    part1_and_2();
}
