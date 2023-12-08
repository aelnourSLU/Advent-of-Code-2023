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

    /* let mut curr_node_str = "AAA";
    let mut count = 0;
    while curr_node_str != "ZZZ" {
        let next_dir = directions.pop_front().unwrap();
        directions.push_back(next_dir.to_owned());
        let curr_node = node_hm.get(curr_node_str).unwrap();
        match next_dir {
            'L' => curr_node_str = curr_node.left.as_str(),
            'R' => curr_node_str = curr_node.right.as_str(),
            _ => panic!()
        }
        println!("Going {} to {}", next_dir, curr_node_str);
        count += 1;
    } */

    let mut count = 0;
    let mut curr_node_str_vec = nodes
        .iter()
        .filter(|n| n.ends_a())
        .map(|n| n.name.clone())
        .collect::<Vec<_>>();
    
    let all_end_z = |v: &Vec<String>, start: usize| { 
        start == v.iter()
            .filter(|&s| s.chars().last().unwrap() == 'Z')
            .collect::<Vec<_>>()
            .len()
    };
    let num_start_nodes = curr_node_str_vec.len();

    while !all_end_z(&curr_node_str_vec, num_start_nodes) {
        let next_dir = directions.pop_front().unwrap();
        directions.push_back(next_dir.to_owned());
        let mut new_nodes = Vec::new();
        for node in curr_node_str_vec {
            let curr_node = node_hm.get(&node).unwrap();
            let to_node;
            match next_dir {
                'L' => to_node = curr_node.left.clone(),
                'R' => to_node = curr_node.right.clone(),
                _ => panic!()
            }
            //println!("Going {} from {} to {}", next_dir, curr_node.name, to_node);
            new_nodes.push(to_node);
        }
        //println!("==");
        curr_node_str_vec = new_nodes;
        count += 1;
    }

    println!("Count: {}", count);
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