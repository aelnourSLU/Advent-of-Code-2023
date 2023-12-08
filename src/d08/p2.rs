use std::collections::{VecDeque, HashMap};

use crate::util::file_reader;

pub fn start() {
    let mut input = file_reader("d08").into_iter().collect::<VecDeque<_>>();
    let directions = input.pop_front().unwrap();
    input.pop_front();
    let nodes = input
        .into_iter()
        .map(|s| {
            let split = s.split_terminator(" = ").collect::<Vec<_>>();
            let name = split[0];
            let rest = split[1];
            let rest = rest
                .replace("(", "");
            let rest = rest
                .replace(")", "");
            let rest = rest
                .split_terminator(", ")
                .collect::<Vec<_>>();
            Node::from(name.to_string(), rest[0].to_string(), rest[1].to_string())
        })
        .collect::<Vec<_>>();

    let mut node_hm: HashMap<String, &Node> = HashMap::new();
    for node in &nodes {
        node_hm.insert(node.name.clone(), node);
    }

    let mut directions = directions
        .chars()
        .collect::<VecDeque<_>>();

    let mut curr_node_str_vec = nodes
        .iter()
        .filter(|n| n.ends_a())
        .map(|n| n.name.clone())
        .collect::<Vec<_>>();
 
    let end_z = |s: &str| { s.chars().last().unwrap() == 'Z' };
    let results = curr_node_str_vec
        .iter()
        .map(|node| {
            let mut count = 0;
            let mut curr_node_str = node.to_owned();
            while !end_z(curr_node_str.as_str()) {
                let next_dir = directions.pop_front().unwrap();
                directions.push_back(next_dir.to_owned());
                let curr_node = node_hm.get(&curr_node_str).unwrap();
                match next_dir {
                    'L' => curr_node_str = curr_node.left.clone(),
                    'R' => curr_node_str = curr_node.right.clone(),
                    _ => panic!()
                }
                // println!("Going {} to {}", next_dir, curr_node_str);
                count += 1;
            }
            count
        })
        .collect::<Vec<_>>();
    let highest = results.iter().max().unwrap();
    let lcm = results
        .iter()
        .map(|x| *x as usize)
        .reduce(|acc, e| lcm(acc, e))
        .unwrap();
    // println!("Count: {}", count);
    println!("Out: {:?}", highest);
    println!("lcm: {}", lcm);
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let buffer = a;
    let a_og = a;
    let b_og = b;
    let a = if b > a {
        b
    } else {
        a
    };
    
    let b = if b_og > a_og {
        buffer
    } else {
        b
    };
    
    let d = a % b;
    if d == 0 {
        b
    } else {
        gcd(b, d)
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String
}

impl Node {
    fn from(name: String, left: String, right: String) -> Self {
        Self { name, left, right }
    }

    fn ends_a(&self) -> bool {
        self.name.chars().last().unwrap() == 'A'
    }

    fn ends_z(&self) -> bool {
        self.name.chars().last().unwrap() == 'Z'
    }
}

// not 1917116283508457903
