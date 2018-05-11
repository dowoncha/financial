use chrono::NaiveDateTime;

pub struct Event {
    // handlers: Vec<>
    // to_subscribe: Vec<>
    // to_unsubscribe: Vec<>
    emitting: bool,
    _type: EventType
}

enum EventType {
    NewValue { datetime: NaiveDateTime, value: f64 }
}

impl Event {
    pub fn new() -> Self {
        Self {
            emitting: false
        }
    }

    // pub fn subscribe(&self, handler)

    fn apply_changes(&self) {
    }

    pub fn emit(&self) {
        // for handler in self.handlers { 
        //     handler()
        // }

        self.apply_changes();
    }
}

pub trait Subject {
    fn start(&self) { }

    fn stop(&self);

    fn join(&self);

    /// Returns true if there are not more events to dispatch
    fn eof(&self) -> bool;

    /// Dispatch events. If true is returned, it means at least one event was dispatched
    fn dispatch(&self) -> bool;

    /// Return datetime for next event;
    /// Needed to properly sync non-realtime subjects
    fn peekDateTime(&self) -> Option<String>;
}