use crate::util::file_reader;

pub fn start() {
    let input = file_reader("d05")
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
    let input = input.1;
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
    let maps = input[1..]
        .into_iter()
        .map(|x| {
            x[1..]
                .into_iter()
                .map(|x| {
                    x.split_ascii_whitespace()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|v| Mapper::from(v[0], v[1], v[2]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let out = seeds
        .iter()
        .map(|x| {
            let mut x = *x;
            for map_group in &maps {
                for map in map_group {
                    if map.can_map(x) {
                        x = map.map(x);
                    }
                }
            }
            x
        })
        .collect::<Vec<_>>();
    println!("end: {:?}", out);
}

#[derive(Debug)]
struct Mapper {
    dest: usize,
    source: usize,
    range: usize
}

impl Mapper {
    fn from(dest: usize, source: usize, range: usize) -> Self {
        Self {
            dest, source, range
        }
    }

    fn map(&self, target: usize) -> usize {
        if target > self.source && target <= self.source + self.range - 1 {
            let mv = target - self.source;
            self.dest + mv
        } else {
            target
        }
    }

    fn can_map(&self, target: usize) -> bool {
        if target > self.source && target <= self.source + self.range - 1 {

            true
        } else {
            false
        }
    }
}