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
        W: [String; N],
    }

    for word in W {
        if ["and", "not", "that", "the", "you"].contains(&word.as_str()) {
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
                r#"10
in that case you should print yes and not no
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10
in diesem fall sollten sie no und nicht yes ausgeben
"#
            ),
            r#"No"#
        );
    }
}
