### Building Blocks 1

#### **The Manifest**

The `Cargo.toml` file for each package is called its *manifest*. It contains **metadata** that is needed to compile the package. *(Reference: [Manifest Doc](https://doc.rust-lang.org/cargo/reference/manifest.html))*

#### **Environment Variables**

You can override some environment variables to change Cargo's behavior on your system.

The programmer can get the environment variable's value by using `env!() / env::var() `.

The env var make sense to the **compile and execute** for the Rust package.

*Reference: [Env Doc](https://doc.rust-lang.org/cargo/reference/environment-variables.html)*

#### **Documentation**

**C-Example:** All items have a rustdoc example to exercise the functionality for the code user. The goal is to show *why someone would want to use the item* or to show *how to use the item*.

**C-Failure**

- Error conditions should be documented in an "Errors" section. 
- Panic conditions should be documented in a "Panics" section. 
- Unsafe functions should be documented with a "Safety" section that explains all invariants that the caller is responsible for upholding to use the function correctly.

**C-Link & C-Metadata**

**C-Hidden:** Rustdoc is supposed to include everything users need to use the crate fully and *nothing more*.  Don't show up the details of implementation to the users.

*Reference: [Rust API: Doc](https://rust-lang-nursery.github.io/api-guidelines/documentation.html)*

























