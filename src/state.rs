use std::collections::HashSet;

type Event = char;
type Ui = ();
type Key = char;

pub struct State {
    pub ui: Vec<Ui>,
}

pub trait StateMachine {
    fn state(&self) -> &State;
    fn next(self, event: Event) -> dyn StateMachine;
    fn hint_input(&self) -> HashSet<Key>;
}
