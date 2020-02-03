mod event;
mod startscreen;
mod state;

pub use event::Event;
pub use state::{State, StateMachine};

use std::io::{stdin, stdout, Write};
use termion::{
    event::{Key, MouseEvent},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
};
use tui::{backend::TermionBackend, terminal::Terminal};

fn main() -> std::io::Result<()> {
    let stdin = stdin();
    let mut term = terminal();
    term.clear()?;

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            termion::event::Event::Key(Key::Char('q')) => break,
            termion::event::Event::Mouse(me) => match me {
                MouseEvent::Press(_, x, y) => {
                    // write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
                }
                _ => (),
            },
            _ => {}
        }
        term.flush().unwrap();
    }

    Ok(())
}

fn terminal() -> Terminal<impl tui::backend::Backend> {
    let stdout = stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let backend = TermionBackend::new(stdout);
    Terminal::new(backend).unwrap()
}
