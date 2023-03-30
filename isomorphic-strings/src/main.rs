fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct LittleMap(Vec<(char, char)>);

impl LittleMap {
    fn with_capacity(size: usize) -> LittleMap {
        LittleMap(Vec::with_capacity(size))
    }

    fn insert(&mut self, key: char, value: char) {
        self.0.push((key, value))
    }

    fn get_by_key_or_value(&self, k_search: char, v_search: char) -> Option<&(char, char)> {
        self.0.iter().find(|(k, v)| *k == k_search || *v == v_search )
    }
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut map = LittleMap::with_capacity(s.len());

    s.chars().zip(t.chars()).all(|(cs, ts)| {
        if let Some((cs_previous, ts_previous)) = map.get_by_key_or_value(cs, ts)  {
            // check if is the same char map
            !(ts != *ts_previous || cs != *cs_previous)
        } else {
            map.insert(cs, ts);
            true // keeps running
        }
    })
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::is_isomorphic;

    #[test_case("egg", "add", true)]
    #[test_case("foo", "bar", false)]
    #[test_case("paper", "title", true)]
    #[test_case("badc", "baba", false)]
    fn examples(a: &str, b: &str, expected: bool) {
        assert_eq!(is_isomorphic(a.to_string(), b.to_string()), expected)
    }
}
