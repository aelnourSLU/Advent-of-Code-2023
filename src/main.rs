#![allow(dead_code)]
#![allow(unused_imports)]

mod util;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;

use std::env;

fn main() {
    // most-recently-completed day and part
    let finished_days = 3;
    let finished_parts = 1;
    let mrc = (finished_days, finished_parts);
    let completed_days = mrc.0;
    // arg collection
    let args: Vec<String> = env::args().collect();

    // fn collection
    let funcs: Vec<(fn(), fn())> = vec![
        (d01::p1::start, d01::p2::start),
        (d02::p1::start, d02::p2::start),
        (d03::p1::start, d03::p2::start),
        (d04::p1::start, d04::p2::start),
        (d05::p1::start, d05::p2::start),
        (d06::p1::start, d06::p2::start),
        (d07::p1::start, d07::p2::start),
        (d08::p1::start, d08::p2::start),
        (d09::p1::start, d09::p2::start),
        (d10::p1::start, d10::p2::start),
        (d11::p1::start, d11::p2::start),
        (d12::p1::start, d12::p2::start),
        (d13::p1::start, d13::p2::start),
        (d14::p1::start, d14::p2::start),
        (d15::p1::start, d15::p2::start),
        (d16::p1::start, d16::p2::start),
        (d17::p1::start, d17::p2::start),
        (d18::p1::start, d18::p2::start),
        (d19::p1::start, d19::p2::start),
        (d20::p1::start, d20::p2::start),
        (d21::p1::start, d21::p2::start),
        (d22::p1::start, d22::p2::start),
        (d23::p1::start, d23::p2::start),
        (d24::p1::start, d24::p2::start),
        (d25::p1::start, d25::p2::start),
    ];
    
    // program selection via cmd line arg
    let err = "input error - enter [day: int] [part: int]";
    let errfn = || println!("{}", err);
    match args.len() {
        1 => {
            // desired manually-launched program goes here
            match mrc.1 {
                1 => funcs[mrc.0 - 1].0(),
                2 => funcs[mrc.0 - 1].1(),
                _ => errfn()
            }
        },
        3 => {
            let day = args[1].parse::<usize>();
            let part = args[2].parse::<usize>();
            if let (Ok(d), Ok(p)) = (day, part) {
                if d >= 1 && d <= completed_days {
                    match p {
                        1 => funcs[d - 1].0(),
                        2 => funcs[d - 1].1(),
                        _ => errfn(),
                    }
                } else { errfn(); }
            } else { errfn(); }
        },
        _ => { errfn(); },
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn init() {

    }
}