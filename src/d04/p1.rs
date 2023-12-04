use crate::util::file_reader;

pub fn start() {
    let input = file_reader("d04");
    let output: usize = input.into_iter()
        .map(|s| {
            let a = s.split_terminator(":");
            let b = a.last().unwrap();
            let mut c = b.split_terminator(" | ");
            let winners = c.next().unwrap().trim_start();
            let winners = winners
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let have = c.next().unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();;
            let mut count = 0;
            for h in &have {
                for w in &winners {
                    if h == w {
                        count += 1;
                    }
                }
            }
            let mut out = 0;
            if count >= 1 { out = 1}
            for i in 1..count {
                out *= 2;
            }
            out
        })  
        .sum();
    println!("{}", output)
}