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
        d: [usize; N],
    }

    let mut result = 0;

    for i in 0..N - 1 {
        for j in i + 1..N {
            result += d[i] * d[j];
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
                r#"3
3 1 2
"#
            ),
            r#"11"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7
5 0 7 8 3 3 2
"#
            ),
            r#"312"#
        );
    }
}
