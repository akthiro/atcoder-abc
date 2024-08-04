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
        K: u32,
    }

    let mut result = vec![vec!['#']];

    for _ in 0..K {
        result = f(result);
    }

    result
        .iter()
        .map(|cs| cs.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn f(css: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = css.len();
    let mut result = vec![vec!['.'; n * 3]; n * 3];

    for i in 0..n * 3 {
        for j in 0..n * 3 {
            if (n <= i && i < n * 2) && (n <= j && j < n * 2) {
                continue;
            }

            result[i][j] = css[i % n][j % n];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1
"#
            ),
            r#"###
#.#
###"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
"#
            ),
            r#"#########
#.##.##.#
#########
###...###
#.#...#.#
###...###
#########
#.##.##.#
#########"#
        );
    }
}
