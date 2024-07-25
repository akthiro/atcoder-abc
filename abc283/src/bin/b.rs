#![allow(non_snake_case)]

use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> String {
    input! {
        N: usize,
        A: [usize; N],
        Q: usize,
    }

    let mut result = vec![];

    let mut a = A;

    for _ in 0..Q {
        input! {
            n: usize,
        }

        if n == 1 {
            input! {
                k: usize,
                x: usize,
            }

            a[k - 1] = x;
        } else {
            input! {
                k: usize,
            }

            result.push(a[k - 1]);
        }
    }

    result
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}
