pub use ropey;
pub use ropey::Rope;

pub type Event = winit::event::Event<'static, ()>;

use std::any::{type_name, Any};
pub use std::ops::Range;
use std::sync::Mutex;
use std::sync::RwLock as Rw;

type InputHandler = Mutex<Box<dyn Fn(&Context, Event, &dyn Any)>>;

#[cfg(test)]
mod context_tests {
    use super::*;

    #[test]
    fn input_state() {
        let mut ctx = Context::default();

        ctx.set_handler(
            Mutex::new(String::from("hello world")),
            |ctx, event, state| {
                let mut state = state.lock().unwrap();
                state.push_str(", this is from the handler");

                assert_eq!("hello world, this is from the handler", *state);
            },
        );
    }
}

pub struct Context {
    pub buffers: Rw<Vec<Buffer>>,
    pub layout: Rw<Layout>,
    pub redraw_requested: Rw<bool>,

    input_handler: InputHandler,
    /// Runtime checked editable object
    handler_state: Rw<Box<dyn Any>>,
}

impl std::default::Default for Context {
    fn default() -> Self {
        Self {
            buffers: Default::default(),
            layout: Default::default(),
            redraw_requested: Default::default(),
            input_handler: Mutex::new(Box::new(|_, _, _| {})),
            handler_state: Rw::new(Box::new(false)),
        }
    }
}

impl Context {
    pub fn set_handler<T: Sized + 'static>(
        &self,
        state: T,
        f: impl Fn(&Context, Event, &T) + 'static,
    ) {
        *self.handler_state.write().expect("to set state of handler") =
            Box::new(state) as Box<dyn Any>;

        *self.input_handler.lock().expect("to set handler") = Box::new(move |ctx, event, state| {
            f(
                ctx,
                event,
                state.downcast_ref::<T>().expect(type_name::<T>()),
            )
        });
    }
}

pub struct Buffer {
    pub path: Option<Rw<String>>,
    pub content: Rw<Rope>,
    pub unsaved_changes: Rw<bool>,
    /// default is "text"
    pub filetype: Rw<String>,
    // TODO onsave, onchange, etc.
}

/// Layout and editing information
#[derive(Default)]
pub struct Layout {
    /// currently open and edited tab
    pub active_tab: usize,

    /// List of all tabs currently open. There should always be a split for every open buffer
    pub tabs: Vec<Tab>,
}

/// a Tab may consist of multiple open splits
#[derive(Default)]
pub struct Tab {
    /// current split inside the tab, that is to be edited
    pub active_split: usize,

    /// all splits that are supposed to be open in this tab
    pub splits: Vec<Split>,
}

/// A split is a view into an Buffer
/// It's a necessity in order for a user to edit the buffers
#[derive(Default)]
pub struct Split {
    /// ID of the Buffer this split is a view into.
    /// The original Buffer lies in the [Context]
    pub buffer_id: usize,
    pub cursor: Position,
    pub selected: Vec<Range<usize>>,
}

#[derive(Default)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}
