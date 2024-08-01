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
        T: usize,
        A: [usize; N],
    }

    let mut t = T % A.iter().sum::<usize>();

    for (i, n) in A.iter().enumerate() {
        if t <= *n {
            return format!("{} {}", i + 1, t);
        } else {
            t -= A[i];
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 600
180 240 120
"#
            ),
            r#"1 60"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 281
94 94 94
"#
            ),
            r#"3 93"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 5678912340
1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000
"#
            ),
            r#"6 678912340"#
        );
    }
}
