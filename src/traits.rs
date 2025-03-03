use crate::Trie;
use std::fmt;

impl fmt::Display for Trie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.root.map)
    }
}

impl From<&Vec<String>> for Trie {
    fn from(sequence: &Vec<String>) -> Self {
        let mut trie = Self::new();
        for s in sequence {
            trie.insert(&s);
        }
        trie
    }
}
impl From<Vec<String>> for Trie {
    fn from(sequence: Vec<String>) -> Self {
        return Self::from(&sequence);
    }
}
impl From<&Vec<&str>> for Trie {
    fn from(sequence: &Vec<&str>) -> Self {
        let mut trie = Self::new();
        for s in sequence {
            trie.insert(s);
        }
        trie
    }
}
impl From<Vec<&str>> for Trie {
    fn from(sequence: Vec<&str>) -> Self {
        return Self::from(&sequence);
    }
}