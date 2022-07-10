use crate::word_count_lib::file_ops;
use std::env;

pub mod word_count_lib;

fn word_count(input_path: &String, output_path: &String) {
    let word_count_dictionary = file_ops::parse_input_file(input_path);
    file_ops::write_output_file(word_count_dictionary, output_path);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_exec_path, input_path, output_path] => word_count(input_path, output_path),
        _ => panic!("Invalid number of arguments. Two expected!"),
    }
}
