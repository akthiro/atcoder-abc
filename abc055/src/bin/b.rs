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
    }

    let m = 10_usize.pow(9) + 7;
    let mut result = 1;

    for n in 1..=N {
        result *= n;
        result %= m;
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
                r#"3
"#
            ),
            r#"6"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10
"#
            ),
            r#"3628800"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"100000
"#
            ),
            r#"457992974"#
        );
    }
}
