// use crate::word_count_lib::dict_ops;
use super::dict_ops;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn parse_input_file(input_path: &String) -> HashMap<String, i32> {
    let input_file = File::open(input_path).expect("Unable to open input file");
    let buffer_reader = BufReader::new(input_file);
    match dict_ops::build_dictionary(buffer_reader.lines()) {
        Ok(dictionary) => dictionary,
        Err(err) => panic!("Failed to parse input file. Details: {}", err),
    }
}

pub fn write_output_file(dictionary: HashMap<String, i32>, output_path: &String) {
    let output_file = File::create(output_path);
    match output_file {
        Ok(mut file) => {
            for (word, count) in dictionary {
                writeln!(file, "{}: {}", word, count);
            }
        }
        Err(err) => panic!("Failed to create output file. Details: {}", err),
    }
}
