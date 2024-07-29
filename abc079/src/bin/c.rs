#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
use std::{
    collections::HashMap,
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
        ABCD: Chars,
    }

    let abcd = ABCD
        .iter()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect::<Vec<_>>();

    let a = abcd[0];
    let b = abcd[1];
    let c = abcd[2];
    let d = abcd[3];

    let mut op = HashMap::new();
    op.insert(0, "+");
    op.insert(1, "-");

    for op1 in 0..2 {
        for op2 in 0..2 {
            for op3 in 0..2 {
                let result = match (op1, op2, op3) {
                    (0, 0, 0) => a + b + c + d,
                    (0, 0, 1) => a + b + c - d,
                    (0, 1, 0) => a + b - c + d,
                    (0, 1, 1) => a + b - c - d,
                    (1, 0, 0) => a - b + c + d,
                    (1, 0, 1) => a - b + c - d,
                    (1, 1, 0) => a - b - c + d,
                    (1, 1, 1) => a - b - c - d,
                    _ => unreachable!(),
                };

                if result == 7 {
                    return format!("{}{}{}{}{}{}{}=7", a, op[&op1], b, op[&op2], c, op[&op3], d);
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1222
"#
            ),
            r#"1+2+2+2=7"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"0290
"#
            ),
            r#"0-2+9+0=7"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3242
"#
            ),
            r#"3+2+4-2=7"#
        );
    }
}
