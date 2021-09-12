every value has a single owner that determines its lifetime
When the owner is freed(dropped), the owned value is dropped too. 

All data stored on the stack must have a known, fixed size. 
Data with unknown size at compile time or size that might change MUST be stored on the __heap__
The memory allocator funds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer (address of that location). 

Pushing to the stack is faster than allocating on the heap

Accessing data in the heap is slower than accesing data on the stack

## Ownership rules
* Each value in Rust has a variable that's called its owner
* There can only be 1 owner at a time
* When the owner goes out of scope, the value will be dropped

## References
They allow to refer to some value without taking ownership of it
Having references as function parameters -> borrowing

## Mutable references
You can only have 1 mutable reference to a particular piece of data in a particular scope
