# CS4414 Labs

Rust programming labs for CS4414. Clone this repository and work inside each lab's crate directory.

## 75-minute track (recommended for in-room labs)

Designed for students with programming background but no prior Rust. Install rustup before lab; run `cargo test` for grading.

| Lab | Crate | Topic |
|-----|-------|-------|
| 1 | `lab1-counter-stats-lite` | Types, enums, methods, `Option`, `Result`, encapsulation |
| 2 | `lab2-process-registry-lite` | Ownership, `Box`, borrowing |
| 3 | `lab3-process-registry-lite` | Traits, generics, iterators, `HashMap`, `dyn` size |

```bash
git clone https://github.com/HypexJuice/CS4414-Labs.git
cd CS4414-Labs/lab1-counter-stats-lite
cargo test
```

Read `ASSIGNMENT.md` inside each crate. Starters compile; tests fail until `todo!()` bodies are replaced.

## Full track (main sequence)

Monitor a machine across three labs: idle samples → process table → upgraded registry API.

| Lab | Crate | Topic |
|-----|-------|-------|
| 1 | `lab1-counter-stats` | Structs, enums, methods |
| 2 | `lab2-process-registry` | Ownership, encapsulation, `Box`, borrowing |
| 3 | `lab3-process-registry` | `HashMap`, errors, traits, iterators, removal |

## Requirements

- [rustup](https://rustup.rs/) (stable toolchain)
