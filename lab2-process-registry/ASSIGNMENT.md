# Lab 2 Assignment: Process Registry

## Introduction

You will build a miniature **process registry** — a table that tracks simulated OS processes, each with a process ID, a name, and a runtime state. Real kernels keep process metadata in structured types whose fields are **not** directly accessible; callers use a controlled API instead. Lab 1 used **public** fields on `Stats` for simplicity. In this lab you **hide** representation details and expose behavior through methods.

**Deliverable:** Implement everything in [`src/lib.rs`](src/lib.rs) so that `cargo test` exits with status 0.

---

## Setup

1. Install [rustup](https://rustup.rs/) (stable toolchain).
2. Clone the course labs repository and enter this crate:

```bash
git clone https://github.com/HypexJuice/CS4414-Labs.git
cd CS4414-Labs/lab2-process-registry
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

Derive `Copy`, `Clone`, `Debug`, `PartialEq`, and `Eq` so states can be compared with `==` and passed by value without moving ownership.

**Layout note:** A plain enum without attached data is typically stored compactly on the stack (often 1 byte). You will measure this in Milestone 2.

---

## Part 2 — The `Process` struct

Define a **public** struct named `Process` with these **private** fields:

| Field | Type | Memory role |
|-------|------|-------------|
| `pid` | `u32` | Stack integer — process identifier |
| `name` | `String` | Heap-backed name; the struct holds pointer/length/capacity on the stack |
| `state` | `ProcessState` | Stack enum — current lifecycle state |

Integration tests cannot construct `Process` with a struct literal because the fields are private. All access goes through the methods below.

### Error type

Define a **public** enum named `ProcessError` with exactly one variant: `EmptyName`. Derive `Debug`, `PartialEq`, and `Eq`.

### Constructor — Milestone 1 (`new`)

Provide an **associated function** that accepts a `u32` PID and an owned `String` name, and returns `Result<Process, ProcessError>`:

- If `name` is empty, return `Err(ProcessError::EmptyName)`.
- Otherwise return `Ok` with a new process whose state is `Running`.

### Accessor methods — Milestone 1

Provide **methods on an immutable reference** (`&self`):

- Return the PID as a `u32` (copied value).
- Return the name as `&str` — a **borrowed view** into the heap-allocated `String` without allocating a new string.
- Return the current `ProcessState`.

### Mutator — Milestone 1 (`set_state`)

Provide a **method on a mutable reference** (`&mut self`) that accepts a `ProcessState` and updates the process state in place.

---

## Part 3 — Memory layout and global capacity — Milestone 2

### Stack size helpers

On `Process`, implement three **public associated functions** (no receiver):

| Function | Returns |
|----------|---------|
| `process_state_stack_bytes` | `std::mem::size_of::<ProcessState>()` |
| `process_struct_stack_bytes` | `std::mem::size_of::<Process>()` |
| `reference_stack_bytes` | `std::mem::size_of::<&Process>()` |

**Why this matters:**

- `process_struct_stack_bytes` reports the stack footprint of a `Process` value — the `String`'s pointer/length/capacity metadata — **not** the bytes of the name stored on the heap. Two processes with very different name lengths should report the **same** struct stack size.
- `reference_stack_bytes` reports how large a reference to `Process` is (one pointer width on typical 64-bit machines).

### Global capacity constant

On `Registry` (defined in Part 4), declare a **public constant** named `MAX_PROCESSES` with value `8`. This is a compile-time program-wide limit on how many processes the registry may hold. Use `const`, not a mutable `static`.

---

## Part 4 — The `Registry` struct

Define a **public** struct named `Registry` with one **private** field:

| Field | Type | Meaning |
|-------|------|---------|
| `processes` | `Vec<Box<Process>>` | Growable heap buffer of heap-allocated process records |

`Vec` stores its elements in a heap-allocated buffer. Each `Box<Process>` is an owning pointer to a `Process` allocated on the heap. Together they model a common pattern: a dynamic collection of heap objects with single ownership.

### Error type

Define a **public** enum named `RegistryError` with exactly one variant: `Full`. Derive `Debug`, `PartialEq`, and `Eq`.

### Empty constructor — Milestone 3 (`new`)

Provide an **associated function** returning an empty registry (empty `Vec`).

### Length — Milestone 3 (`len`)

Provide a **method on `&self`** returning how many processes are currently stored.

### Register — Milestone 3 (`register`)

Provide a **method on `&mut self`** that accepts an owned `Process` (by value, not by reference):

- If the registry already holds `MAX_PROCESSES` entries, return `Err(RegistryError::Full)` without modifying the registry.
- Otherwise move the `Process` into a `Box`, push it into the `Vec`, and return `Ok(())`.
- After a successful call, the caller **no longer owns** the `Process` value — ownership transferred to the registry.

### Immutable lookup — Milestone 4 (`get`, `find_by_pid`)

Provide **methods on `&self`**:

- `get` accepts a `usize` index. Return `Some` with an immutable reference to the process at that index, or `None` if the index is out of range.
- `find_by_pid` accepts a `u32` PID. Search the registry and return `Some` with a reference to the first matching process, or `None` if no match exists.

Returned references are **borrowed from the registry**. They must not be used after the registry is dropped (the compiler enforces this via lifetime elision — you do not need explicit `'a` annotations in this lab).

### Mutable lookup — Milestone 5 (`get_mut`)

Provide a **method on `&mut self`** that accepts a `usize` index and returns `Option` with a **mutable reference** to the process at that index, or `None` if out of range. Use this to call `set_state` on a stored process.

### Remove — Milestone 5 (`remove`)

Provide a **method on `&mut self`** that accepts a `usize` index:

- If the index is out of range, return `None`.
- Otherwise remove the process from the `Vec`, transfer ownership back to the caller as an owned `Process`, and return `Some` containing it.
- The registry's length decreases by one.

---

## Worked example

After building a registry and registering three processes:

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

Expected state:

| Check | Expected value |
|-------|----------------|
| `registry.len()` | `3` |
| `registry.find_by_pid(42).unwrap().name()` | `"shell"` |
| `registry.get(2).unwrap().state()` | `ProcessState::Sleeping` |

After `registry.get_mut(2).unwrap().set_state(ProcessState::Stopped)`:

| Check | Expected value |
|-------|----------------|
| `registry.get(2).unwrap().state()` | `ProcessState::Stopped` |
| `registry.find_by_pid(1).unwrap().pid()` | `1` |

Use this to sanity-check your implementation before running the full test suite.

---

## Rules

1. Keep validation logic on `Process` and storage logic on `Registry`. Do not use separate free functions beyond method receivers.
2. Required patterns: private struct fields with public accessors; `register` takes `Process` by value; store processes in `Vec<Box<Process>>`; lookup methods return `Option` with references tied to `&self` / `&mut self`.
3. You may use everything from Lab 1 plus: `Vec`, `String`, `Option`, `Result`, `Box`, `std::mem::size_of`, `const`, and derive macros (`Copy`, `Clone`, `PartialEq`, `Eq`).
4. **Do not use:** `Rc`, `Arc`, `HashMap`, `BTreeMap`, custom traits, I/O, external crates, `unsafe`, or threads.
5. Assume every `Process` passed to `register` was constructed through `Process::new` with a non-empty name.

---

## Milestones

| Milestone | You implement | Check progress with |
|-----------|---------------|---------------------|
| **M1** | `ProcessState`, `Process`, `ProcessError`, constructor, accessors, `set_state` | `cargo test m1_` |
| **M2** | `size_of` helpers, `Registry::MAX_PROCESSES` | `cargo test m2_` |
| **M3** | `Registry` fields, `new`, `len`, `register`, `RegistryError` | `cargo test m3_` |
| **M4** | `get`, `find_by_pid` | `cargo test m4_` |
| **M5** | `get_mut`, `remove` | `cargo test m5_` |

When all milestones pass, run `cargo test` with no filter to confirm the full suite.

---

## Concept callouts

- **`String` vs `&str`** — `String` owns heap text; `&str` borrows a slice of text for the lifetime of the borrow.
- **`Box<T>`** — single-owner heap allocation; moving the `Box` moves ownership of the heap value.
- **`Vec<T>`** — growable heap-backed array; owns its elements.
- **Move semantics** — `register(process)` takes ownership; the caller cannot use `process` afterward.
- **Borrow checker** — `get` / `find_by_pid` return references valid only while the registry borrow is active.
- **Lifetime elision** — the compiler ties returned `&Process` to `&self` without you writing `'a` explicitly.
- **`const`** — compile-time constant known before the program runs (contrast with mutable `static`, deferred to a later lab).

---

## Grading

| Criterion | Weight |
|-----------|--------|
| All tests pass (`cargo test`) | 90% |
| Optional: two sentences — one way Rust's borrow checker prevents a bug that C/C++ might allow | 10% |

---

## Optional stretch (homework, not graded)

- `find_by_name(&self, &str) -> Option<&Process>` — search by borrowed name slice.
- Add a `HashMap<u32, usize>` PID-to-index map for O(1) lookup (preview of Lab 3).
- Read about `Rc` and when shared ownership is needed instead of `Box`.

---

## FAQ

**`cargo test` fails with "cannot find type `Process` in crate".**  
You have not defined the struct yet. Start with Milestone 1.

**Why `Box<Process>` inside `Vec` instead of `Vec<Process>`?**  
Both store data on the heap. `Box` makes heap allocation and single-owner pointer semantics explicit — practice for smart pointers you will use later.

**Why does `name()` return `&str` instead of `String`?**  
Returning `&str` borrows existing heap data without allocating a copy. The borrow lasts as long as the `Process` is borrowed.

**Can I use `register(&process)` instead of `register(process)`?**  
No. The assignment requires taking ownership by value so the registry owns each stored process.

**What happens if I use a reference after the registry is dropped?**  
The compiler rejects it — the returned `&Process` cannot outlive the registry's borrow.

**Tests compare errors with `==` or pattern matching.**  
Derive `Debug`, `PartialEq`, and `Eq` on `ProcessState`, `ProcessError`, and `RegistryError`.

---

## Test-to-spec trace (for instructors)

| Test | Spec reference |
|------|----------------|
| `m1_new_valid_process` | Constructor + accessors + default `Running` |
| `m1_new_rejects_empty_name` | Empty name validation |
| `m1_set_state_updates` | `set_state` mutator |
| `m2_process_state_size` | `process_state_stack_bytes` |
| `m2_process_struct_fixed_size` | `process_struct_stack_bytes` independent of heap buffer |
| `m2_reference_is_pointer_sized` | `reference_stack_bytes` |
| `m2_max_processes_const` | `MAX_PROCESSES == 8` |
| `m3_register_moves_ownership` | `register` increments `len` |
| `m3_register_respects_capacity` | `RegistryError::Full` at capacity |
| `m4_get_and_find_by_pid` | Index and PID lookup + `None` cases |
| `m4_borrowed_name_does_not_allocate` | `name() -> &str` |
| `m5_get_mut_updates_state` | Mutable borrow updates visible via `get` |
| `m5_remove_returns_owner` | `remove` returns owned `Process`, decreases `len` |
