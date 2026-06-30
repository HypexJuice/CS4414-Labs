use lab2_process_registry::{
    Process, ProcessError, ProcessState, Registry, RegistryError,
};

// --- Milestone 1: ProcessState, encapsulated Process, constructor, accessors, mutator ---

#[test]
fn m1_new_valid_process() {
    let process = Process::new(42, "shell".into()).expect("valid name");
    assert_eq!(process.pid(), 42);
    assert_eq!(process.name(), "shell");
    assert_eq!(process.state(), ProcessState::Running);
}

#[test]
fn m1_new_rejects_empty_name() {
    let result = Process::new(1, String::new());
    assert!(matches!(result, Err(ProcessError::EmptyName)));
}

#[test]
fn m1_set_state_updates() {
    let mut process = Process::new(7, "worker".into()).expect("valid name");
    process.set_state(ProcessState::Sleeping);
    assert_eq!(process.state(), ProcessState::Sleeping);
}

// --- Milestone 2: memory layout helpers and global capacity constant ---

#[test]
fn m2_process_state_size() {
    assert!(Process::process_state_stack_bytes() >= 1);
}

#[test]
fn m2_process_struct_fixed_size() {
    let short = Process::new(1, "a".into()).expect("valid name");
    let long = Process::new(2, "a".repeat(10_000)).expect("valid name");
    assert_eq!(
        Process::process_struct_stack_bytes(),
        std::mem::size_of_val(&short)
    );
    assert_eq!(
        Process::process_struct_stack_bytes(),
        std::mem::size_of_val(&long)
    );
}

#[test]
fn m2_reference_is_pointer_sized() {
    assert_eq!(
        Process::reference_stack_bytes(),
        std::mem::size_of::<usize>()
    );
}

#[test]
fn m2_max_processes_const() {
    assert_eq!(Registry::MAX_PROCESSES, 8);
}

// --- Milestone 3: Registry construction, register, capacity ---

#[test]
fn m3_register_moves_ownership() {
    let mut registry = Registry::new();
    assert_eq!(registry.len(), 0);

    let process = Process::new(1, "init".into()).expect("valid name");
    registry
        .register(process)
        .expect("registry should accept process");
  // `process` is moved; the line below must not compile if uncommented:
  // let _ = process.pid();

    assert_eq!(registry.len(), 1);
}

#[test]
fn m3_register_respects_capacity() {
    let mut registry = Registry::new();
    for pid in 0..Registry::MAX_PROCESSES {
        let name = format!("p{pid}");
        let process = Process::new(pid as u32, name).expect("valid name");
        registry.register(process).expect("room available");
    }
    let extra = Process::new(99, "overflow".into()).expect("valid name");
    assert_eq!(registry.register(extra), Err(RegistryError::Full));
    assert_eq!(registry.len(), Registry::MAX_PROCESSES);
}

// --- Milestone 4: immutable borrows via get and find_by_pid ---

#[test]
fn m4_get_and_find_by_pid() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(1, "init".into()).expect("valid"))
        .expect("register");
    registry
        .register(Process::new(42, "shell".into()).expect("valid"))
        .expect("register");
    registry
        .register(Process::new(99, "logger".into()).expect("valid"))
        .expect("register");

    assert_eq!(registry.get(1).unwrap().pid(), 42);
    assert_eq!(registry.get(1).unwrap().name(), "shell");
    assert_eq!(registry.find_by_pid(99).unwrap().name(), "logger");
    assert!(registry.get(3).is_none());
    assert!(registry.find_by_pid(0).is_none());
}

#[test]
fn m4_borrowed_name_does_not_allocate() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(10, "metrics".into()).expect("valid"))
        .expect("register");
    let name: &str = registry.get(0).unwrap().name();
    assert_eq!(name, "metrics");
}

// --- Milestone 5: mutable borrows and remove (ownership out) ---

#[test]
fn m5_get_mut_updates_state() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(1, "init".into()).expect("valid"))
        .expect("register");
    registry
        .register(Process::new(42, "shell".into()).expect("valid"))
        .expect("register");
    registry
        .register(Process::new(99, "logger".into()).expect("valid"))
        .expect("register");

    registry
        .get_mut(2)
        .expect("index 2 exists")
        .set_state(ProcessState::Stopped);
    assert_eq!(
        registry.get(2).unwrap().state(),
        ProcessState::Stopped
    );
    assert_eq!(registry.find_by_pid(1).unwrap().pid(), 1);
}

#[test]
fn m5_remove_returns_owner() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(5, "temp".into()).expect("valid"))
        .expect("register");
    registry
        .register(Process::new(6, "keep".into()).expect("valid"))
        .expect("register");

    let removed = registry.remove(0).expect("index 0 exists");
    assert_eq!(removed.pid(), 5);
    assert_eq!(removed.name(), "temp");
    assert_eq!(registry.len(), 1);
    assert_eq!(registry.get(0).unwrap().pid(), 6);
    assert!(registry.remove(5).is_none());
}
