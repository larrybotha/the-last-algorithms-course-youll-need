from dataclasses import dataclass
from typing import Generic, Optional, TypeVar

T = TypeVar("T")


@dataclass(kw_only=True)
class Node(Generic[T]):
    value: Optional[T] = None
    next: Optional["Node[T]"] = None


class Queue(Generic[T]):
    length: int = 0
    tail: Node[T]
    head: Node[T]

    def __init__(self):
        self.tail = Node()
        self.head = self.tail

    def enqueue(self, value) -> None:
        node = Node(value=value)

        self.tail.next = node
        self.tail = node
        self.head = self.tail if self.length == 0 else self.head
        self.length += 1

    def deque(self) -> T | None:
        node = self.head

        self.length = max(0, self.length - 1)
        self.tail = Node() if self.length == 0 else self.tail
        self.head = self.head.next if self.length > 0 and self.head.next else self.tail

        return node.value

    def __repr__(self):
        output = ""
        node = self.head

        while node:
            output += f"\tNode(value={node.value}, next="

        return output


if __name__ == "__main__":
    xs = [5, 7, 9]
    queue = Queue()

    for x in xs:
        print(f"Enqueuing {x}")
        queue.enqueue(x)

    assert queue.length == len(xs), "Queue length is not equal to array length"

    for i, x in enumerate(xs):
        print(f"\nDequeuing iteration {i}")
        value = queue.deque()
        print(f"Value is {value}")
        assert value == x, "Dequeued value is incorrect"
        assert queue.length == len(xs) - i - 1, "Queue length is incorrect"
