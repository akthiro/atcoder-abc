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
        h: [usize; N],
    }

    let mut result = 0;
    let mut h = h;

    loop {
        if *h.iter().max().unwrap() == 0 {
            break;
        }

        let mut i = 0;

        while i < N {
            if h[i] == 0 {
                i += 1;
            } else {
                result += 1;

                while i < N && h[i] > 0 {
                    h[i] -= 1;
                    i += 1;
                }
            }
        }
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
                r#"4
1 2 2 1
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
3 1 2 3 1
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8
4 23 75 0 23 96 50 100
"#
            ),
            r#"221"#
        );
    }
}
