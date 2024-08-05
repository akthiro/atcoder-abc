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
        H: [usize; N],
    }

    if let Some((i, _)) = H.iter().enumerate().skip(1).find(|(_, n)| *n > &H[0]) {
        (i + 1).to_string()
    } else {
        (-1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4
3 2 5 2
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
4 3 2
"#
            ),
            r#"-1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"7
10 5 10 2 10 13 15
"#
            ),
            r#"6"#
        );
    }
}
