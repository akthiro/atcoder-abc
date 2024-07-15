#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::HashSet,
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
        s: usize,
    }

    let mut set = HashSet::new();
    set.insert(s);

    let mut cur = s;
    let mut i = 1;

    loop {
        if cur % 2 == 0 {
            cur /= 2;
        } else {
            cur = 3 * cur + 1;
        }

        i += 1;

        if set.contains(&cur) {
            return i.to_string();
        }

        set.insert(cur);
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"8
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7
"#
            ),
            r#"18"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"54
"#
            ),
            r#"114"#
        );
    }
}
