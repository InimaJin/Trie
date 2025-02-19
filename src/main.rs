use trie::Trie;

fn main() {
    let mut t = Trie::new();

    t.insert("PWD");
    t.insert("PWDL");
    t.insert("PWDLA");
    t.remove("PWD");
    println!("{}", t);
}
