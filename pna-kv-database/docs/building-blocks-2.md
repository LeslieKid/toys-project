### Building Blocks 2

#### Error Handling in Rust

`panic` for unrecoverable failure & `Result<T, E>` for recoverable failure 

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `?` operator for *error propagation*. The `?` can be used both in `Result` and `Option`. *(similar to the match)*

pattern matching by using `match` for case analysis

The `unwrap / expect` method abstract away the case analysis `panic!()`

`unwrap_or()` & `and_then()` & `or_else()` for `Option<T>`

`filter` filters the `Option` accoring to a closure

`Result<T, E>` is a richer version of `Option<T>`

`map` and `map_err` can change the value in the `Some()` or `Ok()` or `Err()`

Application Scenario of `panic!()`

- In examples and quick ’n’ dirty code

- When panicking indicates a bug in the program, like `assert!()` failing.

**use good judgment**: There are *trade-offs* to all things

`ok_or(option: Option<T>, err: E)` & `ok_or_else`: convert `Option` to `Result`

**归一化错误类型**

- `Box<dyn Error>`（不限制错误类型）
- self-define error type: take use of `enum`, converting between two diff error types by implementing `From` trait. `impl From<std::Error> for MyError {}` （flexible but complex）
- ***use `thiserror`: simplify the error handling***
- ***use `anyhow`:*** 如果你想要设计自己的错误类型，同时给调用者提供具体的信息时，就使用 `thiserror`，例如当你在开发一个三方库代码时。如果你只想要简单，就使用 `anyhow`，例如在自己的应用服务中。

*Reference: [rust-error-handling](https://blog.burntsushi.net/rust-error-handling/)*

#### Module `std::collections`

Rust's collections can be grouped into four major categories: *Sequences(`Vec/VecDeque/LinkedList`), Maps(`HashMap/BTreeMap`), Sets(`HashSet/BTreeSet`), Misc(`BinaryHeap`)*

*Key: When should you use which collection ?*

*Reference: [std::collections doc](https://doc.rust-lang.org/std/collections/)*

#### Module `std::io`

Traits, helpers, and type definitions for core I/O functionality

The most core parts of this module is `Read()` trait for input and `Write()` trait for output.

Two important traits: `Seek` & `BufRead`, which are builted on the top of `Read`, indicating how to read.

Two Structs: `BufReader` & `BufWriter`, which wrap reader and writer.

Standard input and output: `io::stdin & io::stdout` (stdout is less common something like `println!()`)

`io::Result`: as the return type for the `std::io` functions that may cause an error.

*Reference: [std::io doc](https://doc.rust-lang.org/std/io/)*

#### Serde

Serde is a framework for **ser**ializing and **de**serializing Rust data structures efficiently and generically.













