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
        A: usize,
        B: usize,
    }

    let mut result = 0;

    for n in 1..=N {
        let m = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .sum::<usize>();

        if A <= m && m <= B {
            result += n;
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
                r#"20 2 5
"#
            ),
            r#"84"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10 1 2
"#
            ),
            r#"13"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"100 4 16
"#
            ),
            r#"4554"#
        );
    }
}
