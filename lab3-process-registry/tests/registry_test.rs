use lab3_process_registry::{
    Named, Process, ProcessError, ProcessState, Registry, RegistryError,
};

// --- Milestone 1: HashMap registry ---

#[test]
fn m1_new_rejects_empty_name() {
    let result = Process::new(1, String::new());
    assert!(matches!(result, Err(ProcessError::EmptyName)));
}

#[test]
fn m1_register_and_get_by_pid() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(42, "shell".into()).expect("valid"))
        .expect("register");
    assert_eq!(registry.len(), 1);
    assert_eq!(registry.get_by_pid(42).expect("found").name(), "shell");
}

#[test]
fn m1_duplicate_pid() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(42, "shell".into()).expect("valid"))
        .expect("register");
    let duplicate = registry.register(Process::new(42, "dup".into()).expect("valid"));
    assert_eq!(duplicate, Err(RegistryError::DuplicatePid(42)));
    assert_eq!(registry.len(), 1);
}

#[test]
fn m1_capacity_full() {
    let mut registry = Registry::new();
    for pid in 0..Registry::MAX_PROCESSES {
        registry
            .register(Process::new(pid as u32, format!("p{pid}")).expect("valid"))
            .expect("room available");
    }
    let overflow = registry.register(Process::new(99, "overflow".into()).expect("valid"));
    assert_eq!(overflow, Err(RegistryError::Full));
    assert_eq!(registry.len(), Registry::MAX_PROCESSES);
}

// --- Milestone 2: Named trait ---

#[test]
fn m2_named_trait_for_process() {
    let process = Process::new(1, "init".into()).expect("valid name");
    assert_eq!(Named::name(&process), "init");

    let mut registry = Registry::new();
    registry.register(process).expect("register");
    let stored = registry.get_by_pid(1).expect("found");
    assert_eq!(stored.name(), "init");
    assert_eq!(Named::name(stored), "init");
}

// --- Milestone 3: iter_by_state + iterator chaining ---

#[test]
fn m3_iter_by_state_chain() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(1, "a".into()).expect("valid"))
        .expect("register");
    let mut sleeping = Process::new(2, "b".into()).expect("valid");
    sleeping.set_state(ProcessState::Sleeping);
    registry.register(sleeping).expect("register");
    registry
        .register(Process::new(3, "c".into()).expect("valid"))
        .expect("register");

    let sleeping_pids: Vec<u32> = registry
        .iter_by_state(ProcessState::Sleeping)
        .map(|p| p.pid())
        .collect();
    assert_eq!(sleeping_pids, vec![2]);

    let running_count = registry
        .iter_by_state(ProcessState::Running)
        .count();
    assert_eq!(running_count, 2);
}

#[test]
fn m3_running_pids_collect() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(10, "a".into()).expect("valid"))
        .expect("register");
    let mut stopped = Process::new(20, "b".into()).expect("valid");
    stopped.set_state(ProcessState::Stopped);
    registry.register(stopped).expect("register");
    registry
        .register(Process::new(30, "c".into()).expect("valid"))
        .expect("register");

    let mut pids = registry.running_pids();
    pids.sort();
    assert_eq!(pids, vec![10, 30]);
}

// --- Milestone 4: remove_by_pid ---

#[test]
fn m4_remove_by_pid() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(1, "init".into()).expect("valid"))
        .expect("register");
    registry
        .register(Process::new(42, "shell".into()).expect("valid"))
        .expect("register");

    let removed = registry.remove_by_pid(1).expect("removed");
    assert_eq!(removed.pid(), 1);
    assert_eq!(removed.name(), "init");
    assert_eq!(registry.len(), 1);
    assert!(matches!(
        registry.get_by_pid(1),
        Err(RegistryError::NotFound(1))
    ));
    assert_eq!(registry.get_by_pid(42).expect("found").name(), "shell");
}
