pub mod trie{
    use std::collections::{BTreeMap, LinkedList};
    use std::str::Chars;
    use bincode;
    use serde::{Serialize, Deserialize};
    use std::io::{BufReader, BufWriter, Read, Write};
    use std::fs::{File};


    #[derive(Debug, Serialize, Deserialize)]
    struct TrieNode{
        children: BTreeMap<char, TrieNode>,
        is_word: bool
    }

    impl TrieNode{
        #[inline]
        pub fn new(is_word: bool) ->Self{
            TrieNode{
                children: BTreeMap::new(),
                is_word
            }
        }

        #[inline]
        pub fn insert(&mut self, word_iter: &mut Chars) -> usize{
            let mut node = self;
            for ch in word_iter{
                node.children.entry(ch).or_insert(TrieNode::default());
                node = node.children.get_mut(&ch).unwrap();
            }
            if node.is_word{
                return 0;
            }
            node.is_word = true;
            1
        }

        #[inline]
        pub fn search(&self, word_iter: &mut Chars) -> bool{
            match self.search_node(word_iter){
                None => {false}
                Some(node) => {
                    node.is_word
                }
            }
        }

        #[inline]
        pub fn search_prefix(&self, prefix_word_iter: &mut Chars) -> bool{
            match self.search_node(prefix_word_iter) {
                None => {false}
                Some(_) => {true}
            }
        }

        #[inline]
        fn search_node(&self, word_iter: &mut Chars) -> Option<&TrieNode>{
            let mut node = self;
            for ch in word_iter{
                match node.children.get(&ch){
                    None => {return None}
                    Some(child) => {
                        node = child;
                    }
                }
            }
            Some(node)
        }

        #[inline]
        fn count_prefix(&self, prefix_word_iter: & mut Chars) -> usize{
            match self.search_node(prefix_word_iter) {
                None => {
                    0
                }
                Some(child) => {
                    child.node_size()
                }
            }
        }

        fn collect_word_path_dfs(&self, prefix: & str, word_path_list: &mut Vec<String>){
            if self.is_word{
                word_path_list.push(prefix.to_string());
            }
            for (ch, child) in self.children.iter(){
                let next_prefix = format!("{:}{:}", prefix, ch);
                child.collect_word_path_dfs(&next_prefix, word_path_list);
            }
        }

        fn collect_prefix_path_dfs(&self, prefix: &str, current_prefix: &str, suffix_path_list: &mut Vec<String>){
            if current_prefix.len() > prefix.len(){
                return
            }else if current_prefix.len() == prefix.len() {
                if prefix != current_prefix{
                    return;
                }
                self.collect_word_path_dfs(prefix, suffix_path_list);
            }else{
                for (ch, child) in self.children.iter(){
                    let next_prefix = format!("{:}{:}", current_prefix, ch);
                    child.collect_prefix_path_dfs(prefix, &next_prefix, suffix_path_list);
                }
            }
        }

        unsafe fn common_remove(&mut self, word_iter: & mut Chars, word_mode: bool) -> usize{
            let mut node= self as *mut TrieNode;
            let mut stack = LinkedList::new();
            for ch in word_iter{
                match (*node).children.get_mut(&ch){
                    None => {
                        stack.clear();
                        return 0;
                    }
                    Some(child) => {
                        // let keys = child.children.keys().clone().collect::<Vec<_>>();
                        // println!("{:?}", keys);
                        stack.push_back((ch, child));
                    }
                }
                node = (*node).children.get_mut(&ch).unwrap() as * mut TrieNode;
            }
            if word_mode && !(*node).is_word{
                //remove word not found
                return 0;
            }
            (*node).is_word = false;
            if word_mode{
                if (*node).children.is_empty(){
                    //has not child, the node is last word charset, remove until parent is word
                    Self::strip_until_word(&mut stack);
                }
                stack.clear();
                1
            }
            else {
                let  count = (*node).node_size();
                Self::strip_until_word(&mut stack);
                stack.clear();
                count
            }
        }

        #[inline]
        fn strip_until_word(stack: &mut LinkedList<(char, & mut TrieNode)>){
            while  !stack.is_empty(){
                if let Some((_, node)) = stack.pop_back(){
                    for (_c, child) in node.children.iter(){
                        // println!("{:} --> {:}", ch, c);
                        if child.is_word{
                            return;
                        }
                    }
                    node.children.clear();
                    if node.is_word{
                        break;
                    }
                }
            }
        }

        #[inline]
        fn node_size(&self) -> usize{
            let mut count = 0;
            for (_, child) in self.children.iter(){
                count += child.node_size();
            }
            if self.is_word{
                count += 1;
            }
            count
        }

        #[inline]
        fn clear(&mut self){
            for (_, child) in self.children.iter_mut(){
                child.clear();
            }
            self.children.clear();
        }
    }

