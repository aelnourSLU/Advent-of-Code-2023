use crate::util::file_reader;

pub fn start() {
    let input = file_reader("d06");
    let input = input
        .into_iter()
        .map(|s| {
            let split = s.split_terminator(": ");
            let second = split.last();
            let second = second
                .unwrap()
                .replace(" ", "")
                .parse::<usize>()
                .unwrap();
            second
        })
        .collect::<Vec<_>>();
    println!("input: {:?}", input);

    let time = input[0];
    let distance = input[1];
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
    println!("Number of hold times: {:?}", hold.len());
}

// solution: 33875953