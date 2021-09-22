mod trie{
    use std::collections::{BTreeMap};
    use std::str::Chars;
    use serde::{Serialize, Deserialize};
    use std::io::{BufReader, BufWriter, Write, Read};
    use std::fs::{self, File};


    #[derive(Debug, Serialize, Deserialize)]
    struct TrieNode{
        children: BTreeMap<char, Box<TrieNode>>,
        is_word: bool
    }

    impl TrieNode{
        pub fn new(is_word: bool) -> Self{
            TrieNode{
                children: BTreeMap::new(),
                is_word
            }
        }
        pub fn insert(&mut self, letter_iter: & mut Chars, letter_count: usize){
            match letter_iter.next(){
                None => {/* set is_word in parent, cannot reach */ }
                Some(ch) => {
                    if letter_count > 1{
                        match self.children.get_mut(&ch){
                            None =>{
                                let mut child = TrieNode::default();
                                child.insert(letter_iter, letter_count-1);
                                self.children.insert(ch, Box::new(child));
                            },
                            Some(child) =>{
                                child.insert(letter_iter, letter_count -1);
                            }
                        }
                    }else{
                        match self.children.get_mut(&ch){
                            None =>{
                                self.children.insert(ch, Box::new(TrieNode::new(true)));
                            },
                            Some(child) =>{
                                child.is_word = true
                            }
                        }
                    }
                }
            }
        }
        pub fn remove(&mut self, letter_iter: & mut Chars) -> bool{
            match letter_iter.next(){
                None => {
                    if self.is_word{
                        self.is_word = false;
                        /* if has not child, remove chars until parent is word */
                        self.children.is_empty()
                    }else {
                        false
                    }
                }
                Some(ch) => {
                    match self.children.get_mut(&ch){
                        None => {
                            /* not found */
                            false
                        }
                        Some(child) => {
                            if child.remove(letter_iter) {
                                self.children.remove(&ch);
                                !self.is_word
                            }else {
                                false
                            }
                        }
                    }
                }
            }
        }
        pub fn search(&self, character_iter: & mut Chars) -> bool{
            match self.search_prefix_node(character_iter){
                None => false,
                Some(node) => node.is_word,
            }
        }
        fn search_prefix_node(&self, prefix_iter: &mut Chars) -> Option<&TrieNode>{
            let mut node = self;
            for ch in prefix_iter{
                match node.children.get(&ch){
                    None => return None,
                    Some(child) => {
                        node = child.as_ref();
                    }
                }
            }
            Some(node)
        }
        pub fn start_with(&self, prefix_iter: &mut Chars) -> bool{
            match self.search_prefix_node(prefix_iter){
                None => false,
                Some(_) => true,
            }
        }
        fn export_words_dfs(&self, prefix: & str, words: &mut Vec<String>){
            if self.is_word{
                words.push(prefix.to_string());
            }
            for (ch, child) in self.children.iter(){
                let next_prefix = format!("{:}{:}", prefix, ch);
                child.export_words_dfs(&next_prefix, words);
            }
        }
        fn export_words_with_prefix_dfs(&self, prefix: &str, current_prefix: &str, words: &mut Vec<String>){
            if current_prefix.len() > prefix.len(){
                return
            }else if current_prefix.len() == prefix.len() {
                if prefix != current_prefix{
                    return;
                }
                self.export_words_dfs(prefix, words);
            }else{
                for (ch, child) in self.children.iter(){
                    let next_prefix = format!("{:}{:}", current_prefix, ch);
                    child.export_words_with_prefix_dfs(prefix, &next_prefix, words);
                }
            }
        }
        fn clear(&mut self){
            for (_ch, child) in self.children.iter_mut(){
                child.clear();
            }
            self.children.clear();
        }
    }
    impl Default for TrieNode{
        fn default() -> Self{
            TrieNode{
                children: BTreeMap::new(),
                is_word: false
            }
        }
    }

    #[derive(Debug)]
    pub struct TrieTree{
        root: Box<TrieNode>,
    }


    impl TrieTree{
        pub fn new() -> Self{
            TrieTree{
                root: Box::default()
            }
        }
        pub fn insert(&mut self, item: &str){
            let mut letters_iter = item.chars();
            let letter_count = letters_iter.clone().count();
            if let Some(ch) = letters_iter.next(){
                if letter_count > 1{
                    match self.root.children.get_mut(&ch){
                        None =>{
                            let mut child = TrieNode::default();
                            child.insert(&mut letters_iter, letter_count-1);
                            self.root.children.insert(ch, Box::new(child));
                        },
                        Some(child) =>{
                            child.insert(&mut letters_iter, letter_count-1);
                        }
                    }
                }else {
                    // only one char
                    match self.root.children.get_mut(&ch){
                        None => {
                            self.root.children.insert(ch, Box::new(TrieNode::new(true)));
                        },
                        Some(node) =>{
                            node.is_word = true
                        }
                    }
                }
            }
        }
        pub fn remove(&mut self, item: & str){
            /*
             *   if last char has no child, set is_word false,
             *   otherwise delete char until parent node is_word
            */
            let mut letters_iter = item.chars();
            self.root.remove(&mut letters_iter);
        }
        pub fn remove_prefix(&mut self, prefix: & str){
            /*
            * remove all word with prefix
            * if prefix last char node is word, reset to false
            **/
        }
        pub fn search(&self, item: &str) -> bool{
            let mut character_iter = item.chars();
            match character_iter.next(){
                None => false,
                Some(ch) => {
                    match self.root.children.get(&ch){
                        None => false,
                        Some(child) => {
                            child.search(&mut character_iter)
                        }
                    }
                }
            }
        }
        pub fn starts_with(&self, prefix: &str) -> bool{
            let mut prefix_iter = prefix.chars();
            match prefix_iter.next(){
                None => false,
                Some(ch) => {
                    match self.root.children.get(&ch){
                        None => false,
                        Some(child) => {
                            child.start_with(&mut prefix_iter)
                        }
                    }
                }
            }
        }
        pub fn find_all(&self, prefix: &str) ->Vec<String>{
            let mut result: Vec<String> = Vec::new();
            self.root.export_words_with_prefix_dfs(prefix, "", &mut result);
            result
        }
        pub fn get_all_words(&self) -> Vec<String>{
            let mut words = Vec::new();
            self.root.export_words_dfs("", &mut words);
            words
        }
        pub fn clear(&mut self){
            self.root.clear();
            self.root.children.clear();
        }
        pub fn dump(&self, file_path: &str){
            let msg = format!("File {} not found", file_path);
            let file = File::create(file_path).expect(&msg);
            let mut writer = BufWriter::new(file);
            let serialized_value = serde_json::to_string(self.root.as_ref()).expect("serialize error");
            writer.write_all(serialized_value.as_bytes()).expect("dump to file error");

        }
        pub fn load(file_path: & str) ->Self{
            let msg = format!("File {} not found", file_path);
            let file = File::open(file_path).expect(&msg);
            let mut reader = BufReader::new(file);
            let mut  serialized_value= String::new();
            reader.read_to_string(&mut serialized_value).expect("reader from file failed");
            let deserialized_tree: TrieNode = serde_json::from_str(&serialized_value).expect("deserialize failed");
            Self{
                root: Box::new(deserialized_tree)
            }
        }
    }
}

