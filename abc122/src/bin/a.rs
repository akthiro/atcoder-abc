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
        b: String,
    }

    let result = match b.as_str() {
        "A" => "T",
        "T" => "A",
        "C" => "G",
        "G" => "C",
        _ => unreachable!(),
    };

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"A
"#
            ),
            r#"T"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"G
"#
            ),
            r#"C"#
        );
    }
}
