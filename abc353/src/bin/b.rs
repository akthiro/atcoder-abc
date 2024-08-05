#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        N: usize,
        K: usize,
        A: [usize; N],
    }

    let mut result = 1;
    let mut empty = K;

    for n in A {
        if n <= empty {
            empty -= n;
        } else {
            result += 1;
            empty = K - n;
        }
    }

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"7 6
2 5 1 4 1 2 3
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7 10
1 10 1 10 1 10 1
"#
            ),
            r#"7"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"15 100
73 8 55 26 97 48 37 47 35 55 5 17 62 2 60
"#
            ),
            r#"8"#
        );
    }
}
