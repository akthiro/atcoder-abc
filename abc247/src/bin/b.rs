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
        ST: [(String, String); N],
    }

    for (i, (s, t)) in ST.iter().enumerate() {
        let mut names = vec![];

        for (j, (s, t)) in ST.iter().enumerate() {
            if i == j {
                continue;
            }

            names.push(s.clone());
            names.push(t.clone());
        }

        if names.contains(&s.clone()) && names.contains(&t.clone()) {
            return "No".to_string();
        }
    }

    "Yes".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
tanaka taro
tanaka jiro
suzuki hanako
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
aaa bbb
xxx aaa
bbb yyy
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2
tanaka taro
tanaka taro
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"3
takahashi chokudai
aoki kensho
snu ke
"#
            ),
            r#"Yes"#
        );
    }
}
