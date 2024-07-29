#![allow(non_snake_case)]

use proconio::{input, source::line::LineSource};
use std::io::{self, BufReader};

fn main() {
    let mut src = LineSource::new(BufReader::new(io::stdin()));

    input! {
        from &mut src,
        N: usize,
    }

    let mut l = vec![false; 2 * N + 1];

    l[0] = true;

    println!("{}", 1);

    loop {
        input! {
            from &mut src,
            n: usize,
        }

        if n == 0 {
            return;
        }

        l[n - 1] = true;

        let m = *l
            .iter()
            .enumerate()
            .filter(|(_, b)| !*b)
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
            .first()
            .unwrap();

        l[m] = true;

        println!("{}", m + 1);
    }
}
