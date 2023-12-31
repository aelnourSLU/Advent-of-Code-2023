use std::collections::HashMap;

use crate::util::file_reader;

pub fn start() {
    let (last, mut input) = file_reader("d05")
        .into_iter()
        .fold((Vec::new(), Vec::new()), |(mut current, mut overall), line| {
            if line == "" {
                overall.push(current.clone());
                current.clear();
            } else {
                current.push(line);
            }
            (current, overall)
        });
    input.push(last);
    let seeds = input
        .get(0)
        .unwrap()
        .get(0)
        .unwrap()
        .split_terminator(": ")
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    println!("start: {:?}", seeds);
    let mappers = input[1..]
        .into_iter()
        .map(|x| {
            x[1..]
                .into_iter()
                .map(|x| {
                    x.split_ascii_whitespace()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|v| Mapper::from(v))
        .collect::<Vec<_>>();

    let mut out = seeds
        .iter()
        .map(|x| {
            let mut x = *x;
            for mapper in &mappers {
                x = mapper.map(x);
            }
            x
        })
        .collect::<Vec<_>>();
    println!("end: {:?}", out);
    out.sort();
    println!("lowest: {}", out[0])
}

#[derive(Debug)]
struct Mapper {
    // matches: HashMap<usize, usize>
    tups: Vec<(usize, usize, usize)>
}

impl Mapper {
    fn from(vec: Vec<Vec<usize>>) -> Self {
        let mut nvec = Vec::new();
        for v in vec {
            nvec.push((v[0], v[1], v[2]));
        }
        Self {
            tups: nvec
        }
    }

    fn map(&self, target: usize) -> usize {
        for tup in &self.tups {
            if target - tup.1 < tup.2 {
                return tup.0 + target - tup.1
            }
        }
        target
    }
}
