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
        N: usize,
        X: usize,
        Y: usize,
        A: [usize; N],
        B: [usize; N],
    }

    let mut a = A;
    a.sort_by(|a, b| b.cmp(a));

    let mut b = B;
    b.sort_by(|a, b| b.cmp(a));

    let mut sum1 = 0;
    let mut cnt1 = 0;

    for n in a {
        sum1 += n;
        cnt1 += 1;

        if sum1 > X {
            break;
        }
    }

    let mut sum2 = 0;
    let mut cnt2 = 0;

    for n in b {
        sum2 += n;
        cnt2 += 1;

        if sum2 > Y {
            break;
        }
    }

    let result = cmp::min(cnt1, cnt2);

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4 7 18
2 3 5 1
8 8 1 4
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 200000000000000 200000000000000
1 1 1 1 1
2 2 2 2 2
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8 30 30
1 2 3 4 5 6 7 8
8 7 6 5 4 3 2 1
"#
            ),
            r#"6"#
        );
    }
}
