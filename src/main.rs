use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::io::BufReader;
use std::collections::HashMap;

fn word_count(input_path: &String, output_path: &String) {
    let input_file = File::open(input_path).expect("Unable to open input file");
    let buffer_reader = BufReader::new(input_file);

    let mut word_count_dictionary = HashMap::new();

    for line in buffer_reader.lines() {
        match line {
            Ok(line_content) => {
                let words = line_content.split_whitespace();
                for word in words {
                    let count = word_count_dictionary.entry(word.to_owned()).or_insert(0);
                    *count += 1;
                }
            },
            Err(err) => panic!("Error parsing input file. Details: {}", err),
        }
    }

    let output_file = File::create(output_path);
    match output_file {
        Ok(mut file) => { 
            for (word, count) in &word_count_dictionary {
                writeln!(file, "{}: {}", word, count);
            }
        },
        Err(err) => panic!("Failed to create output file. Details: {}", err),
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_exec_path, input_path, output_path] => word_count(input_path, output_path),
        _ => panic!("Invalid number of arguments. Two expected!"),
    }
}
