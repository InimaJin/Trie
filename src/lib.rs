use std::collections::HashMap;

mod traits;

#[derive(Debug)]
pub struct TrieNode {
    map: HashMap<char, TrieNode>,
    end_of_word: bool,
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        let root = TrieNode {
            map: HashMap::new(),
            end_of_word: false,
        };

        Self { root }
    }

    /* Counts and returns the number of strings present in the Trie. */
    pub fn size(&self) -> usize {
        let mut stack: Vec<&TrieNode> = self.root.map.values().collect();
        let mut size = 0;

        while let Some(node) = stack.pop() {
            if node.end_of_word {
                size += 1;
            }

            node.map.values().for_each(|x| {
                stack.push(x);
            });
        }

        size
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /* Ensures that s is present in the Trie.
     * Returns true only if s is not present in the Trie when insert() is called. */
    pub fn insert(&mut self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        let mut is_new = false;
        let mut node = &mut self.root;
        for ch in s.chars() {
            node = node.map.entry(ch).or_insert_with(|| {
                is_new = true;
                TrieNode {
                    map: HashMap::new(),
                    end_of_word: false,
                }
            });
        }

        node.end_of_word = true;
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
     * The vector is not sorted, but the strings are grouped by prefix, i.e.
     * strings with common prefix are adjacent. */
    pub fn as_vec(&self) -> Vec<String> {
        let mut strings = Vec::new();

        self.walk_nodes(&mut vec![], &self.root, &mut strings);

        strings
    }
    /* Recursive function responsible for reading all strings in the Trie into the vector 'all'. */
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
