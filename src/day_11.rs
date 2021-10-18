#[macro_export]
macro_rules! hashmap {
    ($( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    #[test]
    fn sanity() {
        let hm = hashmap!("a" => 1);
        let mut std_hm = HashMap::new();
        std_hm.insert("a", 1);
        assert_eq!(std_hm, hm);
    }

    #[test]
    fn trailing_comma() {
        let hm = hashmap!("a" => 1,);
        let mut std_hm = HashMap::new();
        std_hm.insert("a", 1);
        assert_eq!(std_hm, hm);
    }

    #[test]
    fn more_pairs() {
        let hm = hashmap!("a" => 1, "b" => 2, "c" => 3);
        let mut std_hm = HashMap::new();
        std_hm.insert("a", 1);
        std_hm.insert("b", 2);
        std_hm.insert("c", 3);
        assert_eq!(std_hm, hm);
    }

    #[test]
    fn more_pairs_trailing_comma() {
        let hm = hashmap!(
            "a" => 1,
            "b" => 2,
            "c" => 3,
        );
        let mut std_hm = HashMap::new();
        std_hm.insert("a", 1);
        std_hm.insert("b", 2);
        std_hm.insert("c", 3);
        assert_eq!(std_hm, hm);
    }
}
