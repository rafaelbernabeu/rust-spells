# rust-spells

## The Rust Programming Language

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