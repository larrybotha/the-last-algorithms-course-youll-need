# Linear Search & Kata Setup

https://frontendmasters.com/courses/algorithms/linear-search-kata-setup/

- [linear search kata](../katas/src/day1/LinearSearchList.ts)

## Takeaways

- visualising problems and algorithmic structures is a huge benefit to
    understanding them and knowing how to work with them

## Notes

- although Javascript has `.pop` etc. on arrays, for the sake of the course, and
    as a good practice for learning how these data structure work, we'll only
    use the `.length` property
- the simplest kind of search is a linear search... iterate over each item in an
    array, evaluating each to determine if its a given value, returning a
    boolean if the value is found or not. This O(N), because the worst case is
    that we traverse the entire array, N items, and find nothing

    ```
    xs = [x0, x1, ..., .xn]
    #     ^   ^         ^
    # -> [0]  |         |
    # -----> [1]        |
    # ...               |
    # ---------------->[n]

    search(xs, v)
    ```

    1. do we have a value on iteration `[0]`? If so, return it, else,
       contineue...
    2. do we have a value on iteration `[1]`? If so, return it, else,
       contineue...
    3. ...
    4. do we have a value on iteration `[n]`? If so, return it, else,
       contineue...
