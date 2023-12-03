use crate::util::file_reader;

pub fn start() {
    let input = file_reader("d03");
    let coords = input
        .iter()
        .enumerate()
        .map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(move |(x, c)| {
                    if !c.is_digit(10) && c != '.' {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .filter(|x| x.is_some())
        .collect::<Vec<_>>();
    let adj = |(x, y)| {
        vec![
            (x, y),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y),
            (x - 1, y),
            (x + 1, y + 1),
            (x - 1, y + 1),
            (x - 1, y - 1),
            (x + 1, y - 1),
        ]
    };
    let y_lim = input.len() as isize;
    let x_lim = input[0].len() as isize;
    for coord in coords {
        for (x, y) in adj(coord.unwrap()) {
            if x > 0 && x <= x_lim && y > 0 && y <= y_lim {
                let target_line = &input[y as usize];
                let target_chars = target_line.chars().collect::<Vec<_>>();
                if let Some(c) = target_chars.get(x as usize) {
                    
                }
            }
        }
    }
}