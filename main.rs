block.py
chain.py
README.md
import hashlib

class Block:
    def __init__(self, index, data, prev_hash):
        self.index = index
        self.data = data
        self.prev_hash = prev_hash
        self.hash = self.calculate_hash()

    def calculate_hash(self):
        value = f"{self.index}{self.data}{self.prev_hash}"
        return hashlib.sha256(value.encode()).hexdigest()
