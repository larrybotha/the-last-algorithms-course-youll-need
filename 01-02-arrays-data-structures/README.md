# Arrays Data Structures

https://frontendmasters.com/courses/algorithms/arrays-data-structure/

## Takeaways


## Notes

- properties of arrays:
  * occupy a contiguous memory space
  * have a fixed size
- by specifying a type and a size, you are describing to the machine how to
    'walk' the memory space. e.g.
  * for this contiguous memory space, where each item is allocated a chunk of
      memory of N bytes (e.g. we have 32bit int)
  * multiply the value of the memory allocation by the index I've given you
  * return the value at that location in memory
- `ArrayBuffer` in Javascript is similar to Rust's array in some ways:
  * it is transferrable, which means that it can only be used in one context at
      a time. Accessing an `ArrayBuffer` in another context will throw once its
      been transferred
  * the size is fixed at instantiation
  * using `Uint8Array` and `Uint16Array` etc. creates a _view_ into the array
      buffer. We are indicating that we want to work with the memory under
      certain constraints, e.g. 8-bits at a time, or 16-bits at a time. By doing
      this, we are changing the size of the steps we walk when we say we want
      the data at index `x` - the same index value will be at different memory
      locations given different sizes of memory to work with in the array
- operations on arrays:
  * insertion - insertion is not necessarily insertion - it's more useful to
      think of it as _replacement_ - you _replace_ values at arrays indices, and
      arrays are initialised with some sort of default values
      - e.g. for inserting a value `x` at index 2 of an 8-bit integer array,
          you're telling the machine to:
        * move 16-bits forward from the starting memory location of the array
        * replace the contents up until 23-bits into the array

      Because of this calculation, using the array's location memory, the size
      of the data in the array, and the index at which we want to insert
      something, we have a time complexity of O(1)
  * deletion - the same rules apply for deletion, because deletion is also
      actually a replacement; there is no change to the size of an array, only
      the values of the data stored at a specific location in memory
  * accessing - accessing is also O(1)
