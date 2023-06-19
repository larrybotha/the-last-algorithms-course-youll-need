# Implementing Binary Search

- https://frontendmasters.com/courses/algorithms/implementing-binary-search/
- [BinarySearchList.ts](../katas/src/day1/BinarySearchList.ts)


## Takeaways

- the `[min, max)` included / excluded array now makes sense - the algorithm
    uses the _length of the array_ to calculate the mid index, even though we'd
    never use the length to access an item in the array in TypeScript.

    `[min, max)` refers specifically to the list of indices we're working with,
    and the `max` value is excluded value is never being evaluated, even when
    the length of the array is 1

## Examples

From this directory

- Typescript:
  ```shell
  $ cd kata && npx jest binarySearchList
  ```
- Rust:
  ```shell
  cargo run
  ```
- Python
  ```shell
  python src/main.py
  ```

## Notes

- we can use the length of the array as the upper bound because we don't
    actually use the index at that value - we only calculate the index of the
    middle value using that value

    This is why we use `[min, max)` with an exclusive upper bound - we never use
    the invalid value of the index at the upper, it's only part of our
    calculation
- Python's `match` statement needs a concrete value to evaluate - unlike the
    `switch` statement in Javascript, where expressions can be evaluated, Python
    accepts values that are more like enumerations
