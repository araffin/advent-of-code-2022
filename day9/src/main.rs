use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut visited_points: HashSet<Point> = HashSet::new();

    let starting_point = Point { x: 0, y: 0 };
    let mut tail_position = starting_point.clone();
    let mut head_position = starting_point.clone();

    visited_points.insert(tail_position.clone());

    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();
        let steps = steps.parse::<i32>().unwrap();

        // println!("{} {} ", direction, steps);

        for _ in 0..steps {
            // Update head position
            match direction {
                "D" => head_position.y -= 1,
                "U" => head_position.y += 1,
                "L" => head_position.x -= 1,
                "R" => head_position.x += 1,
                _ => panic!("Unknown direction"),
            }
            // Update tail position
            // tail and head must be always touching
            let dx = head_position.x - tail_position.x;
            let dy = head_position.y - tail_position.y;

            // if the tail is not touching the head
            // and aren't in the same row/column
            // move the tail diagonally
            if tail_position.x != head_position.x && tail_position.y != head_position.y {
                if dx.abs() > 1 || dy.abs() > 1 {
                    tail_position.x += dx.signum();
                    tail_position.y += dy.signum();
                }
            } else {
                if dx.abs() > 1 {
                    tail_position.x += dx.signum();
                }
                if dy.abs() > 1 {
                    tail_position.y += dy.signum();
                }
            }

            // print new positions
            // println!("head: {} {}", head_position.x, head_position.y);
            // println!("tail: {} {}", tail_position.x, tail_position.y);

            visited_points.insert(tail_position.clone());
        }
        // println!("");
    }

    println!("Part1 {:}", visited_points.len());
}

fn part2() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut visited_points: HashSet<Point> = HashSet::new();

    let starting_point = Point { x: 0, y: 0 };

    let mut rope: Vec<Point> = vec![];
    let n_points = 10;
    // fill the vector
    // the last element is the tail
    for _ in 0..n_points {
        rope.push(starting_point.clone());
    }

    visited_points.insert(rope[n_points - 1].clone());

    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();
        let steps = steps.parse::<i32>().unwrap();

        // println!("{} {} ", direction, steps);

        for _ in 0..steps {
            // Update head position
            let mut head_position = rope.get_mut(0).unwrap();
            match direction {
                "D" => head_position.y -= 1,
                "U" => head_position.y += 1,
                "L" => head_position.x -= 1,
                "R" => head_position.x += 1,
                _ => panic!("Unknown direction"),
            }

            // Same update as part1
            // but for all points in the rope

            for i in 0..n_points - 1 {
                let head_position = rope[i].clone();
                let mut tail_position = rope.get_mut(i + 1).unwrap();
                let dx = head_position.x - tail_position.x;
                let dy = head_position.y - tail_position.y;

                if tail_position.x != head_position.x && tail_position.y != head_position.y {
                    if dx.abs() > 1 || dy.abs() > 1 {
                        tail_position.x += dx.signum();
                        tail_position.y += dy.signum();
                    }
                } else {
                    if dx.abs() > 1 {
                        tail_position.x += dx.signum();
                    }
                    if dy.abs() > 1 {
                        tail_position.y += dy.signum();
                    }
                }
            }

            // print tail position
            // println!("head: {} {}", rope[0].x, rope[0].y);
            // println!("tail: {} {}", rope[n_points - 1].x, rope[n_points - 1].y);

            // get the last element of the rope
            // and add it to the visited points
            visited_points.insert(rope[n_points - 1].clone());
        }
        // println!("");
    }

    println!("Part2 {:}", visited_points.len());
}

fn main() {
    part1();
    part2();
}
