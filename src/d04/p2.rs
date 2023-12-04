use crate::util::file_reader;

pub fn start() {
    let input = file_reader("d04");
    let cards = input.into_iter()
        .map(|s| {
            let mut a = s.split_terminator(":");
            
            let num = a.next().unwrap();
            let num = num.split_ascii_whitespace();
            let num = num.last().unwrap().parse::<usize>().unwrap();
            
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
                .collect::<Vec<_>>();

            (num.to_owned(), winners, have)
        })  
        .collect::<Vec<_>>();
    let mut copy_list: Vec<usize> = Vec::new();
    let mut process = |(num, winners, have), mut v: &mut Vec<usize>| {
        let mut count = 0;
            for h in &have {
                for w in &winners {
                    if h == w {
                        count += 1;
                    }
                }
            }
        for i in 0..count {
            v.push(i + num + 1);
        }
        count
    };

    let get_card = |x: usize| -> (usize, Vec<usize>, Vec<usize>) {
        cards
            .iter()
            .filter(|&y| x == y.0)
            .reduce(|acc, e| e)
            .unwrap()
            .to_owned()
    };

    for x in &cards {
        process(x.to_owned(), &mut copy_list);
    }

    let mut scratchcards = copy_list.len() + cards.len();
    for i in 0..9999 {
        let mut next_cards = Vec::new();
        let mut next_list = Vec::new();
        for x in &copy_list {
            next_cards.push(get_card(*x));
        }
        for x in next_cards {
            scratchcards += process(x, &mut next_list);
        }
        if next_list.len() > 0 {
            copy_list.clear();
            copy_list = next_list.clone();
        } else {
            break;
        }
    }
    println!("scratch: {}", scratchcards);
}