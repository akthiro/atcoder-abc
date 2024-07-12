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
        _N: usize,
        M: usize,
        X: usize,
        A: [usize; M],
    }

    let mut small = 0;
    let mut big = 0;

    for n in A {
        if n < X {
            small += 1;
        }

        if n > X {
            big += 1;
        }
    }

    let result = cmp::min(small, big);

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 3 3
1 2 4
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7 3 2
4 5 6
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 7 5
1 2 3 4 6 8 9
"#
            ),
            r#"3"#
        );
    }
}