    impl Default for TrieNode{
        #[inline]
        fn default() -> Self {
            TrieNode{
                children: BTreeMap::new(),
                is_word: false
            }
        }
    }


    #[derive(Debug,Serialize, Deserialize)]
    pub struct Trie{
        root: TrieNode,
        len: usize
    }

    impl Trie{
        #[inline]
        pub fn new() ->Self{
            Trie{
                root: TrieNode::default(),
                len: 0
            }
        }

        #[inline]
        pub fn len(&self) -> usize{
            self.len
        }

        #[inline]
        pub fn insert(&mut self, text: &str){
            let mut word_iter = text.chars();
            self.len += self.root.insert(&mut word_iter);
        }
        #[inline]
        pub fn search(&self, text: &str) -> bool{
            let mut word_iter = text.chars();
            self.root.search(&mut word_iter)
        }
        #[inline]
        pub fn search_prefix(&self, prefix: &str) -> bool{
            let mut prefix_word_iter = prefix.chars();
            self.root.search_prefix(&mut prefix_word_iter)
        }

        #[inline]
        pub fn prefix_count(&self, prefix: & str) -> usize{
            let mut prefix_word_iter = prefix.chars();
            self.root.count_prefix(&mut prefix_word_iter)
        }

        #[inline]
        pub fn find_all_prefix(&self, prefix: & str) -> Vec<String>{
            let mut prefix_path_list = Vec::new();
            self.root.collect_prefix_path_dfs(prefix, "", &mut prefix_path_list);
            prefix_path_list
        }

        #[inline]
        pub fn find_all(&self) ->Vec<String>{
            let mut word_path_list = Vec::new();
            self.root.collect_word_path_dfs("", &mut word_path_list);
            word_path_list
        }

        #[inline]
        pub fn remove(&mut self, text: & str){
            let mut word_iter = text.chars();
            unsafe {
                self.len -= self.root.common_remove(&mut word_iter, true);
            }

        }

        #[inline]
        pub fn remove_prefix(&mut self, prefix: & str){
            let mut prefix_word_iter = prefix.chars();
            unsafe {
                self.len -= self.root.common_remove(&mut prefix_word_iter, false);
            }
        }

        #[inline]
        pub fn clear(&mut self){
            self.root.clear();
            self.len = 0;
        }

        #[inline]
        pub fn dump(&self, file_path: & str, fmt: & str){
            let mut writer = Self::create_writer(file_path);
            if fmt == "json"{
                let serialized_value = self.to_json_string();
                writer.write_all(serialized_value.as_bytes()).expect("write to file failed");
            }else if fmt == "yaml" {
                let serialized_value = self.to_yaml_string();
                writer.write_all(serialized_value.as_bytes()).expect("write to file failed");
            }else if fmt == "binary"{
                self.save_binary_to_file(&mut writer);
            }else{
                panic!("unexpected file type");
            }
        }

        #[inline]
        pub fn load(file_path: & str, fmt: & str) -> Self{
            let mut reader = Self::create_reader(file_path);
            if fmt == "json"{
                let mut serialized_value = String::new();
                reader.read_to_string(&mut serialized_value).expect("reader serialize value failed");
                Self::from_json_string(&serialized_value)
            }else if  fmt == "yaml"{
                let mut serialized_value = String::new();
                reader.read_to_string(&mut serialized_value).expect("reader serialize value failed");
                Self::from_yaml_string(&serialized_value)
            }else {
                Self::from_binary(&mut reader)
            }
        }

        #[inline]
        fn to_json_string(&self) ->String{
            let serialized_value = serde_json::to_string(&self).expect("serialize json error");
            serialized_value
        }

        #[inline]
        fn to_yaml_string(&self) ->String{
            let serialized_value = serde_yaml::to_string(&self).expect("serialize yaml error");
            serialized_value
        }

        #[inline]
        fn save_binary_to_file(&self, writer: & mut BufWriter<File>){
            bincode::serialize_into(writer, &self.root).expect("serialize binary failed");
        }

        #[inline]
        fn from_json_string(serialize_value: & str) ->Self{
            serde_json::from_str(serialize_value).expect("deserialize json failed")
        }

        #[inline]
        fn from_yaml_string(serialize_value: & str) ->Self{
            serde_yaml::from_str(serialize_value).expect("deserialize json failed")
        }

        #[inline]
        fn from_binary(reader: &mut BufReader<File>) -> Self{
            bincode::deserialize_from(reader).expect("deserialize binary failed")
        }

        #[inline]
        fn create_writer(file_path: & str) -> BufWriter<File>{
            let msg = format!("File {} not found", file_path);
            let file = File::create(file_path).expect(&msg);
            let writer = BufWriter::new(file);
            writer
        }

        #[inline]
        fn create_reader(file_path: & str) -> BufReader<File>{
            let msg = format!("File {} not found", file_path);
            let file = File::open(file_path).expect(&msg);
            let reader = BufReader::new(file);
            reader
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

