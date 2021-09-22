from typing import List


class Trie:

    def __init__(self):
        ...

    def insert(self, word: str):
        ...

    def remove(self, word: str):
        ...

    def remove_prefix(self, prefix: str):
        ...

    def search(self, word: str) ->List[str]:
        ...

    def search_prefix(self, prefix: str) -> List[str]:
        ...

    def find_all(self, prefix: str) -> List[str]:
        ...

    def get_all_words(self) -> List[str]:
        ...

    def clear(self):
        ...

    def dump(self, path):
        ...

    @staticmethod
    def load(path) -> Trie:
        ...
