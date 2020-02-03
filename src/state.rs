use std::collections::HashSet;

use crate::event::{Event, Key, MouseEvent};
use crate::Ui;

pub struct State {
    // Ui Layers
    pub ui: Vec<Box<Ui>>,
}

pub trait StateMachine {
    fn state(&self) -> &State;
    fn next(self, event: Event) -> Box<dyn StateMachine>;
    fn hint_input(&self) -> HashSet<Key>;
}
