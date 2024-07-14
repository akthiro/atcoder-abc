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
        H: usize,
        _W: usize,
        C: [String; H],
    }

    let mut result = vec![];

    for s in C {
        result.push(s.clone());
        result.push(s);
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 2
*.
.*
"#
            ),
            r#"*.
*.
.*
.*"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 4
***.
"#
            ),
            r#"***.
***."#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"9 20
.....***....***.....
....*...*..*...*....
...*.....**.....*...
...*.....*......*...
....*.....*....*....
.....**..*...**.....
.......*..*.*.......
........**.*........
.........**.........
"#
            ),
            r#".....***....***.....
.....***....***.....
....*...*..*...*....
....*...*..*...*....
...*.....**.....*...
...*.....**.....*...
...*.....*......*...
...*.....*......*...
....*.....*....*....
....*.....*....*....
.....**..*...**.....
.....**..*...**.....
.......*..*.*.......
.......*..*.*.......
........**.*........
........**.*........
.........**.........
.........**........."#
        );
    }
}
