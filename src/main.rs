extern crate clap;
extern crate rayon;
extern crate colored;


use clap::{App, Arg};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use colored::*;

fn shannon_entropy(s: &str) -> f64 {
    let mut counter = HashMap::new();
    for c in s.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }
    let len = s.len() as f64;
    counter
        .values()
        .map(|&count| {
            let p = count as f64 / len;
            -p * p.log2()
        })
        .sum()
}

fn process_file(filepath: &Path, entropy_dict: &Mutex<HashMap<String, Vec<(f64, String)>>>) {
    if let Ok(content) = fs::read_to_string(filepath) {
        for word in content.split_whitespace() {
            let entropy = shannon_entropy(word);
            if entropy > 3.0 {
                let mut dict = entropy_dict.lock().unwrap();
                let entry = dict.entry(word.to_string()).or_insert_with(Vec::new);
                let file_str = filepath.to_str().unwrap().to_string();
                if !entry.contains(&(entropy, file_str.clone())) {
                    entry.push((entropy, file_str));
                }
            }
        }
    }
}



fn main() {
        println!(r#"

        ███████ ███    ██ ████████ ██████   ██████  ██████  ██   ██ ██    ██       ██     ██  █████  ██      ██   ██ ███████ ██████  
        ██      ████   ██    ██    ██   ██ ██    ██ ██   ██ ██   ██  ██  ██        ██     ██ ██   ██ ██      ██  ██  ██      ██   ██ 
        █████   ██ ██  ██    ██    ██████  ██    ██ ██████  ███████   ████   █████ ██  █  ██ ███████ ██      █████   █████   ██████  
        ██      ██  ██ ██    ██    ██   ██ ██    ██ ██      ██   ██    ██          ██ ███ ██ ██   ██ ██      ██  ██  ██      ██   ██ 
        ███████ ██   ████    ██    ██   ██  ██████  ██      ██   ██    ██           ███ ███  ██   ██ ███████ ██   ██ ███████ ██   ██ 
                                                                                                                                     
                                                                                                                                     
    Created By 1337-SIGMA                                                    
"#);
    let matches = App::new("Entropy Finder")
        .version("1.0")
        .author("1337-SIGMA")
        .about("Finds high entropy strings in files")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .value_name("DIRECTORY")
                .help("Sets the directory to search")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("entropy")
                .short("e")
                .long("entropy")
                .value_name("ENTROPY")
                .help("Sets the entropy threshold")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let directory = matches.value_of("directory").unwrap();
    let entropy_threshold: f64 = matches
        .value_of("entropy")
        .unwrap_or("3.0")
        .parse()
        .expect("Failed to parse entropy threshold");

    let entropy_dict = Arc::new(Mutex::new(HashMap::new()));

    let paths: Vec<_> = fs::read_dir(directory)
        .expect("Failed to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();

    let entropy_dict_clone = entropy_dict.clone();
    paths.par_iter().for_each(|path| {
        process_file(path, &entropy_dict_clone);
    });

    println!("{}", "High Entropy Strings:".green().bold());
    println!("{}", "=====================".green());

    let dict = entropy_dict.lock().unwrap();
    for (word, entries) in dict.iter() {
        for (entropy, filename) in entries {
            if *entropy > entropy_threshold {
                println!(
                    "{}: {} [Found in: {}]",
                    word.blue().bold(),
                    format!("{:.2}", entropy).red(),
                    filename.yellow()
                );
            }
        }
    }
}
