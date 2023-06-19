# The Two Crystal Ball Problem

https://frontendmasters.com/courses/algorithms/two-crystal-balls-problem/

> Given two crystal balls that will break if dropped from a high enough
  distance, determine the exact spot they would break in the most optimised way

This is an actual interview question ThePrimeagen actually got. It's often given
in a more specific manner, e.g. "Given a building with 100 floors...". This is
the generalised version

## Takeaways

- walking forward by 1 step is linear, but depending on the problem, walking
    forward by 1 step may be an unnecessary constraint

## Notes

- This problem boils down to a list of boolean values. The values at lower
    indices are `false`, and at some point they become `true`

    ```
    [false, false, false, ..., true, true, true]
    0                                          N
                          [*]

    * - this is what we're looking for - what is the index where the first value
      is true
    ```
- we have 2 crystal balls. This means that we could do a linear search, but we
    are "allowed" to break one to short-circuit the search

    Performing a linear search would work, but it's slow, so what can be done to
    utilise the 2 balls to find the breaking point more efficiently...?
- we know that we have an ordered array, i.e.

    ```
        [false, false, false, ..., true, true, true]
    =>  [0, 0, 0, ..., 1, 1, 1]
    ```

    This means that binary search is a potential approach we can use to solve
    the question.

    The problem here, however, is that we can eliminate the first half of the
    array using a binary search, but then we'd have to go back to using a linear
    search:

    ```
    [x0, x1, x2, ..., xn-2, xn-1, xn]
                  x
                  |
    [1] -----------

    [x0, x1, x2, ..., x(n/2)-2, x(n/2)-1, x(n/2)]
    [2] -> -> -> ->

    1 - we could find the mid-value in the list - if the ball breaks, we know we
      need to go back to the beginning of the list
    2 - we go to the beginning of the list... and then we have to do a linear
      search, similar to using the linear search without the binary search
    ```

    With this approach, we have O(1/2n + logn), which then simplifies to O(n) - Big
    O uses the worst-case to describe the time complexity. It's more efficient
    if we were _comparing_ O(n) algorithms, but is there a better way...?
- instead of halving the list, as with a binary search, or walking forward a
    single value at a time, using a binary search, what we can instead do is
    walk forward in the array, but do so in increments larger than in linear
    search, but smaller than in a binary search
- let's try this with `sqrt(n)`

    ```
    [                                                                       ]
    0                                                                       N
                x         x         ^         -         -         -         -
                |         |         |
    [1]----------         |         |
                          |         |
                          |         |
    [2]--------------------         |
                                    |
                                    |
    [3]------------------------------
                          [         ]
    [4]                   <--------->

    1 - we walk sqrt(n) from the first position, and the ball doesn't break
    2 - we walk sqrt(n) from the next position, and the ball doesn't break
    3 - we walk sqrt(n) from the next position, and the ball _does_ break
    4 - we go back to the last known good position, and walk linearly from
      there. The max we can walk here is sqrt(n)!
    ```

    We can only jump a maximum of `sqrt(n)` times, and when the ball breaks, the
    most we can walk from the last known good position to the breaking position
    is also `sqrt(n)`

    Thus, we have a time complexity of O(sqrt(n) + sqrt(n)), which then simplifies
    to O(sqrt(n))!
