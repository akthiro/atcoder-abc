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
        S: String,
        T: String,
    }

    let chars_s = S.chars().collect::<Vec<_>>();
    let chars_t = T.to_lowercase().chars().collect::<Vec<_>>();

    let mut i = 0;

    for c in chars_s {
        if c == chars_t[i] {
            i += 1;

            if i > 2 {
                break;
            }
        }
    }

    if (i == 2 && chars_t[2] == 'x') || i == 3 {
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
                r#"narita
NRT
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"losangeles
LAX
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"snuke
RNG
"#
            ),
            r#"No"#
        );
    }
}
