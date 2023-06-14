# Pseudo Code Binary Search

https://frontendmasters.com/courses/algorithms/pseudo-code-binary-search/

## Takeaways

- always eliminate a checked value when evaluating a subset - whether it's the
    upper or lower bound, it doesn't really matter, as long as at least one of
    the subsets is always eliminating the checked value

    e.g. choose an approach and stick with it generally, such as inclusive min,
    exclusive max:

    ```
    [min, max)
    ```
- the `-1` returned by `Array.indexOf` is called a _sentinel value_

## Notes

- we know:
  * we're going to halve the array
  * we're going to look on one side or the other
- we could describe this as follows:
    - we're searching for a value in a sorted array
    - we have a maximum and minimum for that array
    - we need to find the middle value of this array, which can be described as
        `⌊min + (max - min) / 2⌋`

        ```
        [x0, x1, x2, ..., xn-2, xn-1, xn]
                      ^
                      |
        midpoint ------
        ```

    - conditions:
      * `midpoint == value` => `return true`
      * `value > midpoint`  => `min = midpoint + 1`
      * `else`              => `max = midpoint`

      ```
      [x0, x1, x2, ..., xn-2, xn-1, xn]
      [    2    ]  [1]  [      3      ]

      1 - midpoint equals value, return it
      2 - midpoint is gt value, look at first half of array
      3 - otherwise, look at second half of array
      ```

      By using `midpoint + 1` when `midpoint > value` we ensure that we don't
      evaluate the midpoint again.

      This means we'll evaluate the arrays with an inclusive min, and exclusive
      max:

      ```
      [min, max)
      ```

      e.g.

      ```
      is 9 in the array:

         [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                      ↑
                      m
            ⌊0 + (10 - 0) / 2⌋ = x5
      =>                [5, 6, 7, 8, 9] # exclude 4
                               ↑
                               m
                      ⌊0 + (5 - 0) / 2⌋ = x2
      =>                      [7, 8, 9] # exclude 6
                                  ↑
                                  m
                          ⌊0 + (3 - 0) / 2⌋ = x1
      =>                            [9] # exclude 8
      => true
      ```

      It's important to ensure we don't evaluate the same value again! Whether
      we do eliminate it at the min or the max doesn't matter, as it'll be
      eliminated one way or another - what's important is that it is eliminated
      somewhere!
    - base case: `min >= max`        => `return false`

        At some point the min and max of the array will equal each other - at
        this point we've exhausted the array, and there's nothing left to
        evaluate - the value couldn't be found
    - continue evaluating conditions until we reach the base case
