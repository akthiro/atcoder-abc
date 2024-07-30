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
        _N: usize,
        S: Chars,
    }

    let mut result = vec![];
    let mut must_replace = true;

    for c in S {
        if c == '"' {
            must_replace = !must_replace;
        }

        if must_replace && c == ',' {
            result.push('.');
        } else {
            result.push(c);
        }
    }

    result.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"8
"a,b"c,d
"#
            ),
            r#""a,b"c.d"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
,,,,,
"#
            ),
            r#"....."#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"20
a,"t,"c,"o,"d,"e,"r,
"#
            ),
            r#"a."t,"c."o,"d."e,"r."#
        );
    }
}
