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
    }

    let mut n = N;
    let mut sum = 0;
    let mut digits = 0;
    let mut rems = [0; 3];

    while n > 0 {
        let rem = n % 10;
        rems[rem % 3] += 1;
        sum += rem;
        digits += 1;
        n /= 10;
    }

    if sum % 3 == 1 {
        if rems[1] >= 1 && digits > 1 {
            1.to_string()
        } else if rems[2] >= 2 && digits > 2 {
            2.to_string()
        } else {
            (-1).to_string()
        }
    } else if sum % 3 == 2 {
        if rems[2] >= 1 && digits > 1 {
            1.to_string()
        } else if rems[1] >= 2 && digits > 2 {
            2.to_string()
        } else {
            (-1).to_string()
        }
    } else {
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"35
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"369
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6227384
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"11
"#
            ),
            r#"-1"#
        );
    }
}
