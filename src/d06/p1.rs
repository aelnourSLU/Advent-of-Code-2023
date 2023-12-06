use crate::util::file_reader;

pub fn start() {
    let input = file_reader("d06");
    let input = input
        .into_iter()
        .map(|s| {
            let split = s.split_terminator(": ");
            let second = split.last();
            let second = second
                .into_iter()
                .map(|s| {
                    s.split_ascii_whitespace()
                        .map(|s| s.trim().parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>();
            second
        })
        .collect::<Vec<_>>();
    println!("input: {:?}", input);
    let mut out = Vec::new();
    for i in 0..=input.len() + 1 {
        let time = input[0][i];
        let distance = input[1][i];
        println!("Operating on time {}, distance {}", time, distance);
        let mut hold = Vec::new();
        for hold_time in 0..=time {
            let ntime = time - hold_time;
            let travel = hold_time * ntime;
            hold.push(travel);
        }
        let hold = hold
            .into_iter()
            .filter(|&x| x > distance)
            .collect::<Vec<_>>();
        println!("hold: {}", hold.len());
        out.push(hold.len())
    }
    println!("{:?}", input);
    println!("{:?}", out.iter().product::<usize>());
}

// not 25600