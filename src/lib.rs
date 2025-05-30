use std::{cell::Cell, collections::HashMap};

mod traits;

#[derive(Debug)]
pub struct TrieNode {
    map: HashMap<char, TrieNode>,
    end_of_word: bool,
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
    stored_size: Cell<Option<usize>>,
}

impl Trie {
    pub fn new() -> Self {
        let root = TrieNode {
            map: HashMap::new(),
            end_of_word: false,
        };

        Self {
            root,
            stored_size: Cell::new(Some(0)),
        }
    }

    /* Returns the number of strings in the Trie. If unknown, all strings are counted first and the size is stored. */
    pub fn size(&self) -> usize {
        if let Some(size) = self.stored_size.get() {
            return size;
        }

        let mut stack: Vec<&TrieNode> = self.root.map.values().collect();
        let mut size = 0;

        while let Some(node) = stack.pop() {
            if node.end_of_word {
                size += 1;
            }

            node.map.values().for_each(|x| {
                if x.map.is_empty() {
                    size += 1;
                } else {
                    stack.push(x);
                }
            });
        }

        self.stored_size.set(Some(size));
        size
    }

    /* Increments (incr = true) or decrements (incr = false) the stored size by 1. If stored size is None, nothing happens. */
    fn edit_size(&self, incr: bool) {
        let mut size;
        if let Some(s) = self.stored_size.get() {
            size = s;
        } else {
            return;
        }

        if incr {
            size += 1;
        } else if size != 0 {
            size -= 1;
        } else {
            return;
        }

        self.stored_size.set(Some(size));
    }

    pub fn is_empty(&self) -> bool {
        self.root.map.is_empty()
    }

    pub fn clear(&mut self) {
        self.root.map.clear();
        self.stored_size.set(Some(0));
    }

    /* Ensures that s is present in the Trie.
     * Returns true only if s is not present in the Trie when insert() is called. */
    pub fn insert(&mut self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        let mut node = &mut self.root;
        for ch in s.chars() {
            node = node.map.entry(ch).or_insert(TrieNode {
                map: HashMap::new(),
                end_of_word: false,
            });
        }

        let mut is_new = false;
        if !node.end_of_word {
            is_new = true;
            node.end_of_word = true;
        }

        if is_new {
            self.edit_size(true);
        }
        is_new
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
            self.edit_size(false);
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

        self.edit_size(false);
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
            self.stored_size.set(None);
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

        self.stored_size.set(None);
        true
    }

    /* Whether or not s is present in the Trie. */
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

    /* Whether or not at least one string with a prefix s is present in the Trie. */
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

    /* Builds and returns a vector holding all strings present in the Trie.
     * The vector is not sorted, but the strings are grouped by prefix. */
    pub fn as_vec(&self) -> Vec<String> {
        let mut strings = vec![];

        self.walk_nodes(&mut vec![], &self.root, &mut strings);

        strings
    }

    /* Like as_vec(). However, the returned vector only holds strings that share a common prefix s. */
    pub fn as_vec_pref(&self, s: &str) -> Vec<String> {
        if s.is_empty() {
            return vec![];
        }
        let mut strings = vec![];
        let mut node = &self.root;
        for ch in s.chars() {
            if let Some(next_node) = node.map.get(&ch) {
                node = next_node;
            } else {
                return vec![];
            }
        }

        //walk_nodes() does not consider that the prefix itself might be a string present in the Trie.
        if node.end_of_word {
            strings.push(s.into())
        }

        /* 'node' is the node pointed to by the last character of s. tmp_string is initialized
         * with the characters of s. This way, walk_nodes will not pop any characters within the prefix. */
        self.walk_nodes(&mut s.chars().collect(), node, &mut strings);

        strings
    }

    /* Recursively walks all the child nodes of 'node' to construct the strings formed by their characters,
     * while feeding the complete strings into all_strings. */
    fn walk_nodes(
        &self,
        tmp_string: &mut Vec<char>,
        node: &TrieNode,
        all_strings: &mut Vec<String>,
    ) {
        for (ch, next_node) in node.map.iter() {
            tmp_string.push(*ch);
            if next_node.end_of_word {
                all_strings.push(tmp_string.iter().collect());
            }
            if !next_node.map.is_empty() {
                self.walk_nodes(tmp_string, next_node, all_strings);
            }
            tmp_string.pop();
        }
    }
}
