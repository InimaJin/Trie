use std::collections::HashMap;
use std::fmt;

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
            end_of_word: false,
        };

        Self { root: root }
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
    }

    /* Removes an entire string s from the Trie.
     * Returns true if and only if s was present up until removal. */
    pub fn remove(&mut self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        /* Holds the index at which we can safely remove s without
         * unintentionally removing other strings with the same prefix as s. */
        let mut remove_index = None;
        let mut node = &mut self.root;
        for (i, ch) in s.chars().enumerate() {
            if node.end_of_word {
                /* Reset the index here to ensure we don't remove the substring of s which
                 * seems to be present in the Trie. */
                remove_index = None;
            }
            
            if let Some(next_node) = node.map.get_mut(&ch) {
                node = next_node;
                if node.map.len() > 1 {
                    remove_index = None;
                } else if remove_index.is_none() {
                    remove_index = Some(i);
                }
            } else {
                return false;
            }
        }

        // s is not present in the Trie.
        if !node.end_of_word {
            return false;
        }

        /* s is present in the Trie, but it also is a substring of a longer string within
         * the Trie which must not be removed accidentally when removing s. */
        if !node.map.is_empty() {
            node.end_of_word = false;
            return true;
        }

        // remove_index will not be None at this point.
        let remove_index = remove_index.unwrap();
        node = &mut self.root;
        for (i, ch) in s.chars().enumerate() {
            if i == remove_index {
                node.map.remove(&ch);
                break;
            }
            node = node.map.get_mut(&ch).unwrap();
        }

        true
    }

    /* Removes all strings from the Trie that share a common prefix s.
     * Returns true if at least one string has been removed.
     * In structure similar to remove(), so refer to its comments. */
    pub fn remove_pref(&mut self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        if s.len() == 1 {
            let ch = s.chars().next().unwrap();
            return self.root.map.remove(&ch).is_some();
        }

        let mut remove_index = None;
        let mut node = &mut self.root;
        for (i, ch) in s.chars().enumerate() {
            if let Some(next_node) = node.map.get_mut(&ch) {
                node = next_node;

                if node.map.len() > 1 && i != s.len() - 1 {
                    remove_index = None;
                } else if remove_index.is_none() {
                    remove_index = Some(i);
                }
            } else {
                return false;
            }
        }

        node = &mut self.root;
        let remove_index = remove_index.unwrap();
        for (i, ch) in s.chars().enumerate() {
            if i == remove_index {
                node.map.remove(&ch);
                break;
            }
            node = node.map.get_mut(&ch).unwrap();
        }

        true
    }

    pub fn contains(&self, s: &str) -> bool {
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
}

impl fmt::Display for Trie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.root.map)
    }
}