pub use ropey;
pub use ropey::Rope;

pub type Event = winit::event::Event<'static, ()>;

use std::any::{type_name, Any};
pub use std::ops::Range;
use std::sync::RwLock as Rw;

pub type Context = Rw<EditorContext>;

type InputHandler = Box<dyn Fn(&Context, Event, &dyn Any)>;

pub struct EditorContext {
    pub buffers: Rw<Vec<Buffer>>,
    pub layout: Rw<Layout>,
    pub redraw_requested: Rw<bool>,

    input_handler: InputHandler,
    handler_state: Box<dyn Any>,
}

impl EditorContext {
    pub fn set_handler<T: Sized + 'static>(
        &mut self,
        f: impl Fn(&Context, Event, &T) + 'static,
        state: T,
    ) {
        self.handler_state = Box::new(state) as Box<dyn Any>;

        self.input_handler = Box::new(move |ctx, event, state| {
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
pub struct Layout {
    /// currently open and edited tab
    pub active_tab: usize,

    /// List of all tabs currently open. There should always be a split for every open buffer
    pub tabs: Vec<Tab>,
}

/// a Tab may consist of multiple open splits
pub struct Tab {
    /// current split inside the tab, that is to be edited
    pub active_split: usize,

    /// all splits that are supposed to be open in this tab
    pub splits: Vec<Split>,
}

/// A split is a view into an Buffer
/// It's a necessity in order for a user to edit the buffers
pub struct Split {
    /// ID of the Buffer this split is a view into.
    /// The original Buffer lies in the [Context]
    pub buffer_id: usize,
    pub cursor: Position,
    pub selected: Vec<Range<usize>>,
}

pub struct Position {
    pub row: usize,
    pub col: usize,
}
