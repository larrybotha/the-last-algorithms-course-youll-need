# Big O Time Complexity

https://frontendmasters.com/courses/algorithms/big-o-time-complexity/

## Takeaways

- growth is always considered with respect to the size of the input
- constants are always dropped in Big O notation
- Big O denotes _worst case_ for measuring time / memory consumption

## Notes

- Big O is a generalised way to categorise and understand the time and memory
    requirements of algorithms based on their inputs, e.g. linear vs quadratic
    growth
- Big O describes 'how fast does computation or memory grow?'
  * 'growth' here is with respect to inputs
- the simplest way to look for complexity in an algorithm is to look for loops
- constants are always dropped in Big O, e.g.:

    ```typescript
    // despite 2 loops, the time complexity is still O(n)
    function myFunc() {
      const xs = [1, 2, 3, 4, 5];

      for (let i = 0; i < xs.length; i++) {
      // do something
      }

      for (let i = 0; i < xs.length; i++) {
      // do another thing
      }
    }
    ```

    The general idea of the time / memory is what's important, not the actual
    time
- O(N) may be generally faster than O(N^2) doesn't mean it's always faster - the
  size input can affect the actual performance
- Big O considers worst case - even if we don't iterate over an entire list of
    items, the worst case is that we _may_ have to... thus O(N)
- O(N^2) can be identified by a nested loop
- O(N^3) can be identified by a doubly-nested loop
- quick sort is an O(n log n) algorithm
- binary search is an O(log n) algorithm - the scope of each search is halved,
    which rapidly decreases the number of items in the input to evaluate
