#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
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
        N: usize,
        ST: [(String, String); N],
    }

    let mut map = HashMap::new();

    for (s, t) in ST {
        let name = format!("{} {}", s, t);

        if map.contains_key(&name) {
            return "Yes".to_string();
        }

        map.insert(name, true);
    }

    "No".to_string()
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
sato hanako
tanaka taro
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
saito ichiro
saito jiro
saito saburo
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4
sypdgidop bkseq
bajsqz hh
ozjekw mcybmtt
qfeysvw dbo
"#
            ),
            r#"No"#
        );
    }
}
