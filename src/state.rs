use std::collections::HashSet;

use crate::event::{Event, Key, MouseEvent};

type Ui = ();

pub struct State {
    pub ui: Vec<Ui>,
}

pub trait StateMachine {
    fn state(&self) -> &State;
    fn next(self, event: Event) -> dyn StateMachine;
    fn hint_input(&self) -> HashSet<Key>;
}
