# Queue

https://frontendmasters.com/courses/algorithms/queue/

## Takeaways

- queue is a linked list with constraints - values can only be added to the end,
  removed from the head, and viewed at the head
- constraints are what make data structures performant

## Notes

- people sometimes get pedantic about "is this a data structure, or is it an
  algorithm...?" Why not both...?
- a queue is a specific implementation of a linked list where data moves in a
  FIFO manner
- insertion in a queue requires appending a value to the end of the queue, and
  updating the reference to the tail:

  ```
      A -> B -> C -> D
      ^              ^
      |              |
    Head            Tail

      A -> B -> C -> D
                    [1]
                      \
                       E <- Tail
                             [2]

  1 - point D to E
  2 - point the tail to E
  ```

  We first maintain the linked list's links, and then update the tail:

  1. the current tail, D, can be found at `this.tail`. The tail doesn't point to
     anything, so `this.tail.next` is currently `null`

     ```
     this.tail.next = E
     ```

  1. we point the current tail's `.next` to the new value, E
  1. we then update the data structure's tail to point to E, too
     ```
     this.tail = E
     ```
     The value after D is E, and the tail is currently E, too. We may add new items
     to the list, but D will always point to E as long as D is in the queue

  These two operations are both constant-time

- popping from a queue happens at the head:

  ```
    A -> B -> C -> D
    ^              ^
    |              |
  Head            Tail

    A   B -> C -> D
   [2]  ^
        |
      Head
       [1]

  1 - set this.head to this.head.next
  2 - set old_head.next to null
  3 - return old_head.value
  ```

  These operations are all constant time

- a queue has highly constrained features:

  - values can be added to the tail
  - values can be popped popped from the head

  There's no traversing, or any other operations - the only operations that can
  be performed are constant time

- the operations on a queue are usually as follows:
  - enqueue - add a value
  - dequeue - pop a value
  - peek - look at the value at the head without popping it
- what makes many data structures performant is a deliberate _lack_ of features
