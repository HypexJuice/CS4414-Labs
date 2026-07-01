// Process types from Lab 2 are provided below. Implement Registry, Named, and iterators.

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProcessError {
    EmptyName,
}

pub struct Process {
    pid: u32,
    name: String,
    state: ProcessState,
}

impl Process {
    pub fn new(pid: u32, name: String) -> Result<Self, ProcessError> {
        if name.is_empty() {
            return Err(ProcessError::EmptyName);
        }
        Ok(Self {
            pid,
            name,
            state: ProcessState::Running,
        })
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state(&self) -> ProcessState {
        self.state
    }

    pub fn set_state(&mut self, state: ProcessState) {
        self.state = state;
    }
}

pub trait Named {
    fn name(&self) -> &str;
}

impl Named for Process {
    fn name(&self) -> &str {
        todo!("Milestone 2: return the process name as &str")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegistryError {
    Full,
    DuplicatePid(u32),
    NotFound(u32),
}

pub struct Registry {
    processes: HashMap<u32, Box<Process>>,
}

impl Registry {
    pub const MAX_PROCESSES: usize = 8;

    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn register(&mut self, _process: Process) -> Result<(), RegistryError> {
        todo!()
    }

    pub fn get_by_pid(&self, _pid: u32) -> Result<&Process, RegistryError> {
        todo!()
    }

    pub fn remove_by_pid(&mut self, _pid: u32) -> Result<Process, RegistryError> {
        todo!()
    }

    pub fn iter_by_state(&self, _state: ProcessState) -> impl Iterator<Item = &Process> {
        self.processes.values().map(|boxed| boxed.as_ref()).take(0)
    }

    pub fn running_pids(&self) -> Vec<u32> {
        todo!()
    }
}
