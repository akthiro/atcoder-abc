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
        S1: String,
        S2: String,
    }

    let chars_s1 = S1.chars().collect::<Vec<_>>();
    let chars_s2 = S2.chars().collect::<Vec<_>>();

    if (chars_s1[0] == '.' && chars_s2[1] == '.') || (chars_s1[1] == '.' && chars_s2[0] == '.') {
        "No".to_string()
    } else {
        "Yes".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"##
.#
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#".#
#.
"#
            ),
            r#"No"#
        );
    }
}
