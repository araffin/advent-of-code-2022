// Node to create a graph that will represent the file system
// do not warn about dead code
#![allow(dead_code)]
#[derive(Debug)]
struct Node {
    name: String,
    value: u64,
    children: Vec<usize>,
    parent: Option<usize>,
}

fn part1() {
    // Parse the input
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut all_nodes: Vec<Node> = vec![];

    // Create a root node
    let root = Node {
        name: String::from("/"),
        children: Vec::new(),
        parent: None,
        value: 0,
    };
    all_nodes.push(root);
    
    let mut current_node = 0;

    for line in input.lines() {
        let (start, rest) = line.split_once(" ").unwrap();
        match start {
            "$" if rest == "ls" => continue,
            "$" if rest != "ls" => {
                // cd command: cd folder_name
                let (_, folder_name) = rest.split_once(" ").unwrap();
                match folder_name {
                    "/" => continue,
                    ".." => {
                        // Go to the parent node
                        current_node = all_nodes.get(current_node).unwrap().parent.unwrap();
                        continue
                    },
                    _ => {
                        let new_node_id = all_nodes.len();
                        // Create a folder node and add it to the parent
                        let new_node = Node {
                            name: String::from(folder_name),
                            children: Vec::new(),
                            parent: Some(current_node),
                            value: 0,
                        };
                        all_nodes.push(new_node);
                        all_nodes.get_mut(current_node).unwrap().children.push(new_node_id);
                        current_node = all_nodes.len() - 1;
                    },
                }
            }
            "dir" => {
                continue
                // let folder_name = rest;
                // println!("{} {}", start, folder_name);
            },
            _ => {
                let name = rest;
                let filesize = start.parse::<u64>().unwrap();
                // println!("{} {}", name, filesize);

                let new_node_id = all_nodes.len();
                // Add a leaf node to the current node
                let new_node = Node {
                    name: String::from(name),
                    children: Vec::new(),
                    parent: Some(current_node),
                    value: filesize,
                };
                all_nodes.push(new_node);
                all_nodes.get_mut(current_node).unwrap().children.push(new_node_id);

            },
        }
    }

    // println!("{:?}", all_nodes);

    // Calculate the value of each node
    // it is the sum of the value of each node
    // we need to go at the bottom of the tree first
    // propagate the value up

    let mut stack: Vec<usize> = vec![];
    stack.push(0);
    while !stack.is_empty() {
        let node_id = stack.pop().unwrap();
        let node = all_nodes.get(node_id).unwrap();
        if node.children.is_empty() {
            // This is a leaf node
            // We can propagate the value up
            let mut current_node = all_nodes.get(node_id).unwrap();
            let value = current_node.value;
            while let Some(parent_id) = current_node.parent {
                let parent_node = all_nodes.get_mut(parent_id).unwrap();
                parent_node.value += value;
                current_node = all_nodes.get(parent_id).unwrap();
            }
        } else {
            // This is not a leaf node
            // We need to go deeper
            for child_id in node.children.iter() {
                stack.push(*child_id);
            }
        }
    }

    
    // Sum all nodes that have a value below 100000 and that have children
    let mut sum = 0;
    for node in all_nodes.iter() {
        if node.value < 100000 && !node.children.is_empty() {
            // println!("{} {}", node.name, node.value);
            sum += node.value;
        }
    }

    println!("Part 1: {}", sum);

}

fn main() {
    part1();
}
