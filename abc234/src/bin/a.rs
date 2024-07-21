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
        t: usize,
    }

    let result = f(f(f(t) + t) + f(f(t)));

    format!("{}", result)
}

fn f(x: usize) -> usize {
    x * x + 2 * x + 3
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"0
"#
            ),
            r#"1371"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
"#
            ),
            r#"722502"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10
"#
            ),
            r#"1111355571"#
        );
    }
}
