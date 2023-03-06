use std::fs;
use std::io::{BufReader, BufRead};
use clap::Parser;


/// Simple program to identify the longest word that
/// in a file that can be alternatively typed with your
/// right and left hand.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: std::path::PathBuf,
}

fn is_right(val: &u8) -> bool {
    let right_s: String = String::from("yhnujmikolp");
    let right = right_s.as_bytes();
    
    // check to see if val is in right
    for &item in right.iter() {
        if val == &item {
            return true;
        }
    }
    return false;

}

// test is_right
#[test]
fn test_is_right() {
    assert!(is_right(&b'y'));
    assert!(is_right(&b'h'));
    assert!(is_right(&b'n'));
    assert!(is_right(&b'u'));
    assert!(is_right(&b'j'));
    assert!(is_right(&b'm'));
    assert!(is_right(&b'i'));
    assert!(is_right(&b'k'));
    assert!(is_right(&b'o'));
    assert!(is_right(&b'l'));
    assert!(is_right(&b'p'));
}

fn is_pingpong(word: &String) -> bool {
    let lower_word = word.to_lowercase();
    let bytes = lower_word.as_bytes();
    
    let mut last_seen_right = true; 
    for (i, &item) in bytes.iter().enumerate() {
        if i == 0 {
            last_seen_right = is_right(&item);
        } else {
            let new_last_seen_right = is_right(&item);
            if new_last_seen_right == last_seen_right {
                return false;
            }
            last_seen_right = new_last_seen_right;
        }
    }
    true
}

#[test]
fn test_is_pingpong() {
    assert!(is_pingpong(&String::from("ajajaj")));
    assert!(is_pingpong(&String::from("AJajAJ")));
    assert!(!is_pingpong(&String::from("asdf")));
}

fn check_file(path: std::path::PathBuf) -> Vec<String> {
    let file = fs::File::open(path).expect("Error when trying to open file");
    let reader = BufReader::new(file);
    let mut max_length = 0; 
    let mut longest_words: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.is_empty() {
            continue;
        }
        if is_pingpong(&line) {
            if line.len() == max_length {
                longest_words.push(line);
            }
            else if line.len() > max_length {
                max_length = line.len();
                longest_words = vec![line];
            }
        } 
    } 
    longest_words
}

fn main() {
    let args = Cli::parse();
    let max_pingpong = check_file(args.path); 
    // print max_pingpong
    println!("{:?}", max_pingpong);
}

        // strip the line of whitespace
