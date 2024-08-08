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
        A: isize,
        M: isize,
        L: isize,
        R: isize,
    }

    let l = L - A;
    let r = R - A;

    let result = if l < 0 && 0 < r {
        (-l / M) + (r / M) + 1
    } else if 0 < l {
        (r / M) - (l / M) + if l % M == 0 { 1 } else { 0 }
    } else {
        (-l / M) - (-r / M) + if -r % M == 0 { 1 } else { 0 }
    };

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 3 -1 6
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"-2 2 1 1
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"-177018739841739480 2436426 -80154573737296504 585335723211047198
"#
            ),
            r#"273142010859"#
        );
    }
}
