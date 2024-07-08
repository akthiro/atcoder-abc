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
        _A: [usize; N],
    }

    let mut result = 0;

    let mut end = 0;

    while end < N {
        if end == 0 {
            end += K;
        } else {
            end += K - 1;
        }

        result += 1;
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
                r#"4 3
2 3 1 4
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 3
1 2 3
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8 3
7 3 1 8 4 6 2 5
"#
            ),
            r#"4"#
        );
    }
}
