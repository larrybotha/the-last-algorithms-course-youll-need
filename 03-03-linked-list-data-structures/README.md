# Linked List Data Structures

https://frontendmasters.com/courses/algorithms/linked-list-data-structures/

## Takeaways

- linked lists have constant-time insertion and deletion

## Notes

- limitations of arrays:
  - ungrowable - they represent contiguous blocks of memory
  - unable to delete things - values may only be zero'd
  - unable to insert things - you can only write to indexes that already exist
- linked lists are data structures where every node points to the next node.
  Doubly-linked lists point in both directions
- each node is usually represented by some sort of container which:

  - holds the value at that node
  - holds a reference to the next node in the linked list:

    ```
    Node<T>:
      value: T
      next?: Node<T>

      A -> B -> C -> D
    ```

- singly-linked lists can only be walked forwards - as soon as a value has been
  walked beyond, there's no going back from the current position
- doubly-linked lists can be traversed in either direction from any node:

  ```
  Node<T>:
    value: T
    prev?: Node<T>
    next?: Node<T>
  ```

- the nodes of a linked list are usually heap-allocated - why...? Because linked
  lists are not represented by a contiguous space in memory - they are
  growable and shrinkable, and thus need to be heap-allocated to allow for their
  dynamic nature
- to insert a value into a linked list:

  ```
      A -> B -> C -> D

  [1] A -> B -> C -> D
        ^
        |
        F

  [2] A   B -> C -> D
       \ /
        F
  [3] A -> F -> B -> C -> D

  1 - we want to insert F after A
  2 - we need to make updates in 2 locations:
      - set A.next to F
      - set F.next to B
  3 - the items after A have effectively been shifted to the right
  ```

  This means we performed 2 constant-time operations. For a doubly-linked list
  we'd need to point A to F, F to A, F to B, and B to F, so 4 constant-time
  operations.

  Inserting in linked lists is _always_ constant-time!

- to delete values in linked lists:

  ```
  [1] A -> B -> C -> D
                x

  [2] A -> B   C   D
            \_____/

  [3] A -> B -> D

  1 - we want to remove C, which we got, somewhow
  2 - we need to do 2 things here:
      - set B.next to the value in C.next
      - unset C.next
  3 - C is no longer in the linked list
  ```

  Again, we have 2 constant-time operations, and similar to a doubly-linked
  list, we have 4 operations to complete a deletion:

  - set B.next to the value at C.next
  - set D.prev to the value at C.prev
  - unset C.next
  - unset C.prev

  Deleting values from linked lists is also a constant-time operation
