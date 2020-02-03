pub use termion::event::{Key, MouseEvent};

pub enum Event {
    Key(Key),
    Mouse(MouseEvent),
    Time(std::time::SystemTime),
    SignalRaw(Vec<u8>),
}

impl From<char> for Event {
    fn from(c: char) -> Event {
        let key = Key::Char(c);
        Event::Key(key)
    }
}
