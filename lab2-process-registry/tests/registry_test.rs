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

// --- Milestone 2: Registry construction, register, capacity ---

#[test]
fn m2_register_moves_ownership() {
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
fn m2_register_respects_capacity() {
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

#[test]
fn m2_max_processes_const() {
    assert_eq!(Registry::MAX_PROCESSES, 8);
}

// --- Milestone 3: immutable and mutable borrows via get / get_mut ---

#[test]
fn m3_get_by_index() {
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
    assert_eq!(registry.get(2).unwrap().name(), "logger");
    assert!(registry.get(3).is_none());
}

#[test]
fn m3_borrowed_name_does_not_allocate() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(10, "metrics".into()).expect("valid"))
        .expect("register");
    let name: &str = registry.get(0).unwrap().name();
    assert_eq!(name, "metrics");
}

#[test]
fn m3_get_mut_updates_state() {
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
    assert_eq!(registry.get(0).unwrap().pid(), 1);
}
