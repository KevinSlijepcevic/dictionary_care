use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const WHITESPACE: char = ' ';

/// Create BTreeMap from a given dictionary
fn create_dictionary(reader: BufReader<File>) -> Result<BTreeMap<String, u32>, std::io::Error> {
    let mut map = BTreeMap::new();
    for line_result in reader.lines() {
        let line = line_result?;
        match deconstruct_dictionary_line(&line) {
            Some((word, value)) => map.entry(sanitize_word(word)).or_insert(value),
            None => break,
        };
    }
    Ok(map)
}

/// Update BTreeMap from a given word list
fn update_dictionary(
    map: &mut BTreeMap<String, u32>,
    reader: BufReader<File>,
) -> Result<(), std::io::Error> {
    for line_result in reader.lines() {
        let line = line_result?;
        match get_word_from_line(&line) {
            Some(word) => *map.entry(sanitize_word(word)).or_insert(0) += 1,
            None => break,
        }
    }
    Ok(())
}

fn get_word_from_line(line: &String) -> Option<&str> {
    let splitted_line: Vec<&str> = line.split(WHITESPACE).collect();
    match splitted_line.get(0) {
        Some(word) => Some(*word),
        None => None,
    }
}

/// Deconstructs line into 'word' and corresponding 'value'.
/// Returns None if error arises
fn deconstruct_dictionary_line(line: &String) -> Option<(&str, u32)> {
    if !line.is_empty() {
        let splitted_line: Vec<&str> = line.split(WHITESPACE).collect();
        let word = match splitted_line.get(0) {
            Some(word) => *word,
            None => return None,
        };
        let value = match splitted_line.get(1) {
            Some(value) => *value,
            None => return None,
        };
        let value_u32 = match value.parse::<u32>() {
            Ok(value) => value,
            Err(_) => return None,
        };
        Some((word, value_u32))
    } else {
        None
    }
}

/// Removes punctuations from a &str and converts it to a lowercase String.
fn sanitize_word(word: &str) -> String {
    word.replace(&[',', '.', '!', '?'][..], "").to_lowercase()
}

/// Opens a file from a path and returns a new buffered reader
fn open_file(path: &str) -> BufReader<File> {
    let file = File::open(path)
        .unwrap_or_else(|error| panic!("Could not open the dictionary: {:?}", error));
    BufReader::new(file)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dictionary_reader = open_file(&args[1]);
    let mut map = match create_dictionary(dictionary_reader) {
        Ok(map) => map,
        Err(error) => panic!("Problem while creating map: {:?}", error),
    };
    let input_reader = open_file(&args[2]);
    match update_dictionary(&mut map, input_reader) {
        Ok(_) => (),
        Err(error) => panic!("Problem while updating map: {:?}", error),
    }
    for (key, val) in map {
        if val != 0 {
            println!("{key}: {val}");
        }
    }
}
