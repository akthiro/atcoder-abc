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
        sc: [(usize, usize); M],
    }

    for n in 0..=999 {
        let result = n.to_string();
        let chars = result.chars().collect::<Vec<_>>();

        if chars.len() != N {
            continue;
        }

        let mut ok = true;

        for (s, c) in sc.iter() {
            if chars[*s - 1].to_digit(10).unwrap() as usize != *c {
                ok = false;
            }
        }

        if ok {
            return result;
        }
    }

    format!("{}", -1)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 3
1 7
3 2
1 7
"#
            ),
            r#"702"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 2
2 1
2 3
"#
            ),
            r#"-1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3 1
1 0
"#
            ),
            r#"-1"#
        );
    }
}
