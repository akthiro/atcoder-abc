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
        R: usize,
    }

    if R <= 99 {
        (100 - R).to_string()
    } else if R <= 199 {
        (200 - R).to_string()
    } else if R <= 299 {
        (300 - R).to_string()
    } else if R <= 399 {
        (400 - R).to_string()
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
                r#"123
"#
            ),
            r#"77"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"250
"#
            ),
            r#"50"#
        );
    }
}
