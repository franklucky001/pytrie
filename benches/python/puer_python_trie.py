# -*- coding: utf-8 -*-

class TrieTree:

    def __init__(self):
        self._root_node = {}

    def insert(self, word):
        node = self._root_node
        for ch in word:
            child = node.get(ch, {})
            node[ch] = child
            node = child
        node['is_word'] = True

    def remove(self, word):
        pass

    def common_search(self, chars, search_prefix):
        node = self._root_node
        for ch in chars:
            child = node.get(ch, {})
            if not child:
                return False
        if search_prefix:
            return True
        else:
            return node.get('is_word', False)

    def search(self, word):
        return self.common_search(word, False)

    def search_prefix(self, prefix):
        return self.common_search(prefix, True)

    def clear(self):
        self._root_node = {}