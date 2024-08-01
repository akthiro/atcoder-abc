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
        H: usize,
        W: usize,
        S: [Chars; H],
    }

    let mut s = S;

    for row in s.iter_mut() {
        for i in 0..W - 1 {
            if row[i] == 'T' && row[i + 1] == 'T' {
                row[i] = 'P';
                row[i + 1] = 'C';
            }
        }
    }

    s.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 3
TTT
T.T
"#
            ),
            r#"PCT
T.T"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 5
TTT..
.TTT.
TTTTT
"#
            ),
            r#"PCT..
.PCT.
PCPCT"#
        );
    }
}
