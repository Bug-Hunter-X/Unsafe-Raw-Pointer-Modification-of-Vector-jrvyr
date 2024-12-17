# Unsafe Raw Pointer Modification of Vector in Rust

This repository demonstrates a potential issue in Rust when directly manipulating a vector's contents via a raw pointer.  The example shows how modifying a vector using `as_mut_ptr()` can easily lead to unexpected behavior if not carefully managed.  The solution offers a safer alternative that avoids this kind of raw pointer manipulation.

## Bug
The `bug.rs` file contains code that modifies a vector using a raw pointer obtained using `as_mut_ptr()`. Although this works in this simple example, it is unsafe and prone to errors in more complex scenarios.

## Solution
The `bugSolution.rs` file presents a safer way to modify the vector by using standard vector methods instead of raw pointers.