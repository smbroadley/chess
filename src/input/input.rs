use crate::core::Chess;

#[derive(Debug)]
pub struct Error {}

pub trait Input {
    fn update(&mut self, game: &mut Chess) -> Result<(), Error>;
}
