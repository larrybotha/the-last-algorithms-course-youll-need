# Binary Search Algorithms

https://frontendmasters.com/courses/algorithms/binary-search-algorithm/

## Takeaways

- algorithms that half the dataset on each iteration are usually O(log N) or
  O(N log N)

## Notes

- an important question to ask about a dataset is 'is it ordered?'. Ordered
    datasets have advantages over unordered datasets
- binary search can be visualised like so:

    ```
    xs = [x0, x1, x2, ..., xn-3, xn-2, xn-1, xn]
    #     |       ^     ^
    #     |       |     |
    # ----|-------|--->[1]
    #     |       |
    #     |       |
    # ----|----->[2]
    #     |
    #     |
    # -->[3]
    ```

    1. we have N/2 elements to search
    2. we have N/4 elements to search
    3. we have N/8 elements to search

    At some point we'll be left with 1 element (for arrays >= length 1), and we've
    either found a match or not

   This is what we're after - we want the series of values to decrease until
   there's only 1 element remaining, i.e.

   ```
   N/2^k = 1
     => N = 2^k
     => log₂N = k
   ```

   Leaving us with a time complexity of O(log n)
