#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::HashMap,
    io::{self, Read},
};

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

    let mut map = HashMap::new();
    map.insert("ABC", false);
    map.insert("ARC", false);
    map.insert("AGC", false);
    map.insert("AHC", false);

    let titles = &[S1, S2, S3];

    for title in titles {
        map.insert(title, true);
    }

    map.iter()
        .filter(|(_, v)| !*v)
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .0
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"ARC
AGC
AHC
"#
            ),
            r#"ABC"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"AGC
ABC
ARC
"#
            ),
            r#"AHC"#
        );
    }
}
