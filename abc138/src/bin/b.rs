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
        A: [usize; N],
    }

    let mut total = 0.0;

    for n in A {
        total += 1.0 / n as f64;
    }

    let result = 1.0 / total;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2
10 30
"#
            ),
            r#"7.5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
200 200 200
"#
            ),
            r#"66.66666666666667"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1
1000
"#
            ),
            r#"1000"#
        );
    }
}
