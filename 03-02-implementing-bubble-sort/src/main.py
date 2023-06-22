from typing import List


def bubble_sort(xs: List):
    length = len(xs)

    for i in range(0, length):
        for j in range(0, length - 1 - i):
            if xs[j] > xs[j + 1]:
                xs[j], xs[j + 1] = xs[j + 1], xs[j]


def assert_order(xs: List):
    if len(xs) == 0:
        return

    for i in range(0, len(xs) - 1):
        assert xs[i] <= xs[i + 1]


if __name__ == "__main__":
    xs = [2, 1, 123, 4, 613, 23]
    ys = ["Z", "foo", "1", "A", "quux", "-1", "a"]

    for name, zs in [("xs", xs), ("ys", ys)]:
        print(f"{name} before: {zs}")
        bubble_sort(zs)
        assert_order(zs)
        print(f"{name} after: {zs}\n")
