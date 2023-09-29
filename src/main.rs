pub struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end_of_word: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode {
                children: Default::default(),
                is_end_of_word: false,
            },
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            let index = (ch as u8 - b'a') as usize;
            if current.children[index].is_none() {
                current.children[index] = Some(Box::new(TrieNode {
                    children: Default::default(),
                    is_end_of_word: false,
                }));
            }
            current = current.children[index].as_mut().unwrap();
        }
        current.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for ch in word.chars() {
            let index = (ch as u8 - b'a') as usize;
            if current.children[index].is_none() {
                return false;
            }
            current = current.children[index].as_ref().unwrap();
        }
        current.is_end_of_word
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current = &self.root;
        for ch in prefix.chars() {
            let index = (ch as u8 - b'a') as usize;
            if current.children[index].is_none() {
                return false;
            }
            current = current.children[index].as_ref().unwrap();
        }
        true
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("fruit");
    trie.insert("rot");
    println!("{}", trie.search("fruit")); 
    println!("{}", trie.search("rot"));   
    println!("{}", trie.search("ro"));    
    println!("{}", trie.starts_with("ro")); 
}



