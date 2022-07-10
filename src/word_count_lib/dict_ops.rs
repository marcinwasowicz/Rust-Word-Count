use std::collections::HashMap;
use std::io;

fn process_line(line: &String) -> Vec<&str> {
    line.split_whitespace().collect()
}

fn update_dictionary(dict: &mut HashMap<String, i32>, words: Vec<&str>) {
    for word in words {
        let count = dict.entry(word.to_owned()).or_insert(0);
        *count += 1;
    }
}

pub fn build_dictionary(
    buffer_reader: impl Iterator<Item = io::Result<String>>,
) -> Result<HashMap<String, i32>, String> {
    let mut word_count_dictionary = HashMap::new();
    for line in buffer_reader {
        match line {
            Ok(line_content) => {
                let words = process_line(&line_content);
                update_dictionary(&mut word_count_dictionary, words)
            }
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(word_count_dictionary)
}
