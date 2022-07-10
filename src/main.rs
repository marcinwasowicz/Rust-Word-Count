use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

fn process_line(line: &String) -> Vec<&str> {
    line.split_whitespace().collect()
}

fn update_dictionary(dict: &mut HashMap<String, i32>, words: Vec<&str>) {
    for word in words {
        let count = dict.entry(word.to_owned()).or_insert(0);
        *count += 1;
    }
}

fn build_dictionary(buffer_reader: BufReader<File>) -> Result<HashMap<String, i32>, String> {
    let mut word_count_dictionary = HashMap::new();
    for line in buffer_reader.lines() {
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

fn parse_input_file(input_path: &String) -> HashMap<String, i32> {
    let input_file = File::open(input_path).expect("Unable to open input file");
    let buffer_reader = BufReader::new(input_file);
    match build_dictionary(buffer_reader) {
        Ok(dictionary) => dictionary,
        Err(err) => panic!("Failed to parse input file. Details: {}", err),
    }
}

fn write_output_file(dictionary: HashMap<String, i32>, output_path: &String) {
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

fn word_count(input_path: &String, output_path: &String) {
    let word_count_dictionary = parse_input_file(input_path);
    write_output_file(word_count_dictionary, output_path);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_exec_path, input_path, output_path] => word_count(input_path, output_path),
        _ => panic!("Invalid number of arguments. Two expected!"),
    }
}
