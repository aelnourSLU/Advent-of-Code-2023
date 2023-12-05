use std::cmp;

use crate::util::file_reader;

pub fn start() {
    let input = file_reader("d02");
    let out: usize = input.into_iter()
        .map(|x| {
            let mut split = x.split_terminator(":");
            let game = split.next().unwrap();
            let rest = split.next().unwrap();

            let _game = game
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            // index 0 red, 1 green, 2 blue
            let mut col_counts: [usize; 3] = [ 0, 0, 0 ];

            let rest = rest.split_terminator(";");
            let _valid = rest.into_iter()
                .map(|x| {
                    let x = x.trim();

                    let color_split = x.split_terminator(",")
                        .into_iter()
                        .map(|x| {
                            let mut str_split = x.split_ascii_whitespace();
                            let num = str_split
                                .next()
                                .unwrap()
                                .parse::<usize>()
                                .unwrap();
                            let color = str_split.next().unwrap();

                            let col_valid = if color == "red" {
                                col_counts[0] = cmp::max(col_counts[0], num);
                                num <= 12
                            } else if color == "green" {
                                col_counts[1] = cmp::max(col_counts[1], num);
                                num <= 13
                            } else if color == "blue" {
                                col_counts[2] = cmp::max(col_counts[2], num);
                                num <= 14
                            } else {
                                false
                            };
                            col_valid
                        })
                        .fold(true, |acc, e| acc && e);
                    color_split
                })
                .fold(true, |acc, e| {
                    acc && e
                });
            
            // (game, valid, col_counts.into_iter().product::<usize>())
            col_counts.into_iter().product::<usize>()
        })
        .sum();
    println!("{}", out);
}