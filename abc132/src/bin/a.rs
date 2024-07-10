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
        S: String,
    }

    let mut map: HashMap<char, usize> = HashMap::new();

    for c in S.chars() {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    if map.iter().all(|(_, v)| *v == 2) {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"ASSA
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"STOP
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"FFEE
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"FREE
"#
            ),
            r#"No"#
        );
    }
}
