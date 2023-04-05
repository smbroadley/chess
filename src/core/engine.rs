use crate::core::{self, Chess, State};
use crate::input;
use crate::render::{self, Theme};

#[derive(Debug)]
pub enum Error {
    Render(render::Error),
    Input(input::Error),
}

impl From<input::Error> for Error {
    fn from(err: input::Error) -> Self {
        Self::Input(err)
    }
}

impl From<render::Error> for Error {
    fn from(err: render::Error) -> Self {
        Self::Render(err)
    }
}

pub struct Engine<R: render::Renderer, I1: input::Input, I2: input::Input> {
    game: Chess,
    renderer: R,
    p1_input: I1,
    p2_input: I2,
}

#[allow(dead_code)]
impl<R: render::Renderer, I1: input::Input, I2: input::Input> Engine<R, I1, I2> {
    #[allow(dead_code)]
    pub fn new(game: Chess, renderer: R, p1_input: I1, p2_input: I2) -> Self {
        Self {
            game,
            renderer,
            p1_input,
            p2_input,
        }
    }

    #[allow(dead_code)]
    pub fn run(mut self) -> Result<(), Error> {
        let Engine {
            game,
            renderer,
            p1_input,
            p2_input,
        } = &mut self;

        renderer.init()?;
        renderer.set_theme(Theme::default());

        game.start();

        loop {
            match game.turn {
                core::Player::White => p1_input.update(game)?,
                core::Player::Black => p2_input.update(game)?,
            }

            renderer.render(game)?;

            match game.state {
                State::Exit => break,
                State::Paused => {}
                State::Playing => {}
            }
        }

        renderer.shutdown()?;

        Ok(())
    }
}
