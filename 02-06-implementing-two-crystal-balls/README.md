# Implementing Two Crystal Balls

https://frontendmasters.com/courses/algorithms/implementing-two-crystal-balls/

## Takeaways

- using `Iterator::enumerate` in Rust and `enumerate` in Python are nice ways to
    avoid having to reference values by index when looping over iterables
- Rust has a few analogues to Python's slice operators:
  * `[num_to_skip::]` => `Iterator::skip(num_to_skip)`
  * `[:num_to_take:]` => `Iterator::take(num_to_take)`
  * `[::step_value]` => `Iterator::step_by(step_value)`
- Rust has `Iterator::find`, which works similarly to Javascript's `Array.find`,
    except that it returns an `Option` of the value if the predicate passes
  - This is analogous to `Iterator::filter` followed by `Iterator::next`
- Rust has `Iterator::position` which is analogous to Javascript's
    `Array.indexOf`, except that instead of returning the `-1` sentinel value,
    it returns an `Option<usize>`. `usize` cannot be negative, so this makes
    sense
- `Option::map_or` allows for neatly defining a way to map an `Option`, whilc
    providing a default if it evaluates to `None`. `Option::map_or_else` works
    similarly, except the default value is derived by executing a lambda

## Examples

- Typescript

  ```shell
  $ cd ../katas/src/day1 && npx jest crystal
  ```
- Rust:

    ```shell
    cargo run
    ```
- Python

    ```shell
    python src/main.py
    ```
