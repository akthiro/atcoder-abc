#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
    io::{self, Read},
};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        K: usize,
        G: usize,
        M: usize,
    }

    let mut glass = 0;
    let mut mug = 0;

    for _ in 0..K {
        if glass == G {
            glass = 0;
            continue;
        }

        if mug == 0 {
            mug = M;
            continue;
        }

        let diff = cmp::min(G - glass, mug);
        glass += diff;
        mug -= diff;
    }

    format!("{} {}", glass, mug)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 300 500
"#
            ),
            r#"200 500"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 100 200
"#
            ),
            r#"0 0"#
        );
    }
}