use pyo3::prelude::*;
use pyo3::ffi::Py_GetPath;

#[pyclass(subclass)]
struct Trie{
    root: trie::TrieTree,
}

#[pymethods]
impl Trie{
    #[new]
    fn new() -> Self{
        Self{
            root: trie::TrieTree::new(),
        }
    }
    ///@function: insert word to trie
    ///@param word: str
    #[text_signature = "insert($self, word)"]
    fn insert<'a>(&mut self, py: Python, word: &'a str){
        py.allow_threads(move || self.root.insert(word))
    }
    fn remove<'a>(&mut self, py: Python, word: &'a str){
        py.allow_threads(move || self.root.remove(word))
    }
    fn remove_prefix<'a>(& mut self, py: Python, prefix: &'a str){
        py.allow_threads(move || self.root.remove_prefix(prefix))
    }
    /// @function: find word is existed
    /// @param word: str
    /// @param prefix_mode: bool, default true
    /// @return: bool
    #[args(prefix_mode = "true")]
    #[text_signature = "search($self, word)"]
    fn search<'a>(&self, py: Python, word: & 'a str) -> bool{
        py.allow_threads(move || {
            self.root.search(word)
        })
    }
    fn search_prefix<'a>(& self, py: Python, prefix: & 'a str) -> bool{
        py.allow_threads(move || {
            self.root.starts_with(prefix)
        })
    }
    /// @function: find all words with prefix
    /// @param prefix: str
    /// @return: List[str]
    #[text_signature = "find_all($self, prefix)"]
    fn find_all<'a>(&self, py:Python, word: & 'a str) -> Vec<String>{
        py.allow_threads(move || self.root.find_all(word))
    }
    /// @function: get all words in the trie tree
    /// @return: List[str]
    #[text_signature = "get_all_words($self)"]
    fn get_all_words(&self, py:Python) -> Vec<String>{
        py.allow_threads(move || self.root.get_all_words())
    }

    #[text_signature = "clear($self)"]
    fn clear(& mut self, py: Python){
        py.allow_threads(move || {self.root.clear()})
    }

    fn dump<'a>(&self, py:Python, path:&'a str){
        py.allow_threads(move || self.root.dump(path))
    }

    #[staticmethod]
    fn load(py: Python, path: &str) -> Self{
        py.allow_threads(move || {
            Trie{
                root: trie::TrieTree::load(path)
            }
        })
    }
}

#[pymodule]
fn pytrie(_py: Python, m: & PyModule) -> PyResult<()>{
    m.add_class::<Trie>()?;
    Ok(())
}