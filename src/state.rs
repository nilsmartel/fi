use std::collections::HashSet;

use crate::event::{Event, Key, MouseEvent};
use tui::widgets::Widget;

pub struct State {
    // Ui Layers
    pub ui: Vec<Box<dyn Widget>>,
}

pub trait StateMachine {
    fn state(&self) -> &State;
    fn next(self, event: Event) -> Box<dyn StateMachine>;
    fn hint_input(&self) -> HashSet<Key>;
}
