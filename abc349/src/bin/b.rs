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

    let chars = S.chars().collect::<Vec<_>>();

    let mut map = HashMap::new();

    for c in chars {
        map.entry(c).and_modify(|n| *n += 1).or_insert(1);
    }

    for n in 1..=S.len() {
        let mut cnt = 0;

        for m in map.values() {
            if n == *m {
                cnt += 1;
            }
        }

        if cnt != 0 && cnt != 2 {
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
                r#"commencement
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"banana
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"ab
"#
            ),
            r#"Yes"#
        );
    }
}
