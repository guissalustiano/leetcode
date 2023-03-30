fn main() {
    println!("Hello, world!");
}

pub fn is_subsequence(s: String, t: String) -> bool {
    let mut scs = s.chars();

    let mut sc = scs.next();
    for tc in t.chars() {
        match sc {
            None => break,
            Some(scc) => {
                if tc == scc {
                    sc = scs.next();
                } else {
                    continue
                }
            }
        }
    }

    // if traversing all consumed all s then a subarray
    sc.is_none()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::is_subsequence;

    #[test_case("abc", "ahbgdc", true)]
    #[test_case("axc", "ahbgdc", false)]
    #[test_case("b", "c", false)]
    fn examples(a: &str, b: &str, expected: bool) {
        assert_eq!(is_subsequence(a.to_string(), b.to_string()), expected)
    }
}
