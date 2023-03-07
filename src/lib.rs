use std::fs;
use std::io::{BufRead, BufReader};
fn is_right(val: &char) -> bool {
    let right_s: String = String::from("yhnujmikolp");
    // check to see if val is in right
    for item in right_s.chars() {
        if val == &item {
            return true;
        }
    }
    return false;
}

// test is_right
#[test]
fn test_is_right() {
    assert!(is_right(&'y'));
    assert!(is_right(&'h'));
    assert!(is_right(&'n'));
    assert!(is_right(&'u'));
    assert!(is_right(&'j'));
    assert!(is_right(&'m'));
    assert!(is_right(&'i'));
    assert!(is_right(&'k'));
    assert!(is_right(&'o'));
    assert!(is_right(&'l'));
    assert!(is_right(&'p'));
}

pub fn is_pingpong(word: &String) -> bool {
    let lower_word = word.to_lowercase();

    let mut last_seen_right = true;
    for (i, item) in lower_word.char_indices() {
        if i == 0 {
            last_seen_right = is_right(&item);
        } else {
            // check to see if we have a non ascii or non
            // alphabetic value
            if !item.is_ascii_alphabetic() {
                return false;
            }
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
    assert!(!is_pingpong(&String::from("1o3j")));
    assert!(!is_pingpong(&String::from("楽o❤️j")));
}

pub fn check_file(path: std::path::PathBuf) -> Vec<String> {
    let file = fs::File::open(path).expect("Error when trying to open file");
    let reader = BufReader::new(file);
    let mut max_length = 0;
    let mut longest_words: Vec<String> = Vec::new();
    for line in reader.lines() {
        // convert the line to a single word
        let line = line.unwrap().trim().to_string();
        // check to see if it is an empty string
        if line.is_empty() {
            continue;
        }
        // check to see if it is valid
        if is_pingpong(&line) {
            if line.len() == max_length {
                longest_words.push(line);
            } else if line.len() > max_length {
                max_length = line.len();
                longest_words = vec![line];
            }
        }
    }
    longest_words
}
