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
        w: String,
    }

    let mut map = HashMap::new();

    for c in w.chars() {
        map.entry(c).and_modify(|n| *n += 1).or_insert(1);
    }

    for n in map.values() {
        if n % 2 != 0 {
            return "No".to_string();
        }
    }

    "Yes".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"abaccaba
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"hthth
"#
            ),
            r#"No"#
        );
    }
}
