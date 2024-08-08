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
        N: usize,
        Q: [usize; N],
        A: [usize; N],
        B: [usize; N],
    }

    let mut a_max = usize::MAX;

    for i in 0..N {
        if A[i] > 0 {
            a_max = a_max.min(Q[i] / A[i]);
        }
    }

    let mut result = 0;

    for a in 0..=a_max {
        let mut b = usize::MAX;

        for i in 0..N {
            if B[i] > 0 {
                b = b.min((Q[i] - (A[i] * a)) / B[i]);
            }
        }

        result = result.max(a + b);
    }

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2
800 300
100 100
200 10
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
800 300
100 0
0 10
"#
            ),
            r#"38"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2
800 300
801 300
800 301
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"10
1000000 1000000 1000000 1000000 1000000 1000000 1000000 1000000 1000000 1000000
0 1 2 3 4 5 6 7 8 9
9 8 7 6 5 4 3 2 1 0
"#
            ),
            r#"222222"#
        );
    }
}
