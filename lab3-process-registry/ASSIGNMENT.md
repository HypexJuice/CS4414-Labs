# Lab 3 Assignment: Process Registry API

## Introduction

You will rebuild the **process registry** as version 2. Lab 2 stored processes in a `Vec` and scanned linearly for PID lookup, with simple error enums. Real subsystems need **fast lookup**, **expressive errors**, and **composable queries**. This lab adds a `HashMap` index, richer `Result`-based APIs, **traits and generics** for reusable logic, and **iterator chaining** with `impl Trait` return types — without class inheritance.

**Deliverable:** Implement everything in [`src/lib.rs`](src/lib.rs) so that `cargo test` exits with status 0.

---

## Setup

1. Install [rustup](https://rustup.rs/) (stable toolchain).
2. Clone the course labs repository and enter this crate:

```bash
git clone https://github.com/HypexJuice/CS4414-Labs.git
cd CS4414-Labs/lab3-process-registry
```

3. Run `cargo test`. On the starter code, you should see **compile errors** until you implement the types below. That is normal.

---

## Part 1 — The `ProcessState` enum

Define a **public** enum named `ProcessState` with exactly three variants:

| Variant | Meaning |
|---------|---------|
| `Running` | Process is actively executing |
| `Sleeping` | Process is blocked or waiting |
| `Stopped` | Process has been terminated |

Derive `Copy`, `Clone`, `Debug`, `PartialEq`, and `Eq`.

---

## Part 2 — The `Process` struct and errors

Define a **public** struct named `Process` with **private** fields:

| Field | Type |
|-------|------|
| `pid` | `u32` |
| `name` | `String` |
| `state` | `ProcessState` |

### `ProcessError` enum

Define a **public** enum named `ProcessError` with these variants:

| Variant | Meaning |
|---------|---------|
| `EmptyName` | Constructor received an empty name |
| `InvalidTransition { from, to }` | Illegal state change (carries both states) |

Derive `Debug`, `PartialEq`, and `Eq`.

### Constructor — Milestone 1 (`new`)

Associated function accepting `u32` PID and owned `String` name, returning `Result<Process, ProcessError>`:

- Empty name → `Err(ProcessError::EmptyName)`.
- Otherwise → `Ok` with state `Running`.

### Accessors — Milestone 1

Methods on `&self` returning `pid` (`u32`), `name` (`&str`), and `state` (`ProcessState`).

### State machine — Milestone 1 (`transition`)

Method on `&mut self` accepting a new `ProcessState`, returning `Result<(), ProcessError>`:

- If the current state is `Stopped`, any transition is rejected with `InvalidTransition { from: Stopped, to: new_state }`.
- Otherwise update state and return `Ok(())`.

**Rule:** `Stopped` is **terminal** — a stopped process cannot resume.

Use the `?` operator where appropriate inside methods that return `Result`.

---

## Part 3 — Traits and generics — Milestone 3

### `Named` trait

Define a **public** trait named `Named` with one method on `&self` that returns the entity's name as `&str`.

Implement `Named` for `Process` using the existing name accessor logic.

### `count_where` generic function

Define a **public** function named `count_where` that:

- Accepts a slice `&[T]` of items to inspect.
- Accepts a predicate callable as `FnMut(&T) -> bool`.
- Returns how many items satisfy the predicate.

This is **static dispatch** — the compiler generates specialized code for each call site. No `dyn Trait` in the graded path.

---

## Part 4 — The `Registry` struct

Define a **public** struct named `Registry` with one **private** field:

| Field | Type | Meaning |
|-------|------|---------|
| `processes` | `HashMap<u32, Box<Process>>` | PID-keyed heap map of processes |

Use `HashMap` as the **sole** process store. Do not keep a parallel `Vec` index.

### `RegistryError` enum

| Variant | Meaning |
|---------|---------|
| `Full` | Registry at capacity |
| `DuplicatePid(u32)` | PID already registered |
| `NotFound(u32)` | No process with that PID |

Derive `Debug`, `PartialEq`, and `Eq`.

### Global capacity

Public constant `MAX_PROCESSES` with value `8` (same policy as Lab 2).

### Core registry methods — Milestone 2

| Method | Behavior |
|--------|----------|
| `new()` | Empty `HashMap` |
| `len(&self)` | Number of stored processes |
| `register(&mut self, process: Process) -> Result<(), RegistryError>` | If at capacity → `Full`. If PID exists → `DuplicatePid`. Else insert and return `Ok(())`. Takes ownership of `process`. |
| `get_by_pid(&self, pid: u32) -> Result<&Process, RegistryError>` | Immutable borrow of stored process, or `NotFound`. |

Use `?`, `ok_or`, or `match` — no panics on expected error paths.

### Removal — Milestone 5 (`remove_by_pid`)

Method on `&mut self` accepting a PID, returning `Result<Process, RegistryError>`:

- If PID missing → `NotFound`.
- Otherwise remove from map, return owned `Process`, decrease length.

---

## Part 5 — Iterators and `impl Trait` — Milestone 4

### `iter_by_state`

Method on `&self` accepting a `ProcessState`, returning **`impl Iterator<Item = &Process>`**:

- Yield references to processes whose current state matches.
- Implement using `HashMap` iteration and iterator adapters (`filter`, etc.).
- Return type must use `impl Trait` — do not box as `dyn Iterator` in the graded solution.

### `running_pids`

Method on `&self` returning `Vec<u32>`:

- Collect PIDs of all processes in `Running` state.
- Build using `iter_by_state` chained with `map` and `collect`.

---

## Worked example

```text
let mut reg = Registry::new();
reg.register(Process::new(1, "init".into())?)?;
reg.register(Process::new(42, "shell".into())?)?;

reg.get_by_pid(42)?.name() == "shell"
reg.register(Process::new(42, "dup".into())?)  → Err(DuplicatePid(42))

let mut init = reg.remove_by_pid(1)?;
init.transition(ProcessState::Stopped)?;
init.transition(ProcessState::Running)
    → Err(InvalidTransition { from: Stopped, to: Running })

reg.running_pids() == [42]   // shell still Running; init removed
```

---

## Rules

1. Keep validation and transitions on `Process`; storage policy on `Registry`.
2. Required patterns: `HashMap` sole store; fallible methods return `Result`; `iter_by_state` returns `impl Iterator`; `Stopped` is terminal.
3. You may use everything from Lab 2 plus: custom traits, trait impls, generics, `HashMap`, `FnMut` bounds, `impl Trait`, iterator adapters (`filter`, `map`, `count`, `collect`), `?`, `ok_or`, `map` on `Result`.
4. **Do not use (graded core):** `dyn Trait`, `Rc`, `Arc`, threads, `async`, external crates, `unsafe`, I/O, `std::error::Error`.
5. Library code must not panic on expected error paths (tests may use `expect`).

---

## Milestones

| Milestone | You implement | Check progress with |
|-----------|---------------|---------------------|
| **M1** | `ProcessError`, `transition` FSM, accessors, `new` | `cargo test m1_` |
| **M2** | `Registry`, `RegistryError`, `HashMap` store, `register`, `get_by_pid`, `len` | `cargo test m2_` |
| **M3** | `Named` trait + `count_where` | `cargo test m3_` |
| **M4** | `iter_by_state`, `running_pids` | `cargo test m4_` |
| **M5** | `remove_by_pid` | `cargo test m5_` |

When all milestones pass, run `cargo test` with no filter.

---

## Concept callouts

- **`Result` and `?`** — express failure without panics; `?` propagates errors early.
- **`Option` vs `Result`** — use `Option` when absence is normal; `Result` when absence is an error (this lab uses `Result` for registry lookups).
- **Traits** — shared behavior without inheritance; compile-time polymorphism via monomorphization.
- **Generics + trait bounds** — one function works over many types (`count_where`).
- **`impl Trait`** — hide a concrete iterator type behind an opaque return type.
- **Iterator chaining** — lazy pipelines: `filter`, `map`, `collect`.
- **`HashMap`** — O(1) average PID lookup on heap.

---

## Grading

| Criterion | Weight |
|-----------|--------|
| All tests pass (`cargo test`) | 90% |
| Optional: two sentences — one advantage of `impl Trait` over naming a concrete iterator type | 10% |

---

## Optional stretch (homework, not graded)

### Dynamic traits and vtables

- Define `trait Observer { fn on_transition(&self, pid: u32, from: ProcessState, to: ProcessState); }`
- Store `Vec<Box<dyn Observer>>` on `Registry`; notify observers after successful transitions.
- Compare `size_of::<&dyn Observer>()` vs `size_of::<&Process>()` — trait objects use a **fat pointer** (data address + vtable address).

### Other stretch

- `find_by_name(&self, &str) -> Option<&Process>`
- `BTreeMap` for sorted PID iteration
- Read about `std::error::Error` and crates like `thiserror`

---

## FAQ

**Why `HashMap` instead of `Vec`?**  
PID lookup is O(1) average instead of scanning every entry.

**Why not `dyn Iterator` for `iter_by_state`?**  
`impl Trait` avoids heap allocation and keeps static dispatch; `dyn Iterator` is heap-indirected and not required here.

**Can a stopped process transition to `Running`?**  
No. `Stopped` is terminal in this lab's state machine.

**Do I need explicit lifetime annotations?**  
No. Returned `&Process` references use lifetime elision tied to `&self`, as in Lab 2.

---

## Test-to-spec trace (for instructors)

| Test | Spec reference |
|------|----------------|
| `m1_new_rejects_empty_name` | `EmptyName` validation |
| `m1_transition_valid_and_invalid` | FSM + `InvalidTransition` |
| `m2_register_and_get_by_pid` | `register` + `get_by_pid` |
| `m2_duplicate_pid` | `DuplicatePid` |
| `m2_capacity_full` | `Full` at `MAX_PROCESSES` |
| `m3_named_trait_count_where` | `Named` + `count_where` |
| `m4_iter_by_state_chain` | `iter_by_state` + adapters |
| `m4_running_pids_collect` | `running_pids` chain |
| `m5_remove_by_pid` | `remove_by_pid` + `NotFound` |
