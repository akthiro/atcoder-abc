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
        A: [isize; N],
        B: [isize; N],
    }

    let mut result = 0;

    for i in 0..N {
        result += A[i] * B[i];
    }

    if result == 0 {
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
                r#"2
-3 6
4 2
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
4 5
-1 -3
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3
1 3 5
3 -6 3
"#
            ),
            r#"Yes"#
        );
    }
}
