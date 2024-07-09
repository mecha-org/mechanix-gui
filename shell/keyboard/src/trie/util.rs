use std::{
    fs,
    io::{stdin, Write},
    path::Path,
};

use super::trie::Trie;

pub fn get_trie(raw_file: &str, cached_file: &str) -> Trie {
    //Check if struct file exits
    let path = Path::new(cached_file);
    let trie = if path.exists() {
        //if yes the load struct from file
        let struct_content = read_from_file(cached_file);
        let trie: Trie = serde_json::from_str(&struct_content).unwrap();
        trie
    } else {
        //if no load the struct from content file
        let trie = load_contents(raw_file);
        trie
    };
    //write to file
    let json: String = serde_json::to_string(&trie).unwrap();
    write_to_file(cached_file, &json);
    trie
}

fn load_contents(path: &str) -> Trie {
    let content: String = read_from_file(path);
    let mut trie = Trie::new();
    for line in content.lines().skip(1).take(41284) {
        let line_splitted: Vec<&str> = line.split('\t').collect();
        let string = line_splitted[0].to_owned();
        let weight = line_splitted[1].parse::<i32>().unwrap();
        trie.insert(string, weight);
    }
    trie
}

fn write_to_file(path: &str, content: &str) -> bool {
    let path = Path::new(path);
    let mut file = fs::File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    true
}

fn read_from_file(path: &str) -> String {
    let path = Path::new(path);
    let content = fs::read_to_string(path).unwrap();
    content
}
