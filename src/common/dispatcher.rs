use common::observer;

use std::cell::Cell;

/// Responsible for dispatching events from multiple subjects, sync them if necessary
pub struct Dispatcher {
    subjects: Vec<Box<observer::Subject>>,
    stop: Cell<bool>,
    start_event: observer::Event,
    idle_event: observer::Event,
    // current_datetime
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            subjects: Vec::new(),
            stop: Cell::new(false),
            start_event: observer::Event::new(),
            idle_event: observer::Event::new()
        }
    }

    pub fn stop(&self) {
        self.stop.set(true);
    }

    pub fn add_subject(&self, subject: Box<observer::Subject>) {
        // Check if subject is already added

        // Subject dispatch priority
    } 

    pub fn run(&self) {
        for subject in &self.subjects {
            subject.start();
        }

        self.start_event.emit();

        while !self.stop.get() {
            let (eof, events_dispatched) = self.dispatch();

            if eof {
                self.stop();
            } else if !events_dispatched {
                self.idle_event.emit();
            }
        }

        // Finally
        for subject in &self.subjects {
            subject.stop();
        }
        for subject in &self.subjects {
            subject.join();
        }
    }

    /// 1. True if all subjects hit eof
    /// 2. True if at least one subject dispatched events
    fn dispatch(&self) -> (bool, bool) {
        let mut eof = true;
        let mut events_dispatched = false;

        // Scan for lowest datetime

        // Dispatch realtime subjects and those subjects with the lwoest datetime

        (eof, events_dispatched)
    }
}

pub enum DispatcherPriority {
    First = 0,
    Broker = 1000,
    BarFeed = 2000,
    Last = 100000
}