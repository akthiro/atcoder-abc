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
        S: usize,
        D: usize,
        XY: [(usize, usize); N],
    }

    for (x, y) in XY {
        if x >= S || y <= D {
            continue;
        }

        return "Yes".to_string();
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
                r#"4 9 9
5 5
15 5
5 15
15 15
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 691 273
691 997
593 273
691 273
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"7 100 100
10 11
12 67
192 79
154 197
142 158
20 25
17 108
"#
            ),
            r#"Yes"#
        );
    }
}
