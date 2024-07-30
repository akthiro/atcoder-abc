#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
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
        S: [Chars; N],
    }

    let mut result = 0;

    for i in 0..N {
        'j: for j in i + 1..N {
            for k in 0..M {
                if S[i][k] == 'x' && S[j][k] == 'x' {
                    continue 'j;
                }
            }

            result += 1;
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
                r#"5 5
ooooo
oooxx
xxooo
oxoxo
xxxxx
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 2
ox
xo
xx
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2 4
xxxx
oxox
"#
            ),
            r#"0"#
        );
    }
}
