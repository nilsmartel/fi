pub use termion::event::{Key, MouseEvent};

pub enum Event {
    Key(Key),
    Mouse(MouseEvent),
    Time(std::time::SystemTime),
    Signal,
}
