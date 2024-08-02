#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
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
        S: Chars,
    }

    let mut m = HashMap::new();
    m.insert('A', 0);
    m.insert('B', 0);
    m.insert('C', 0);
    m.insert('D', 0);
    m.insert('E', 0);
    m.insert('F', 0);

    for c in S {
        m.entry(c).and_modify(|n| *n += 1);
    }

    format!(
        "{} {} {} {} {} {}",
        m[&'A'], m[&'B'], m[&'C'], m[&'D'], m[&'E'], m[&'F'],
    )
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"BEAF
"#
            ),
            r#"1 1 0 0 1 1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"DECADE
"#
            ),
            r#"1 0 1 2 2 0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"ABBCCCDDDDEEEEEFFFFFF
"#
            ),
            r#"1 2 3 4 5 6"#
        );
    }
}
