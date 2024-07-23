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
        A: isize,
        B: isize,
        C: u32,
    }

    let c = if C % 2 == 0 { 2 } else { 1 };
    let a = A.pow(c);
    let b = B.pow(c);

    match a.cmp(&b) {
        Ordering::Less => "<".to_string(),
        Ordering::Greater => ">".to_string(),
        Ordering::Equal => "=".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 2 4
"#
            ),
            r#">"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"-7 7 2
"#
            ),
            r#"="#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"-8 6 3
"#
            ),
            r#"<"#
        );
    }
}
