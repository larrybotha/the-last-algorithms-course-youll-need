# Linked List Complexity

https://frontendmasters.com/courses/algorithms/linked-list-complexity/

## Takeaways

- any time you perform an O(n) operation, there's probably a better way to do it

## Notes

- linked list time complexity differs depending on the action being performed:

  - insertion and deletion are constant-time, but depending on where you are in
    the list, and what types of references to different locations in the list,
    there could be different implications

    e.g. if the linked list has a concept of _head_, then inserting or
    deleting at the head would be constant time

    Generally speaking, though, if we're not at an extremity of the list,
    there could be a costly O(n) / linear traversal of items to get to where
    we want to perform the operation

- generally, deletion or insertion at the front or end of the list is very
  fast; as soon as we move towards the middle is where things become more
  complex, and we may be subject to a less efficient algorithm
- linked lists are a foundational topic in data structures and algorithms.
  Understanding how linked lists work forms the basis for working effectively
  with trees, graphs, and other more complex structures
- the implementation of a linked list depends on what you're using it for - if
  you need to be able to quickly delete values at the end of the list, you'd
  implement the linked list in such a way that it holds a reference to the
  last value so that you can perform constant-time operations there
- every linked list is a graph
