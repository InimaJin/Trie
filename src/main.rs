use trie::Trie;

fn main() {
    let mut t = Trie::new();
    t.insert("Hello");
    t.insert("Hi");
    t.insert("Why");
    
    println!("{}", t.contains_full("Hello"));
}
