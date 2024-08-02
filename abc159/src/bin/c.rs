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
        L: usize,
    }

    let result = (L as f64 / 3.0).powi(3);

    format!("{:.12}", result)
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
            r#"1.000000000000"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"999
"#
            ),
            r#"36926037.000000000000"#
        );
    }
}
