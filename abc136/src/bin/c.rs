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
        H: [isize; N],
    }

    let mut h = H;

    for i in (1..N).rev() {
        if h[i - 1] > h[i] + 1 {
            return "No".to_string();
        }

        if h[i - 1] == h[i] + 1 {
            h[i - 1] -= 1;
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
                r#"5
1 2 1 1 3
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4
1 3 2 1
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"5
1 2 3 4 5
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"1
1000000000
"#
            ),
            r#"Yes"#
        );
    }
}
