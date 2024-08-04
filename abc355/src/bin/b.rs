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
        M: usize,
        A: [usize; N],
        B: [usize; M],
    }

    let a = A.iter().map(|n| (n, true)).collect::<Vec<_>>();
    let b = B.iter().map(|n| (n, false)).collect::<Vec<_>>();

    let mut v = [a, b].concat();
    v.sort_by(|a, b| a.0.cmp(b.0));

    for i in 0..N + M - 1 {
        if v[i].1 && v[i + 1].1 {
            return "Yes".to_string();
        }
    }

    "No".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 2
3 2 5
4 1
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 2
3 1 5
4 2
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1 1
1
2
"#
            ),
            r#"No"#
        );
    }
}
