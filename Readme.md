A parser combinator library based on nightly rust.


## Tutorial

[Choose your parser combinator](https://docs.rs/pex/latest/pex/helpers/index.html)


## Technical features

- **Aggressive Nightly**: pex tries all the beneficial nightly features, adopting any features that can achieve compile-time evaluation, lazy evaluation or other features that are beneficial to efficiency.
- **`#![no_std]`**: pex can be used in no std environment.
- **Zero Copy**: If a parser returns a subset of its input data, it will return a slice of that input, without copying.
- **Zero Allocation**: pex is designed zero allocation, all operations try to [avoid allocation](#zero-allocation).

### Compile-time Evaluation

### Lazy Evaluation

### Zero Allocation

Pex does not cause allocation, unless you try to parse `a*` and `a+`, which returns `Vec<T>`.

And if you use the `arena partten`, such allocation can also be avoided.

First you need to return `Vec<()>`, [ZST](https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees) will not cause allocation.

Then use the pre-allocated arena tree structure (such as [indextree](https://crates.io/crates/indextree)) to record the parent-child relationship of nodes to avoid reallocation.

Memory allocation is not terrible, what is terrible is frequent fragmentary allocation.

### Error Reporting

pex provides offset to mark the error position.

Pex does not provide and will not provide lines and columns,
this is because the lines and columns are related to the encoding,
while offset has nothing to do with encoding.

For example, [LSP]() uses utf16, but rust uses utf8,

For pretty print the error message, you can use [miette](https://crates.io/crates/miette).