#![feature(test)]
extern crate test;
extern crate trie_rs;
use std::time;
use std::fs::File;
use std::io::{BufReader, BufRead};
use trie_rs::trie::Trie;


fn read_data(file_path: & str) -> Vec<String>{
    let mut result  = Vec::new();
    let file = File::open(file_path).expect("open error");
    let reader = BufReader::new(file);
    for line in reader.lines(){
        if let Ok(text) = line{
            let text_value = text.to_string();
            result.push(text_value.to_owned());
        }
    }
    result
}
fn trie_insert(trie: & mut Trie, texts:& Vec<String>){
    for text in texts{
        trie.insert(text);
    }
}
fn trie_search(trie: &Trie, texts: & Vec<String>){
    for text in texts{
        trie.search(text);
    }
}

#[cfg(test)]
mod tests{
    use super::{read_data, trie_insert, trie_search};
    use test::Bencher;
    use super::Trie;

    #[bench]
    fn bench_trie_insert(b: &mut Bencher){
        let mut trie = Trie::new();
        let texts = read_data("text_data.txt");
        b.iter(||{
            trie_insert(& mut trie, &texts);
        });
    }
    #[bench]
    fn bench_trie_search(b: & mut Bencher){
        let mut trie = Trie::new();
        let texts = read_data("text_data.txt");
        for text in texts.iter(){
            trie.insert(text);
        }
        b.iter(||{
            trie_search(&trie, &texts);
        });
    }
}
fn main() {
    println!("Hello, world!");
}
