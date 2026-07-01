# Lab 2: Process Registry

## Introduction

Lab 1 summarized CPU idle samples. Now track the processes on the machine.

Implement a process table. Each process has a PID, name, and state: `Running`, `Sleeping`, or `Stopped`. Store processes in a `Registry` backed by `Vec<Box<Process>>`.

Lab 1 used public struct fields. Here the fields are private; use methods to read and update them.

Put all code in [`src/lib.rs`](src/lib.rs). Run `cargo test` when you are done.

## Setup

1. Install [rustup](https://rustup.rs/) (stable toolchain).
2. Clone the course labs repository and enter this crate:

```bash
git clone https://github.com/HypexJuice/CS4414-Labs.git
cd CS4414-Labs/lab2-process-registry
```

3. Run `cargo test`. The starter code will not compile until you add the types below.

## Part 1 — `ProcessState` and `Process`

Define a public enum named `ProcessState` with exactly three variants:

| Variant | Meaning |
|---------|---------|
| `Running` | Process is actively executing |
| `Sleeping` | Process is blocked or waiting |
| `Stopped` | Process has been terminated |

Derive `Copy`, `Clone`, `Debug`, `PartialEq`, and `Eq`.

Define a public struct named `Process` with private fields:

| Field | Type |
|-------|------|
| `pid` | `u32` |
| `name` | `String` |
| `state` | `ProcessState` |

Tests cannot build a `Process` with a struct literal from outside your crate. Expose data through the methods below.

### `ProcessError`

Public enum with one variant: `EmptyName`. Derive `Debug`, `PartialEq`, and `Eq`.

### `new`

`u32` PID and owned `String` name → `Result<Process, ProcessError>`.

- Empty name → `Err(ProcessError::EmptyName)`.
- Otherwise → `Ok` with state `Running`.

### Accessors

On `&self`, return the PID as `u32`, the name as `&str`, and the current `ProcessState`. The name method should borrow from the internal `String` without allocating a copy.

### `set_state`

On `&mut self`, set `state`.

## Part 2 — `Registry`

Define a public struct named `Registry` with one private field:

| Field | Type |
|-------|------|
| `processes` | `Vec<Box<Process>>` |

Each `Box<Process>` owns one heap-allocated `Process`. The `Vec` owns the boxes.

### `RegistryError`

Public enum with one variant: `Full`. Derive `Debug`, `PartialEq`, and `Eq`.

### `MAX_PROCESSES`

Public `const MAX_PROCESSES: usize = 8`. Use `const`, not `static mut`.

### `new`

Returns an empty `Vec`.

### `len`

On `&self`, returns the number of stored processes.

### `register`

On `&mut self`, takes an owned `Process` by value:

- At capacity → `Err(RegistryError::Full)` without changing the registry.
- Otherwise → move into `Box`, push, return `Ok(())`.
- After success, the caller no longer owns that `Process`.

### `get` and `get_mut`

On `&self`, `get` takes a `usize` index and returns `Option<&Process>`.

On `&mut self`, `get_mut` takes a `usize` index and returns `Option<&mut Process>`.

References borrow from the registry. You do not need explicit lifetime annotations.

Look up processes by index only in this lab. Lab 3 adds PID-keyed lookup with a `HashMap`.

## Worked example

```text
let mut registry = Registry::new();

let init = Process::new(1, "init".into())?;
let shell = Process::new(42, "shell".into())?;
let mut logger = Process::new(99, "logger".into())?;
logger.set_state(ProcessState::Sleeping);

registry.register(init)?;
registry.register(shell)?;
registry.register(logger)?;
```

| Check | Expected |
|-------|----------|
| `registry.len()` | `3` |
| `registry.get(1).unwrap().name()` | `"shell"` |
| `registry.get(2).unwrap().state()` | `ProcessState::Sleeping` |

After `registry.get_mut(2).unwrap().set_state(ProcessState::Stopped)`:

| Check | Expected |
|-------|----------|
| `registry.get(2).unwrap().state()` | `ProcessState::Stopped` |
| `registry.get(0).unwrap().pid()` | `1` |

## Rules

1. Validation on `Process`; storage on `Registry`. No extra free functions.
2. `register` takes `Process` by value. Store in `Vec<Box<Process>>`. Lookups return `Option` with references tied to `&self` or `&mut self`.
3. Allowed: everything from Lab 1 plus `Vec`, `String`, `Option`, `Result`, `Box`, `const`, and derives `Copy`, `Clone`, `PartialEq`, `Eq`.
4. Do not use: `Rc`, `Arc`, `HashMap`, `BTreeMap`, custom traits, I/O, external crates, `unsafe`, or threads.
5. Every `Process` passed to `register` came from `Process::new` with a non-empty name.

## Milestones

| Milestone | You implement | Check with |
|-----------|---------------|------------|
| M1 | `ProcessState`, `Process`, `ProcessError`, `new`, accessors, `set_state` | `cargo test m1_` |
| M2 | `Registry`, `MAX_PROCESSES`, `new`, `len`, `register`, `RegistryError` | `cargo test m2_` |
| M3 | `get`, `get_mut` | `cargo test m3_` |

When M1–M3 pass, run `cargo test`.

## If you finish early

- Add `find_by_pid` scanning the `Vec`, or `remove` returning an owned `Process`. Lab 3 covers PID lookup and removal on a `HashMap`.
- Read about `std::mem::size_of` and stack vs heap layout for `String` and `Box`.
- Skim the `Rc` docs if you want to see shared ownership.
