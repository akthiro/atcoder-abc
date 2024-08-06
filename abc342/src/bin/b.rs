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
        P: [usize; N],
        Q: usize,
        AB: [(usize, usize); Q],
    }

    let mut p = vec![0; N];

    for (i, x) in P.iter().enumerate() {
        p[x - 1] = i;
    }

    let mut result = vec![];

    for (a, b) in AB {
        result.push(if p[a - 1] < p[b - 1] { a } else { b });
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
                r#"3
2 1 3
3
2 3
1 2
1 3
"#
            ),
            r#"2
2
1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7
3 7 2 1 6 5 4
13
2 3
1 2
1 3
3 6
3 7
2 4
3 7
1 3
4 7
1 6
2 4
1 3
1 3
"#
            ),
            r#"3
2
3
3
3
2
3
3
7
1
2
3
3"#
        );
    }
}
