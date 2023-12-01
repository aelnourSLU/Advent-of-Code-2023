use core::fmt;
use std::ops::SubAssign;

use crate::util::file_reader;

pub fn start() {
    let both = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let input = file_reader("d01");
    let input = input
        .into_iter()
        .map(|s| {
            let botha = vec![
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                "1", "2", "3", "4", "5", "6", "7", "8", "9"
            ];
            let mut all = botha.into_iter()
                .map(|x| s.match_indices(x))
                .flatten()
                .collect::<Vec<_>>();
            all.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            let mut a = all.first().unwrap().1;
            let mut b = all.last().unwrap().1;
            for (s1, s2) in &both {
                if a == *s1 { a = s2 }
                if b == *s1 { b = s2 }
            }
            (a.to_string() + b).parse::<u32>().unwrap()
        })
        .reduce(|acc, e| acc + e).unwrap();
    println!("{:?}", input);
}
