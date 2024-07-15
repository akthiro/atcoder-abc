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
        D: usize,
    }

    match D {
        25 => "Christmas".to_string(),
        24 => "Christmas Eve".to_string(),
        23 => "Christmas Eve Eve".to_string(),
        22 => "Christmas Eve Eve Eve".to_string(),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"25
"#
            ),
            r#"Christmas"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"22
"#
            ),
            r#"Christmas Eve Eve Eve"#
        );
    }
}
