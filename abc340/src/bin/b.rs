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
        Q: usize,
        q: [(usize, usize); Q],
    }

    let mut result = vec![];
    let mut v = vec![];

    for (c, n) in q {
        match c {
            1 => {
                v.push(n);
            }
            2 => {
                result.push(v[v.len() - n]);
            }
            _ => unreachable!(),
        }
    }

    result
        .iter()
        .map(ToString::to_string)
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
                r#"5
1 20
1 30
2 1
1 40
2 3
"#
            ),
            r#"30
20"#
        );
    }
}
