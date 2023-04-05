pub mod theme;
pub mod tui;

use crate::core::Chess;

pub use theme::Theme;

#[derive(Debug)]
pub struct Error {}

pub trait Renderer {
    fn init(&mut self) -> Result<(), Error>;
    fn set_theme(&mut self, theme: Theme);
    fn render(&mut self, game: &mut Chess) -> Result<(), Error>;
    fn shutdown(&mut self) -> Result<(), Error>;
}
