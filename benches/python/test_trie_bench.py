# -*- coding:utf-8 -*-
import pytest
from pytrie import Trie
from pure_python_trie import TrieTree
from typing import List


def read_data(file_path: str) -> List[str]:
    with open(file_path, encoding='utf-8') as f:
        texts = []
        for line in f:
            texts.append(line.strip())
    return texts


def run_trie_insert(trie, texts):
    trie.clear()
    for text in texts:
        trie.insert(text)


def run_trie_search(trie, texts):
    for text in texts:
        trie.search(text)


def test_rust_insert_bench(benchmark):
    texts = read_data("../text_data.txt")
    trie = Trie()
    benchmark(run_trie_insert, trie, texts)


def test_rust_search_bench(benchmark):
    texts = read_data("../text_data.txt")
    trie = Trie()
    for text in texts:
        trie.insert(text)
    benchmark(run_trie_search, trie, texts)


def test_py_insert_bench(benchmark):
    texts = read_data("../text_data.txt")
    trie = TrieTree()
    benchmark(run_trie_insert, trie, texts, )


def test_py_search_bench(benchmark):
    texts = read_data("../text_data.txt")
    trie = TrieTree()
    for text in texts:
        trie.insert(text)
    benchmark(run_trie_search, trie, texts)


if __name__ == "__main__":
    pytest.main(['-s', '-q', "test_trie_bench.py"])
