use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub state: ProcessState,
    pub priority: u8,
    pub memory_pages: Vec<u32>,
}

#[derive(Debug, Clone)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

pub struct Scheduler {
    processes: VecDeque<Process>,
    next_pid: u32,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            processes: VecDeque::new(),
            next_pid: 1,
        }
    }

    pub fn create_process(&mut self, priority: u8) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;

        let process = Process {
            pid,
            state: ProcessState::Ready,
            priority,
            memory_pages: Vec::new(),
        };

        self.processes.push_back(process);
        pid
    }

    pub fn schedule(&mut self) -> Option<u32> {
        if let Some(mut process) = self.processes.pop_front() {
            if matches!(process.state, ProcessState::Ready) {
                process.state = ProcessState::Running;
                let pid = process.pid;
                self.processes.push_back(process);
                Some(pid)
            } else {
                self.processes.push_back(process);
                None
            }
        } else {
            None
        }
    }
}