<div align="center">

# Liquid Rust 🦀

**A toy liquid type checker for Rust**

[![Rust](https://img.shields.io/badge/Built_with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![GitHub repo size](https://img.shields.io/github/repo-size/rcosta358/liquid-rust)

</div>

---

Very simple liquid type checker for Rust that performs straightforward compile-time checks for integer literals.
Uses Logos and LALRPOP to parse the refinement string into a tiny AST and translates it into SMT formulas using the Z3 SMT solver. 

### Examples

With the procedural macro `refine!`, it's possible to check if a value satisfies a refinement condition at compile-time:

```rust
use liquid_rust::refine;

fn main() {
    let val1 = refine!("(_ > 0 && _ < 10) || _ == -1", -1); // between 0 and 10 or exactly -1
    let val2 = refine!("_ * 10 < 100", 9); // less than 100 when multiplied by 10
    let val3 = refine!("_ % 2 == 0 ? _ >= 0 : _ < 0", -7); // positive evens or negative odds
}
```

It is also possible to annotate constants with the `#[refinement("...")]` attribute which wraps the constant in a `refine!` call:

```rust
use liquid_rust::refinement;

#[refinement("_ > 0")]
const X: i32 = 1;
```

### Limitations
- Support for integer literals only
- Checks made only at variable definition
- Very limited set of operations

<br />

This is not intended for production use, as it is not useful at all - it is just a fun experiment to learn more about this topic.