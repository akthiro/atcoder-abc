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
        K: usize,
        S: String,
    }

    if S.len() <= K {
        S
    } else {
        format!("{}...", &S[..K])
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"7
nikoandsolstice
"#
            ),
            r#"nikoand..."#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"40
ferelibenterhominesidquodvoluntcredunt
"#
            ),
            r#"ferelibenterhominesidquodvoluntcredunt"#
        );
    }
}
