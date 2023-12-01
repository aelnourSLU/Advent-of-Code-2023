use core::fmt;

use crate::util::file_reader;

pub fn start() {
    let input = file_reader("d01");
    let input = input
        .iter()
        .map(|x| {
            x.chars()
                .filter(|&c| !c.is_alphabetic())
                .collect::<Vec<_>>()
        })
        .into_iter()
        .fold(0_u32, |acc, e| {
            acc + format!("{}{}", e.first().unwrap(), e.last().unwrap()).parse::<u32>().unwrap()
        });
    println!("{}", input);

}