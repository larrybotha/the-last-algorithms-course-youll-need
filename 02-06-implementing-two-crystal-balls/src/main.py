from math import floor, sqrt
from textwrap import dedent
from typing import Sequence, Tuple


def two_crystal_balls(xs: Sequence[bool]) -> Tuple[int, int]:
    sqrt_xs = floor(sqrt(len(xs)))
    indices = (x * sqrt_xs for x in range(0, len(xs)))
    safe_index, result, iterations = 0, -1, 0

    for i in indices:
        iterations += 1
        if xs[i]:
            break

        safe_index = i

    for i in range(safe_index, safe_index + sqrt_xs):
        iterations += 1
        if xs[i]:
            result = i
            break

    return result, iterations


if __name__ == "__main__":
    max_ball_height = 1234
    xs = [x > max_ball_height for x in range(0, 10000)]

    index, iterations = two_crystal_balls(xs)

    print(
        dedent(
            f"""
            With max ball height at {max_ball_height},
            index {index} found after {iterations} iterations
        """
        )
    )
