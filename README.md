<div align="center">

# Liquid Rust 🦀

**Simple liquid type checker for Rust**

[![Rust](https://img.shields.io/badge/Built_with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![GitHub repo size](https://img.shields.io/github/repo-size/rcosta358/liquid-rust)

</div>

---


Very simple liquid type checker for Rust that performs straightforward compile-time checks for numerical constants using the `refine!` procedural macro.
Uses Logos and LALRPOP to parse the refinement string into an AST and encodes it into SMT formulas for validation through the Z3 SMT solver on constant values at compile-time.

The goal of this project is to get more familiar with liquid types and the Z3 SMT solver.

### Example

```rust
use liquid_rust::refine;

fn main() {
    let val1 = refine!("(_ > 0 && _ < 10) || _ == -1", -1); // between 0 and 10 or exactly -1
    let val2 = refine!("_ * 10 < 100", 9); // less than 100 when multiplied by 10
    let val3 = refine!("_ % 2 == 0 ? _ >= 0 : _ < 0", -7); // positive evens or negative odds
    let val4 = refine!("_ == 10 ? true : _ > 0 ? !(_ >= 10) : false", 5); // between 0 and 10

    println!("{}", val1);
    println!("{}", val2);
    println!("{}", val3);
    println!("{}", val4);
}
```

<br />

**Note:** This is not a practical project to be used, it is just a fun experiment to learn more about this topic.