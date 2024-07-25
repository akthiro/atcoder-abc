#![allow(non_snake_case)]

use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> String {
    let mut result = vec![];

    loop {
        input! {
            n: usize,
        }

        result.push(n);

        if n == 0 {
            break;
        }
    }

    result.reverse();

    result
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}
