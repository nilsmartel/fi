//! This modules includes the initial State of fi
//! if no arguments are passed to it
//! refering to a file that is supposed to be opened

use crate::{Event, State, StateMachine};
use std::collections::HashSet;
use termion::event::Key;

/// Base for most other States
pub struct BaseMachine {
    state: State,
}

impl BaseMachine {
    fn new() -> BaseMachine {
        let ui = vec![ui];
        let state = State { ui };

        BaseMachine { state }
    }
}

impl StateMachine for BaseMachine {
    fn state(&self) -> &State {
        &self.state
    }

    fn next(self, event: Event) -> Box<dyn StateMachine> {
        match event {
            _ => unimplemented!(),
        }
    }

    fn hint_input(&self) -> HashSet<Key> {
        HashSet::new()
    }
}
