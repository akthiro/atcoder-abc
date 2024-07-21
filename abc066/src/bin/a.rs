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
        a: usize,
        b: usize,
        c: usize,
    }

    let mut l = [a, b, c];
    l.sort();

    let result = l[0] + l[1];

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"700 600 780
"#
            ),
            r#"1300"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10000 10000 10000
"#
            ),
            r#"20000"#
        );
    }
}
