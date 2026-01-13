use std::{
    fs,
    io::{stdin, Write},
    path::{Path, PathBuf},
};

use super::trie::Trie;

pub fn config_paths_for(file_name: &str) -> Vec<PathBuf> {
    let mut config_paths = Vec::new();

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        if let Ok(dev_asset_dir) = PathBuf::from(manifest_dir)
            .join(format!("../../../assets/{}", file_name))
            .canonicalize()
        {
            config_paths.push(dev_asset_dir);
        };
    } else {
        println!("CARGO_MANIFEST_DIR not set");
    };

    config_paths.push(PathBuf::from(format!(
        "/usr/share/mechanix/keyboard/assets/{}",
        file_name
    )));
    config_paths.push(PathBuf::from(format!(
        "/etc/mechanix/keyboard/assets/{}",
        file_name
    )));

    if let Some(home_dir) = dirs::home_dir() {
        config_paths.push(home_dir.join(format!(".config/mechanix/keyboard/assets/{}", file_name)));
    }

    config_paths
}

pub fn get_trie() -> Trie {
    let cached_file_path = dirs::home_dir().unwrap().join(format!(
        ".config/mechanix/keyboard/assets/words_cached.json"
    ));

    if cached_file_path.exists() {
        let struct_content = read_from_file(cached_file_path.to_str().unwrap());
        let trie: Trie = serde_json::from_str(&struct_content).unwrap();
        return trie;
    }

    let mut raw_file_path = PathBuf::new();
    for path in config_paths_for("trie/words_raw.tsv") {
        if path.exists() {
            raw_file_path = path;
            break;
        }
    }

    if !raw_file_path.exists() {
        println!("Could not find raw trie file");
        return Trie::new();
    }

    let trie = load_contents(raw_file_path.to_str().unwrap());
    let json: String = serde_json::to_string(&trie).unwrap();
    let _ = fs::create_dir_all(cached_file_path.parent().unwrap()).unwrap();
    write_to_file(cached_file_path.to_str().unwrap(), &json);
    trie
}

fn load_contents(path: &str) -> Trie {
    let content: String = read_from_file(path);
    let mut trie = Trie::new();
    for line in content.lines().skip(1) {
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
