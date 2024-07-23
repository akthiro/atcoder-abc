#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
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
        L1: isize,
        R1: isize,
        L2: isize,
        R2: isize,
    }

    let l = cmp::max(L1, L2);
    let r = cmp::min(R1, R2);

    if l > r {
        0.to_string()
    } else {
        (r - l).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"0 3 1 5
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"0 1 4 5
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"0 3 3 7
"#
            ),
            r#"0"#
        );
    }
}
