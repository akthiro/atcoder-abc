#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
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
        H: usize,
        _W: usize,
        S: [Chars; H],
    }

    let mut h1 = usize::MAX;
    let mut h2 = usize::MIN;
    let mut w1 = usize::MAX;
    let mut w2 = usize::MIN;

    for (h, row) in S.iter().enumerate() {
        for (w, col) in row.iter().enumerate() {
            if *col == '#' {
                h1 = cmp::min(h1, h);
                h2 = cmp::max(h2, h);
                w1 = cmp::min(w1, w);
                w2 = cmp::max(w2, w);
            }
        }
    }

    for (h, row) in S.iter().enumerate().take(h2 + 1).skip(h1) {
        for (w, col) in row.iter().enumerate().take(w2 + 1).skip(w1) {
            if *col == '.' {
                return format!("{} {}", h + 1, w + 1);
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 6
......
..#.#.
..###.
..###.
......
"#
            ),
            r#"2 4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 2
#.
##
##
"#
            ),
            r#"1 2"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6 6
..####
..##.#
..####
..####
..####
......
"#
            ),
            r#"2 5"#
        );
    }
}
