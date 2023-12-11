use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

advent_of_code::solution!(8);

struct Node<'a> {
    value: &'a str,
    left: Option<Rc<RefCell<Node<'a>>>>,
    right: Option<Rc<RefCell<Node<'a>>>>,
}

impl<'a> Node<'a> {
    fn new(value: &'a str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            left: None,
            right: None,
        }))
    }

    fn set_left(&mut self, node: Rc<RefCell<Node<'a>>>) {
        self.left = Some(node);
    }

    fn set_right(&mut self, node: Rc<RefCell<Node<'a>>>) {
        self.right = Some(node);
    }
}

struct Graph<'a> {
    root: Option<Rc<RefCell<Node<'a>>>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph { root: None }
    }

    fn set_root(&mut self, node: Rc<RefCell<Node<'a>>>) {
        self.root = Some(node);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn parse_input(input: &str) -> (Vec<char>, Graph<'_>, HashMap<String, Rc<RefCell<Node<'_>>>>) {
    let mut lines = input.lines();
    let directions: Vec<char> = lines.next().unwrap_or("").chars().collect();

    let mut graph = Graph::new();
    let mut nodes = HashMap::new();

    // First, ensure all nodes are created and inserted into the HashMap
    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        if parts.len() == 2 {
            let node_name = parts[0].trim();
            nodes
                .entry(node_name.to_string())
                .or_insert_with(|| Node::new(node_name));
        }
    }

    // Then, set up the connections
    for line in input.lines().skip(1) {
        // skip the directions line
        let parts: Vec<&str> = line.split(" = ").collect();
        if parts.len() == 2 {
            let node_name = parts[0].trim();
            let connections: Vec<&str> = parts[1]
                .trim_matches(|patch| patch == '(' || patch == ')')
                .split(",")
                .map(str::trim)
                .collect();

            if let Some(current_node) = nodes.get(node_name) {
                for (i, connection_name) in connections.iter().enumerate() {
                    if let Some(connected_node) = nodes.get(*connection_name) {
                        if i == 0 {
                            current_node.borrow_mut().set_left(connected_node.clone());
                        } else {
                            current_node.borrow_mut().set_right(connected_node.clone());
                        }
                    }
                }
            }
        }
    }

    // Set the root node
    if let Some(root) = nodes.get("AAA") {
        graph.set_root(root.clone());
    }

    (directions, graph, nodes)
}

pub fn part_one(_input: &str) -> Option<u32> {
    let (directions, graph, _) = parse_input(_input);
    let root_node = graph.root.clone();
    let mut current_node = graph.root.clone();
    let mut directions_index = 0;
    let mut steps = 0;

    while let Some(node) = &current_node {
        // traverse the graph according to the directions infinitely until we find the destination node "ZZZ"
        if node.borrow().value == "ZZZ" {
            return Some(steps);
        }

        steps += 1;
        let next_node = if directions[directions_index] == 'L' {
            node.borrow().left.clone()
        } else {
            node.borrow().right.clone()
        };
        current_node = next_node.or_else(|| root_node.clone());
        directions_index = (directions_index + 1) % directions.len();
    }

    Some(steps)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let (directions, mut graph, nodes) = parse_input(_input);
    let mut cycle_lengths = Vec::new();

    for (name, node) in &nodes {
        if name.ends_with('A') {
            graph.set_root(node.clone());
            let mut current_node = graph.root.clone();
            let mut directions_index = 0;
            let mut steps = 0;

            while let Some(node_rc) = &current_node {
                let next_node;
                let is_end_with_z;

                {
                    let node = node_rc.borrow();
                    // Check if the node ends with 'Z'
                    is_end_with_z = node.value.ends_with('Z');

                    next_node = if directions[directions_index] == 'L' {
                        node.left.clone()
                    } else {
                        node.right.clone()
                    };
                } // The borrow ends here

                if is_end_with_z {
                    cycle_lengths.push(steps);
                    break;
                }

                steps += 1;
                current_node = next_node.or_else(|| graph.root.clone());
                directions_index = (directions_index + 1) % directions.len();
            }
        }
    }

    println!("Cycle Lengths: {:?}", cycle_lengths);
    if cycle_lengths.is_empty() {
        None
    } else {
        let mut result_lcm = cycle_lengths[0];
        for &length in &cycle_lengths[1..] {
            result_lcm = lcm(result_lcm, length);
        }

        println!("LCM: {}", result_lcm);
        Some(result_lcm as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
