use crate::util::file_reader;

pub fn start() {
    let input = file_reader("d02");
    let out = input.into_iter()
        .map(|x| {
            let mut split = x.split_terminator(":");
            let game = split.next().unwrap();
            let rest = split.next().unwrap();

            let game = game
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let rest = rest.split_terminator(";");
            let valid = rest.into_iter()
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

                            if color == "red" {
                                num <= 12
                            } else if color == "green" {
                                num <= 13
                            } else if color == "blue" {
                                num <= 14
                            } else {
                                false
                            }
                        })
                        .fold(true, |acc, e| acc && e);
                    color_split
                })
                .fold(true, |acc, e| {
                    acc && e
                });
            (game, valid)
        })
        .filter(|&(_, valid)| valid)
        .fold(0, |acc, (e, _)| {
            acc + e
        });
    println!("{}", out);
}