use std::collections::HashMap;
use std::collections::HashSet;

type Point = (usize, usize);

fn is_accessible(current_height: u8, (x, y): Point, height_map: &[Vec<u8>]) -> bool {
    let neighbor_height = height_map[x][y];
    // neighbor is accessible if the height difference is at most 1
    // or lower
    (neighbor_height as i8 - current_height as i8) <= 1
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

fn part1() {
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

    // Dijkstra's algorithm
    // Note: we would need to use a priority queue for A*
    let mut queue = Vec::new();
    queue.push(start);

    let mut came_from: HashMap<Point, Option<Point>> = HashMap::new();
    let mut cost_so_far: HashMap<Point, u64> = HashMap::new();
    came_from.insert(start, None);
    cost_so_far.insert(start, 0);

    let mut visited_nodes: HashSet<Point> = HashSet::new();

    while !queue.is_empty() {
        let current = queue.pop().unwrap();

        // Do not do early stopping for finding the shortest path
        // if current == end {
        //     break;
        // }

        visited_nodes.insert(current);

        let neighbors_vec = neighbors(current, &height_map);
        if neighbors_vec.is_empty() {
            println!("Found a dead end at {:?}", current);
        }

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

    let total_nodes = height_map.len() * height_map[0].len();
    println!("Total nodes: {}", total_nodes);
    println!("Visited {} unique nodes", visited_nodes.len());

    // print the map with the visited nodes
    for (x, line) in height_map.iter().enumerate() {
        println!(
            "{}",
            line.iter()
                .enumerate()
                .map(|(y, c)| {
                    if visited_nodes.contains(&(x, y)) {
                        '*'
                    } else {
                        *c as char
                    }
                })
                .collect::<String>()
        );
    }

    // // Print the new map
    // for line in height_map {
    //     println!(
    //         "{}",
    //         line.iter()
    //             .map(|c| *c as char)
    //             .collect::<String>()
    //     );
    // }

    // Print the length of the shortest path
    println!("Part1: {}", cost_so_far.get(&end).unwrap());
    // Print the path
    // let mut current = end;
    // while let Some(next) = came_from.get(&current).unwrap() {
    //     let (x, y) = current;
    //     println!("{:?} {}", current, height_map[x][y] as char);
    //     current = *next;
    // }
}

fn main() {
    part1();
}
