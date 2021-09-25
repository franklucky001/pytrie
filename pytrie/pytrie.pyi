from typing import List


class Trie:

    def __init__(self):
        ...

    def __len__(self):
        """
        :return: trie length
        """
        ...

    def insert(self, word: str):
        """
        :param word:
        :return:
        """
        ...

    def remove(self, word: str):
        """
        @function remove word in trie if exists
        :param word:
        :return:
        """
        ...

    def remove_prefix(self, prefix: str):
        """
        @function remove word with prefix
        :param prefix:
        :return:
        """
        ...

    def search(self, word: str) ->List[str]:
        """
        :param word:
        :return: whether word in trie or not
        """
        ...

    def startswith(self, prefix: str) -> List[str]:
        """
        :param prefix:
        :return: whether trie has word with prefix or not
        """
        ...

    def prefix_count(self, prefix: str) -> int:
        """
        :param prefix:
        :return: number of word with prefix
        """
        ...

    def find_all_prefix(self, prefix: str) -> List[str]:
        """
        :param prefix: word prefix
        :return: all words with prefix
        """
        ...

    def find_all(self) -> List[str]:
        """
        :return: all words in trie
        """
        ...

    def clear(self):
        ...

    def dump(self, path: str, fmt: str = "json"):
        """
        :param path: save path
        :param fmt: json|yaml|binary
        :return: None
        """
        ...

    @staticmethod
    def load(path: str, fmt: str = "json") -> Trie:
        """
        :param path: trie cache path
        :param fmt: json|yaml|binary
        :return: Trie
        """
        ...
