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
        x1: usize,
        x2: usize,
        x3: usize,
        x4: usize,
        x5: usize,
    }

    if x1 == 0 {
        "1".to_string()
    } else if x2 == 0 {
        "2".to_string()
    } else if x3 == 0 {
        "3".to_string()
    } else if x4 == 0 {
        "4".to_string()
    } else if x5 == 0 {
        "5".to_string()
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"0 2 3 4 5
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 2 0 4 5
"#
            ),
            r#"3"#
        );
    }
}
