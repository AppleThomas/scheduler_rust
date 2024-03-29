use std::{num::ParseIntError, str::SplitWhitespace};

#[derive(PartialEq, Clone, Debug)]
pub enum ProcessState {
    Ready,
    Running,
}

/// A data structure representing a running Program
#[derive(Clone)]
pub struct Process {
    pub name: String,
    pub arrival_time: i32,
    pub burst_time: i32,
    pub time_remaining: i32,
    pub state: Option<ProcessState>,

    pub turnaround_time: i32,
    pub response_time: i32,
    pub wait_time: i32,
    pub finish_time: i32,
    pub last_selection_time: Option<i32>
}

impl Process {
    pub fn new(name: String, arrival_time: i32, burst_time: i32) -> Process {
        Process {
            name,
            arrival_time,
            burst_time,
            time_remaining: burst_time,
            state: None,
            turnaround_time: 0,
            response_time: 0,
            wait_time: 0,
            finish_time: 0,
            last_selection_time: None
        }
    }

    /// Takes a split whitespace line and attempts to great a new process
    pub fn parse(mut process_line: SplitWhitespace) -> Result<Process, ParseIntError>{
        // process name
        process_line.next();
        let process_name = process_line.nth(0).unwrap_or_default().to_string();

        // arrival time
        process_line.next();
        let arrival_time = process_line.nth(0).unwrap_or_default().parse::<i32>()?;

        // burst time
        process_line.next();
        let burst_time = process_line.nth(0).unwrap_or_default().parse::<i32>()?;

        Ok(Process::new(process_name, arrival_time, burst_time))
    }

    /// Simulates the ticking of time by 1 given the time that the process is ticking to
    pub fn tick(&mut self, cur_time: i32) {
        if let Some(state) = &self.state {
            self.turnaround_time += 1;
            match state {
                ProcessState::Ready => {
                    self.wait_time += 1;
                    
                    // if we have not been selected before
                    if self.burst_time == self.time_remaining {
                        self.response_time += 1;
                    }
                },
                ProcessState::Running => {
                    self.time_remaining -= 1;

                    if self.time_remaining == 0 {
                        self.finish(cur_time);
                    }
                },
            }
        }
    }

    /// Sets the process state to running (if not already running)
    /// Also prints selection time and remaining time for the process
    pub fn select(&mut self, cur_time: i32) {
        self.state = Some(ProcessState::Running);
        self.last_selection_time = Some(cur_time);
    }

    /// Sets the process state back to ready (if not already finished)
    pub fn deselect(&mut self) {
        if !self.finished() {
            self.state = Some(ProcessState::Ready);
        }
    }

    // Sets the process to a finished state and prints finish time
    fn finish(&mut self, cur_time: i32) {
        self.state = None;
        self.finish_time = cur_time;
    }

    /// Returns whether the process has finished
    pub fn finished(&self) -> bool {
        self.time_remaining == 0
    }

    /// Returns whether the process has arrived at the given time
    pub fn arrived(&self, time: i32) -> bool {
        self.arrival_time <= time
    }
}