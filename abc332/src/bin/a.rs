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
        S: usize,
        K: usize,
        PQ: [(usize, usize); N],
    }

    let mut total = 0;

    for (p, q) in PQ {
        total += p * q;
    }

    let shipping = if total >= S { 0 } else { K };
    let result = total + shipping;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 2000 500
1000 1
100 6
"#
            ),
            r#"2100"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 2000 500
1000 1
100 6
5000 1
"#
            ),
            r#"6600"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2 2000 500
1000 1
1000 1
"#
            ),
            r#"2000"#
        );
    }
}
