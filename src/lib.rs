use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    map: HashMap<char, TrieNode>,
    end_of_word: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        let root = TrieNode {
            map: HashMap::new(),
            end_of_word: true,
        };

        Self { root }
    }

    pub fn insert(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }

        let mut node = &mut self.root;
        for ch in s.chars() {
            node = node.map.entry(ch).or_insert(TrieNode {
                map: HashMap::new(),
                end_of_word: false,
            });
        }
        
        node.end_of_word = true;
        println!("{:#?}", self.root.map);
    }    

    pub fn contains_pref(&self, s: &str) -> bool {
        let mut node = &self.root;
        for ch in s.chars() {
            if let Some(next_node) = node.map.get(&ch) {
                node = next_node;
            } else {
                return false;
            }
        }

        true
    }
    
    pub fn contains_full(&self, s: &str) -> bool {
        let mut node = &self.root;
        for ch in s.chars() {
            if let Some(next_node) = node.map.get(&ch) {
                node = next_node;
            } else {
                return false;
            }
        }
        node.end_of_word
    }
}
