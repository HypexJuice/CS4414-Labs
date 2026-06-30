use lab3_process_registry::{
    count_where, Named, Process, ProcessError, ProcessState, Registry, RegistryError,
};

// --- Milestone 1: rich errors, transition FSM ---

#[test]
fn m1_new_rejects_empty_name() {
    let result = Process::new(1, String::new());
    assert!(matches!(result, Err(ProcessError::EmptyName)));
}

#[test]
fn m1_transition_valid_and_invalid() {
    let mut process = Process::new(1, "init".into()).expect("valid name");
    process
        .transition(ProcessState::Sleeping)
        .expect("Running to Sleeping is allowed");
    assert_eq!(process.state(), ProcessState::Sleeping);

    process
        .transition(ProcessState::Stopped)
        .expect("Sleeping to Stopped is allowed");
    assert_eq!(process.state(), ProcessState::Stopped);

    let result = process.transition(ProcessState::Running);
    assert_eq!(
        result,
        Err(ProcessError::InvalidTransition {
            from: ProcessState::Stopped,
            to: ProcessState::Running,
        })
    );
}

// --- Milestone 2: HashMap registry ---

#[test]
fn m2_register_and_get_by_pid() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(42, "shell".into()).expect("valid"))
        .expect("register");
    assert_eq!(registry.len(), 1);
    assert_eq!(registry.get_by_pid(42).expect("found").name(), "shell");
}

#[test]
fn m2_duplicate_pid() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(42, "shell".into()).expect("valid"))
        .expect("register");
    let duplicate = registry.register(Process::new(42, "dup".into()).expect("valid"));
    assert_eq!(duplicate, Err(RegistryError::DuplicatePid(42)));
    assert_eq!(registry.len(), 1);
}

#[test]
fn m2_capacity_full() {
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

// --- Milestone 3: Named trait + count_where ---

#[test]
fn m3_named_trait_count_where() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(1, "init".into()).expect("valid"))
        .expect("register");
    registry
        .register(Process::new(2, "shell".into()).expect("valid"))
        .expect("register");
    registry
        .register(Process::new(3, "init".into()).expect("valid"))
        .expect("register");

    let p1 = registry.get_by_pid(1).expect("pid 1");
    let p2 = registry.get_by_pid(2).expect("pid 2");
    let p3 = registry.get_by_pid(3).expect("pid 3");
    assert_eq!(Named::name(p1), "init");
    let processes = [p1, p2, p3];

    let named_matches = count_where(&processes, |p| p.name() == "init");
    assert_eq!(named_matches, 2);

    let running_count = count_where(&processes, |p| p.state() == ProcessState::Running);
    assert_eq!(running_count, 3);
}

// --- Milestone 4: iter_by_state + iterator chaining ---

#[test]
fn m4_iter_by_state_chain() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(1, "a".into()).expect("valid"))
        .expect("register");
    let mut sleeping = Process::new(2, "b".into()).expect("valid");
    sleeping
        .transition(ProcessState::Sleeping)
        .expect("transition");
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
fn m4_running_pids_collect() {
    let mut registry = Registry::new();
    registry
        .register(Process::new(10, "a".into()).expect("valid"))
        .expect("register");
    let mut stopped = Process::new(20, "b".into()).expect("valid");
    stopped.transition(ProcessState::Stopped).expect("transition");
    registry.register(stopped).expect("register");
    registry
        .register(Process::new(30, "c".into()).expect("valid"))
        .expect("register");

    let mut pids = registry.running_pids();
    pids.sort();
    assert_eq!(pids, vec![10, 30]);
}

// --- Milestone 5: remove_by_pid integration ---

#[test]
fn m5_remove_by_pid() {
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
