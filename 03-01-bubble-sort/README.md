# Bubble Sort

https://frontendmasters.com/courses/algorithms/bubble-sort/

## Takeaways

- bubble sort moves the largest unsorted value to the end of the list on every
    iteration
- bubble sort has a time complexity of O(N²)

## Notes

- the definition of a sorted array is:

    For every item `Xn` in the array `Xn <= Xn+1`

    The converse is then useful - an array is _not_ sorted if there exists any
    `Xn+1` s.t. `Xn+1 < Xn`
- bubble sort comes down to the following:
  - start at the zeroth index of the array
  - iterate over each item, comparing the next item to the current item
  - if the next item is smaller than the current item, swap them

  ```
  [ 1,  3,  7,  4,  2 ]
    ^   ^
    |   |
  ---[1]-

  [ 1,  3,  7,  4,  2 ]
        ^   ^
        |   |
  -------[2]-

  [ 1,  3,  7,  4,  2 ]
            ^   ^
            |   |
  -----------[3]-

  [ 1,  3,  4,  7,  2 ]
            <--->
             [4]

  [ 1,  3,  4,  7,  2 ]
                ^   ^
                |   |
  ---------------[5]-

  [ 1,  3,  4,  2,  7 ]
                <--->
                 [6]

  [ 1,  3,  4,  2,  7 ]
    ^   ^
    |   |
  ---[7]-

  ...

  1 - is 1 <= 3? Yes, do nothing
  2 - is 3 <= 7? Yes, do nothing
  3 - is 7 <= 4? Nope
  4 - switch 7 and 4
  5 - is 7 <= 2? Nope
  6 - switch 7 and 2
  7 - start again at zeroth index, evaluating up until before the last sorted
    value, stopping when the length of the final array we evaluate is 1
  ```
- every array of length is sorted - this is a potential base-case for
    terminating the loop
- every iteration of bubble sort moves the largest unsorted value to its final
  position

    This means that for every iteration, we only need to evaluate the array up
    until, but not including, the last sorted value towards the end of the array

    e.g. for an array of length `N >= 2` we'll have the following number of
    iterations:

    ```
    N + (N - 1) + (N - 2) + ... + 2 + 1
    ```

    Reorganising this, we get the following:

    ```
        N + (N - 1) + (N - 2) + ... + (N - N + 2) + (N - N + 1)
            [_____]                   [_________]
               |______      _______________|
                     |      |
    => (N + (N - N + 1)) + ((N - 1) + (N - N + 2)) + ... + ...
    => (N + 1)           + (N + 1)                 + ... + ...
    => N(N + 1) / 2
    ```

    This has a time complexity of O(N²)!
- This is analogous to a counting problem:

    > What is the sum of the first 100 integers?

    We know that:

    - 1 + 100 = 101
    - 2 + 99 = 101
    - ...
    - 50 + 51 = 101

    Therefore, we have 50 * 101, or more generally:

    ```
        (N / 2) * (N + 1)
    => N(N + 1) / 2
    ```
