use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{
    core::Vec2,
    input::{Error, Input},
};

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        todo!()
    }
}

pub struct CrosstermInput;

impl Input for CrosstermInput {
    fn update(&mut self, game: &mut crate::core::Chess) -> Result<(), Error> {
        if poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            }) = read()?
            {
                match code {
                    KeyCode::Esc => game.quit(),
                    KeyCode::Char('h') => game.move_cursor(Vec2::LEFT),
                    KeyCode::Char('j') => game.move_cursor(Vec2::DOWN),
                    KeyCode::Char('k') => game.move_cursor(Vec2::UP),
                    KeyCode::Char('l') => game.move_cursor(Vec2::RIGHT),
                    KeyCode::Char(' ') => game.action(),
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
