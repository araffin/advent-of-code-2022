fn is_visible(trees: &Vec<Vec<u64>>, x: usize, y: usize) -> bool {
    let width = trees[0].len();
    let height = trees.len();
    let tree_height = trees[x][y];

    // check if there is a tree higher than the current one
    // in the same row or column
    let left = trees[x][0..y].iter().all(|&value| value < tree_height);
    let right = trees[x][y + 1..width]
        .iter()
        .all(|&value| value < tree_height);

    // note: 0..x is exclusive (x is not included)
    let top = trees[0..x].iter().all(|row| row[y] < tree_height);

    let bottom = trees[x + 1..height].iter().all(|row| row[y] < tree_height);

    left || right || top || bottom
}

fn compute_score(trees: &Vec<Vec<u64>>, x: usize, y: usize) -> usize {
    let width = trees[0].len();
    let height = trees.len();
    let tree_height = trees[x][y];

    // check how many trees are below its height
    // we need to take the tree until it is higher than the current one
    // take while does not include the last element
    // so we have to use for loops...
    let mut n_left = 0;
    for i in (0..y).rev() {
        n_left += 1;
        if trees[x][i] >= tree_height {
            break;
        }
    }

    let mut n_right = 0;
    for i in (y + 1)..width {
        n_right += 1;
        if trees[x][i] >= tree_height {
            break;
        }
    }

    let mut n_top = 0;
    for i in (0..x).rev() {
        n_top += 1;
        if trees[i][y] >= tree_height {
            break;
        }
    }

    let mut n_bottom = 0;
    for i in (x + 1)..height {
        n_bottom += 1;
        if trees[i][y] >= tree_height {
            break;
        }
    }

    n_left * n_right * n_top * n_bottom
}

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    // create the tree map
    let trees: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    let width = trees[0].len();
    let height = trees.len();
    let mut n_visible: usize = 0;

    // All the outer edges are visible
    n_visible += width * 2 + height * 2 - 4;

    // check if trees are visible
    for i in 1..(height - 1) {
        for j in 1..(width - 1) {
            if is_visible(&trees, i, j) {
                n_visible += 1;
            }
        }
    }

    println!("Part1 {:}", n_visible);
}

fn part2() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    // create the tree map
    let trees: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    let width = trees[0].len();
    let height = trees.len();

    let max_score = trees
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| {
                    if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
                        0
                    } else {
                        compute_score(&trees, i, j)
                    }
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Part2: {:}", max_score);
}

fn main() {
    part1();
    part2();
}
