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
        r: usize,
        D: usize,
        x: usize,
    }

    let mut result = vec![];
    let mut cur = x;

    for _ in 0..10 {
        cur = r * cur - D;
        result.push(cur);
    }

    result
        .iter()
        .map(|n| n.to_string())
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
                r#"2 10 20
"#
            ),
            r#"30
50
90
170
330
650
1290
2570
5130
10250"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4 40 60
"#
            ),
            r#"200
760
3000
11960
47800
191160
764600
3058360
12233400
48933560"#
        );
    }
}
