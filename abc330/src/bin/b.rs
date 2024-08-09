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
        A: [usize; N],
    }

    let mut result = vec![];

    for x in A {
        if x < L {
            result.push(L);
        } else if R < x {
            result.push(R);
        } else {
            result.push(x);
        }
    }

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
                r#"5 4 7
3 1 4 9 7
"#
            ),
            r#"4 4 4 7 7"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 10 10
11 10 9
"#
            ),
            r#"10 10 10"#
        );
    }
}
