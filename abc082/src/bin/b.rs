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
        s: String,
        t: String,
    }

    let mut s = s.chars().collect::<Vec<_>>();
    s.sort();

    let mut t = t.chars().collect::<Vec<_>>();
    t.sort_by(|a, b| b.cmp(a));

    if s < t {
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
                r#"yx
axy
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"ratcode
atlas
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"cd
abc
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"w
ww
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            solve(
                r#"zzz
zzz
"#
            ),
            r#"No"#
        );
    }
}
