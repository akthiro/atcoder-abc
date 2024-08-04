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
        L: usize,
        R: usize,
    }

    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];

    for n in 1..=N {
        if n < L {
            a.push(n);
        } else if n > R {
            c.push(n);
        } else {
            b.push(n);
        }
    }

    b.reverse();

    let result = [a, b, c].concat();

    result
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 2 3
"#
            ),
            r#"1 3 2 4 5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7 1 1
"#
            ),
            r#"1 2 3 4 5 6 7"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 1 10
"#
            ),
            r#"10 9 8 7 6 5 4 3 2 1"#
        );
    }
}
