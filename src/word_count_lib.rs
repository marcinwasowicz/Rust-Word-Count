pub mod dict_ops;
pub mod file_ops;

#[cfg(test)]
mod dict_ops_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_dictionary_building() {
        let elements = ["word1", "word2", "word1", "word3"]
            .iter()
            .map(|x| Ok(String::from(x.to_owned())));

        let expected_dictionary: HashMap<String, i32> = HashMap::from_iter(
            [("word1", 2), ("word2", 1), ("word3", 1)]
                .iter()
                .map(|(k, v)| (String::from(k.to_owned()), v.to_owned())),
        );
        assert_eq!(
            dict_ops::build_dictionary(elements).unwrap(),
            expected_dictionary
        );
    }
}
