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
        D: usize,
        T: [usize; N],
    }

    for i in 1..N {
        if T[i] - T[i - 1] <= D {
            return T[i].to_string();
        }
    }

    "-1".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4 500
300 900 1300 1700
"#
            ),
            r#"1300"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 99
100 200 300 400 500
"#
            ),
            r#"-1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4 500
100 600 1100 1600
"#
            ),
            r#"600"#
        );
    }
}
