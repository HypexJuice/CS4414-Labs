# Lab 3: Process Registry API

## Introduction

Upgrade the process registry from Lab 2:

| Lab 2 | Lab 3 |
|-------|-------|
| `Vec<Box<Process>>` | `HashMap<u32, Box<Process>>` keyed by PID |
| Index lookup with `get` | PID lookup with `get_by_pid` |
| `RegistryError::Full` only | `Full`, `DuplicatePid`, `NotFound` |
| No removal | `remove_by_pid` returns owned `Process` |

The starter [`src/lib.rs`](src/lib.rs) already includes `Process`, `ProcessState`, and `ProcessError` from Lab 2. Your work is the `HashMap` registry, the `Named` trait, iterators, and `remove_by_pid`.

Put all code in [`src/lib.rs`](src/lib.rs). Run `cargo test` when you are done.

## Setup

1. Install [rustup](https://rustup.rs/) (stable toolchain).
2. Clone the course labs repository and enter this crate:

```bash
git clone https://github.com/HypexJuice/CS4414-Labs.git
cd CS4414-Labs/lab3-process-registry
```

3. Run `cargo test`. The starter compiles; tests fail until you implement the registry API below.

## Part 1 — `Process` (provided)

The starter already defines `ProcessState`, `Process`, `ProcessError`, `new`, accessors, and `set_state` — the same API as Lab 2. Read that code before starting the registry.

## Part 2 — `Registry`

Private field:

| Field | Type |
|-------|------|
| `processes` | `HashMap<u32, Box<Process>>` |

Use the `HashMap` as the only store. No parallel `Vec`.

### `RegistryError`

| Variant | Meaning |
|---------|---------|
| `Full` | At capacity |
| `DuplicatePid(u32)` | PID already present |
| `NotFound(u32)` | No such PID |

Derive `Debug`, `PartialEq`, and `Eq`.

### `MAX_PROCESSES`

Public `const MAX_PROCESSES: usize = 8`, same as Lab 2.

### Registry methods

| Method | Behavior |
|--------|----------|
| `new` | Empty map |
| `len` | Entry count |
| `register` | At capacity → `Full`. Duplicate PID → `DuplicatePid`. Else insert and return `Ok(())`. Takes ownership of the `Process`. |
| `get_by_pid` | `&Process` or `NotFound` |

No panics on expected errors in your library code. Use `?`, `ok_or`, or `match`.

### `remove_by_pid`

On `&mut self`, takes a PID and returns `Result<Process, RegistryError>`. Missing PID → `NotFound`. Otherwise remove from the map and return the owned `Process`.

## Part 3 — `Named` trait

Public trait with one method on `&self` returning `&str`. Implement it for `Process`.

The starter includes a stub `impl Named for Process` with `todo!()`. Replace it in Milestone 2.

## Part 4 — Iterators

### `iter_by_state`

On `&self`, takes a `ProcessState` and returns `impl Iterator<Item = &Process>`. Yield processes in that state. Use `HashMap` iteration and adapters such as `filter`. Return `impl Trait`, not `dyn Iterator`.

### `running_pids`

On `&self`, returns `Vec<u32>` of all `Running` PIDs. Build from `iter_by_state` with `map` and `collect`.

## Worked example

```text
let mut reg = Registry::new();
reg.register(Process::new(1, "init".into())?)?;
reg.register(Process::new(42, "shell".into())?)?;

reg.get_by_pid(42)?.name() == "shell"
reg.register(Process::new(42, "dup".into())?)  // Err(DuplicatePid(42))

let mut init = reg.remove_by_pid(1)?;
init.set_state(ProcessState::Stopped);

reg.running_pids() == [42]  // shell still Running; init removed
```

## Rules

1. Validation on `Process`; storage rules on `Registry`.
2. `HashMap` is the only store. Fallible methods return `Result`. `iter_by_state` returns `impl Iterator`.
3. Allowed: everything from Lab 2 plus custom traits, `HashMap`, `impl Trait`, iterator adapters, `?`, `ok_or`, `Result::map`.
4. Do not use in graded code: `dyn Trait`, `Rc`, `Arc`, threads, `async`, external crates, `unsafe`, I/O, `std::error::Error`.
5. Your library code must not panic on expected error paths.

## Milestones

| Milestone | You implement | Check with |
|-----------|---------------|------------|
| M1 | `Registry`, `RegistryError`, `register`, `get_by_pid`, `len` | `cargo test m1_` |
| M2 | `Named` for `Process` | `cargo test m2_` |
| M3 | `iter_by_state`, `running_pids` | `cargo test m3_` |
| M4 | `remove_by_pid` | `cargo test m4_` |

When M1–M4 pass, run `cargo test`.

## If you finish early

- Add `find_by_name` taking `&str` and returning `Result<&Process, RegistryError>`.
- Swap the map for `BTreeMap` and iterate PIDs in sorted order.
- Add a `transition` state machine where `Stopped` is terminal.
- Add an `Observer` trait and `Vec<Box<dyn Observer>>` on `Registry`. Compare `size_of::<&dyn Observer>()` to `size_of::<&Process>()` — fat pointer vs thin pointer.
