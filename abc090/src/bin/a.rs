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
        S1: String,
        S2: String,
        S3: String,
    }

    format!(
        "{}{}{}",
        S1.chars().collect::<Vec<_>>()[0],
        S2.chars().collect::<Vec<_>>()[1],
        S3.chars().collect::<Vec<_>>()[2],
    )
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"ant
obe
rec
"#
            ),
            r#"abc"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"edu
cat
ion
"#
            ),
            r#"ean"#
        );
    }
}
