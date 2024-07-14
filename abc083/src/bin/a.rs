#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp::Ordering,
    io::{self, Read},
};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }

    match (A + B).cmp(&(C + D)) {
        Ordering::Greater => "Left".to_string(),
        Ordering::Less => "Right".to_string(),
        Ordering::Equal => "Balanced".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 8 7 1
"#
            ),
            r#"Left"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 4 5 2
"#
            ),
            r#"Balanced"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1 7 6 4
"#
            ),
            r#"Right"#
        );
    }
}
