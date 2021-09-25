extern crate trie_rs;
use trie_rs::trie;
use pyo3::prelude::*;
use regex::Regex;

#[pyclass(subclass)]
struct Trie{
    root: trie::Trie,
}

#[pymethods]
impl Trie{
    #[new]
    fn new() -> Self{
        Self{
            root: trie::Trie::new(),
        }
    }
    #[pyo3(text_signature = "($self)", name = "__len__")]
    fn len(&self, py: Python) -> usize{
        py.allow_threads(move || {
            self.root.len()
        })
    }
    #[pyo3(text_signature = "($self, word)")]
    fn insert<'a>(&mut self, py: Python, word: &'a str){
        py.allow_threads(move || self.root.insert(word))
    }

    #[pyo3(text_signature = "($self, word)")]
    fn remove<'a>(&mut self, py: Python, word: &'a str){
        py.allow_threads(
            move || self.root.remove(word)
        )
    }

    #[pyo3(text_signature = "($self, prefix)")]
    fn remove_prefix<'a>(& mut self, py: Python, prefix: &'a str){
        py.allow_threads(
            move || self.root.remove_prefix(prefix)
        )
    }

    #[pyo3(text_signature = "($self, word)")]
    fn search<'a>(&self, py: Python, word: & 'a str) -> bool{
        py.allow_threads(move || {
            self.root.search(word)
        })
    }
    #[pyo3(text_signature = "($self, prefix)")]
    fn startswith<'a>(& self, py: Python, prefix: & 'a str) -> bool{
        py.allow_threads(move || {
            self.root.search_prefix(prefix)
        })
    }
    #[pyo3(text_signature = "($self, prefix)")]
    fn prefix_count(&self, py: Python, prefix: & str) -> usize{
        py.allow_threads(move || {
            self.root.prefix_count(prefix)
        })
    }
    #[pyo3(text_signature = "($self, prefix)")]
    fn find_all_prefix<'a>(&self, py:Python, text: & 'a str) -> Vec<String>{
        py.allow_threads(move || self.root.find_all_prefix(text))
    }

    #[pyo3(text_signature = "($self)")]
    fn find_all(&self, py:Python) -> Vec<String>{
        py.allow_threads(move || self.root.find_all())
    }

    #[pyo3(text_signature = "($self)")]
    fn clear(& mut self, py: Python){
        py.allow_threads(move || {self.root.clear()})
    }

    #[args(fmt="\"json\"")]
    #[pyo3(text_signature = "($self, path, fmt)")]
    fn dump<'a>(&self, py:Python, path:&'a str, fmt: & 'a str){
        let re = Regex::new("^(json|yaml|binary)$").unwrap();
        assert!(re.is_match(fmt), "only support fmt in [json|yaml|binary]");
        py.allow_threads(move || self.root.dump(path, fmt))
    }

    #[args(fmt="\"json\"")]
    #[pyo3(text_signature = "(path, fmt)")]
    #[staticmethod]
    fn load(py: Python, path: &str, fmt:  & str) -> Self{
        let re = Regex::new("^(json|yaml|binary)$").unwrap();
        assert!(re.is_match(fmt), "only support fmt in [json|yaml|binary]");
        py.allow_threads(move || {
            Trie{
                root: trie::Trie::load(path, fmt)
            }
        })
    }
}

#[pymodule]
fn pytrie(_py: Python, m: & PyModule) -> PyResult<()>{
    m.add_class::<Trie>()?;
    Ok(())
}