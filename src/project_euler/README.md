# Project Euler

Put any solutions to problems from [project euler](https://projecteuler.net/)
here.

# Guidelines

Please put your solution in a folder, following the `_<problem_id>` format.
Add `pub mod _<problem_id>;` into the `mod.rs` in the `project_euler`
directory. If there is only one solution, feel free to keep the solution in
`mod.rs`. The structure if there's one solution:

```
_3:
 mod.rs 
```

The structure for multiple solutions:
```
_3:
 mod.rs 
 <solution_name>.rs
 <other_solution_name>.rs
 <other_solution_name>.rs
```

If there are multiple solutions, also make sure to include them in the
respective `mod.rs` files.

Also provide a link and the number of the problem in a module level doc 
comment. (in the respective `mod.rs`) For example:

```rust
//! Larget Prime Factor
//! 
//! Problem #3.
//! https://projecteuler.net/problem=3
```
