use core::num;

use regex::Regex;

use crate::util::file_reader;

fn adj(x: isize, width: usize, height: usize) -> Vec<usize> {
    let width = width as isize;
    let height = height as isize;
    vec![
        x + width,
        x + width + 1,
        x + width - 1,
        x - width,
        x - width + 1,
        x - width - 1,
        x + 1,
        x - 1
    ].into_iter()
        .filter(|&x| x >= 0 && x <= width * height)
        .map(|x| x as usize)
        .collect::<Vec<_>>()
}

pub fn start() {
    let input = file_reader("d03");
    let width = input[0].len();
    let height = input.len();
    let input = input
        .into_iter()
        .reduce(|acc, e| acc + e.as_str())
        .unwrap();
    let re = Regex::new(r"(\d+)").unwrap();
    let num_caps = re.captures_iter(&input);
    let mut nums = Vec::new();
    for cap in num_caps {
        let (_, [thing]) = cap.extract();
        let thing = thing.parse::<usize>().unwrap();
        let start = cap.get(1).unwrap().start() as isize;
        let end = cap.get(1).unwrap().end() as isize;
        nums.push((thing, start, end));
    }
    let re = Regex::new(r"(\*)").unwrap();
    let sym_caps = re.captures_iter(&input);
    let mut syms = Vec::new();
    for cap in sym_caps {
        let (_, [_]) = cap.extract();
        let pos = cap.get(1).unwrap().start() as isize;
        syms.push(pos);
    }
    let mut sum: usize = 0;
    for pos in &syms {
        let mut tempvec = Vec::new();
        for (num, start, end) in &nums {
            for p in adj(*pos, width, height) {
                if p >= *start as usize && p < *end as usize {
                    if !tempvec.contains(num) {
                        tempvec.push(*num);
                    }
                }
            }
        }
        if tempvec.len() == 2 {
            sum += tempvec.iter().product::<usize>();
        }
    }
    println!("{:?}", sum);
}
