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
        txa: isize,
        tya: isize,
        txb: isize,
        tyb: isize,
        T: usize,
        V: usize,
        n: usize,
        xy: [(isize, isize); n],
    }

    for (x, y) in xy {
        let a = (((txa - x) * (txa - x) + (tya - y) * (tya - y)) as f64).sqrt();
        let b = (((txb - x) * (txb - x) + (tyb - y) * (tyb - y)) as f64).sqrt();

        if a + b <= (T * V) as f64 {
            return "YES".to_string();
        }
    }

    "NO".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1 1 8 2 2 4
1
4 5
"#
            ),
            r#"NO"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 1 8 2 2 6
1
4 5
"#
            ),
            r#"YES"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1 1 8 2 2 5
1
4 5
"#
            ),
            r#"YES"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"7 7 1 1 3 4
3
8 1
1 7
9 9
"#
            ),
            r#"YES"#
        );
    }
}
