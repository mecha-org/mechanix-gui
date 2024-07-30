use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Node {
    pub children: HashMap<char, Node>,
    pub suggestions: Vec<(i32, String)>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            children: HashMap::new(),
            suggestions: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Trie {
    root: Node,
}

pub struct WeightedString {
    pub word: String,
    pub weight: i32,
}

impl Trie {
    pub fn new() -> Trie {
        Trie { root: Node::new() }
    }

    pub fn build(weighted_strings: Vec<WeightedString>) -> Trie {
        let mut trie = Trie::new();
        weighted_strings
            .into_iter()
            .for_each(|ws| trie.insert(ws.word, ws.weight));
        trie
    }

    pub fn insert(&mut self, word: String, weight: i32) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(|| Node::new());
            let pos = node
                .suggestions
                .binary_search_by_key(&weight, |&(w, _)| w)
                .unwrap_or_else(|x| x);
            node.suggestions.insert(pos, (weight, word.clone()));
        }
    }

    pub fn search(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for c in prefix.chars() {
            if let Some(child) = node.children.get(&c) {
                node = child;
            } else {
                return vec![];
            }
        }

        node.suggestions
            .iter()
            .take(3)
            .map(|(_, word)| word.clone())
            .collect()
    }

    pub fn next_char_probabilities(&self, prefix: &str) -> HashMap<String, f64> {
        let mut node = &self.root;
        for c in prefix.chars() {
            if let Some(child) = node.children.get(&c) {
                node = child;
            } else {
                return HashMap::new(); // Prefix not found, return an empty map
            }
        }

        let total_weight: i32 = node.suggestions.iter().map(|(w, _)| w).sum();
        let mut probabilities = HashMap::new();

        for (weight, word) in &node.suggestions {
            let probability = (total_weight as f64 - *weight as f64) / total_weight as f64;
            if let Some(next_char) = word.chars().nth(prefix.len()) {
                *probabilities.entry(next_char.to_string()).or_insert(0.0) += probability;
            }
        }

        probabilities
    }
}
