This repository demonstrates a common error in Rust: unsafe memory access. The `bug.rs` file shows code that attempts to access memory outside the bounds of a vector, leading to undefined behavior.  The `bugSolution.rs` file provides a safer alternative using bounds checking.