# rust-spells

## The Rust Programming Language

### Ownership

 - Each value in Rust has a variable that’s called its owner.
 - There can only be one owner at a time.
 - When the owner goes out of scope, the value will be dropped.

### Borrowing

 - At any given time, you can have either one mutable reference or any number of immutable references.
 - References must always be valid.

### Data races
A data race is similar to a race condition and happens when these three behaviors occur:

 - Two or more pointers access the same data at the same time.
 - At least one of the pointers is being used to write to the data.
 - There’s no mechanism being used to synchronize access to the data.

 ### Strings
 - Rust uses a deref coercion, which here turns &String into &String[..]
 - There are actually three relevant ways to look at strings from Rust’s perspective: 
    - as bytes, 
    - scalar values, 
    - grapheme clusters.

### Generics
```
Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.
Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. 
Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
```

### Iterators

 - Although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself.

 - Iterators are one of Rust’s zero-cost abstractions.

 - Rust knows that there are X iterations, so it “unrolls” the loop. Unrolling is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop. All of the coefficients get stored in registers, which means accessing the values is very fast. There are no bounds checks on the array access at runtime.

 - Currently all for loops with integer literal bounds will be unrolled.


[Docs - unroll](https://docs.rs/unroll/0.1.5/unroll/) \
[Comparing performance loops vs iterators](https://doc.rust-lang.org/book/ch13-04-performance.html?highlight=zero,cost#comparing-performance-loops-vs-iterators)