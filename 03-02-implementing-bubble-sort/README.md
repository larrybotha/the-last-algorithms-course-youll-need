# Implementing Bubble Sort

https://frontendmasters.com/courses/algorithms/implementing-bubble-sort/

## Takeaways

- Rust has a `slice::swap` method for swapping values at given indices in slices
- functions that accept vectors can be made more generic by accepting
  `[T]` rather than `Vec<T>`, e.g.:

  ```rust
  fn do_the_thing<T: SomeTrait>(xs: &[T]) {}

  // vs

  fn do_the_thing<T: SomeTrait>(xs: &Vec<T>) {}
  ```

  The first signature allows for passing in arrays, slices, or vectors, while
  the second only accepts vectors

- `PartialOrd` needs to be used when comparing values of types that are not
  truly comparable, such as `f32::NAN`

  If you know you'll only ever compare values of types that are _always_
  comparable, such as strings or integers, then one can use `Ord`

- the inner loop's indexes are easier to work with than trying to determine how
  the outer loop's index works with the internal index
- ChatGPT, as well as the _Common-Sense Guide to Data Structures and Algorithms_
  book recommended in [../00-introduction](../00-introduction), both take the
  `is_swapped` approach to implementing bubble sort. This works, but results in
  a full N² number of iterations.

  This can be improved by making the following observations:

  - the array only needs to be evaluated until the `N - 1th` position, since the
    second last value will always be evaluated against the last
  - every iteration moves the largest unsorted value to its final position

  we know that for every iteration we need to traverse up until the most
  recently sorted value, or `N - i - 1` where `i` is the current outer loop's
  iteration, which results in N(N + 1) / 2.

  If we were to compare both algorithms within an O(N²) time complexity, this
  approach would be significantly more efficient.

  e.g. for an array containing 100 elements we have:

  ```
    N²           => 10 000 iterations
    N(N + 1) / 2 => 5 050  iterations
  ```

- accessing items in an array is always constant time, as is settings values at
  indexes in an array. All of the work done during swapping is constant time,
  and has no effect on the time complexity of the overall algorithm

## Examples

- Typescript

  ```shell
  $ cd ../katas/src/day1 && npx jest bubble
  ```

- Rust:

  ```shell
  cargo run
  ```

- Python

  ```shell
  python src/main.py
  ```
