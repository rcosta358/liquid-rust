# Liquid Rust 🦀

Very simple liquid type checker for Rust that performs straightforward compile-time checks using the `refine!` procedural macro.
Uses Logos and LALRPOP to parse the refinement string into an AST and encodes it into SMT formulas for validation through the Z3 SMT solver on constant values at compile-time.

The goal of this project is to get more familiar with liquid types and the Z3 SMT solver.

### Example

```rust
fn main() {
    let val = refine!("_ > 0 && _ < 10", -1); // compile error
    println!("{}", val);
}
```