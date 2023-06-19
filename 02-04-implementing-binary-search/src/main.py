import math
from typing import Any, Sequence


def binary_search(haystack: Sequence, needle: Any) -> bool:
    min_index = 0
    max_index = len(haystack)
    result = False

    while min_index < max_index:
        mid_index = math.floor(min_index + (max_index - min_index) / 2)
        mid_value = haystack[mid_index]

        if mid_value == needle:
            result = True
            break
        elif mid_value < needle:
            min_index = mid_index + 1
        else:
            max_index = mid_index

    return result


if __name__ == "__main__":
    xs = [1, 2, 6, 8, 234, 24524, 123157]
    search_values = [6, 12, 24524]

    for value in search_values:
        result = binary_search(xs, value)
        print(f"is {value} in {xs}: {result}")
