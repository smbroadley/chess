use crate::core::{self, Chess, State};
use crate::input;
use crate::render::{self, Theme};

pub struct Engine<R: render::Renderer, I1: input::Input, I2: input::Input> {
    game: Chess,
    renderer: R,
    p1_input: I1,
    p2_input: I2,
}

impl<R: render::Renderer, I1: input::Input, I2: input::Input> Engine<R, I1, I2> {
    pub fn new(game: Chess, renderer: R, p1_input: I1, p2_input: I2) -> Self {
        Self {
            game,
            renderer,
            p1_input,
            p2_input,
        }
    }

    pub fn run(mut self) {
        let Engine {
            game,
            renderer,
            p1_input,
            p2_input,
        } = &mut self;

        renderer.init();
        renderer.set_theme(Theme::default());

        game.start();

        loop {
            match game.turn {
                core::Player::White => p1_input.update(game),
                core::Player::Black => p2_input.update(game),
            }
            .err();

            renderer.render(game);

            match game.state {
                State::Exit => break,
                State::Paused => {}
                State::Playing => {}
            }
        }

        renderer.shutdown();
    }
}
