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
        X: usize,
        A: [usize; N],
    }

    let mut result = X;

    for n in A {
        result += if D % n == 0 { D / n } else { D / n + 1 };
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
7 1
2
5
10
"#
            ),
            r#"8"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
8 20
1
10
"#
            ),
            r#"29"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"5
30 44
26
18
81
18
6
"#
            ),
            r#"56"#
        );
    }
}
