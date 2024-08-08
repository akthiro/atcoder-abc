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
        Q: usize,
        q: [(isize, String); Q],
    }

    let mut dragon: Vec<(isize, isize)> = vec![];

    for i in (1..=N).rev() {
        dragon.push((i as isize, 0));
    }

    let mut result = vec![];

    for (n, s) in q {
        match n {
            1 => {
                let (x, y) = dragon[dragon.len() - 1];
                match s.as_str() {
                    "R" => dragon.push((x + 1, y)),
                    "L" => dragon.push((x - 1, y)),
                    "U" => dragon.push((x, y + 1)),
                    "D" => dragon.push((x, y - 1)),
                    _ => unreachable!(),
                }
            }
            2 => {
                let m = s.parse::<usize>().unwrap();
                result.push(dragon[dragon.len() - m]);
            }
            _ => unreachable!(),
        }
    }

    result
        .iter()
        .map(|(x, y)| format!("{} {}", x, y))
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
                r#"5 9
2 3
1 U
2 3
1 R
1 D
2 3
1 L
2 1
2 5
"#
            ),
            r#"3 0
2 0
1 1
1 0
1 0"#
        );
    }
}
