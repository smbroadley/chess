mod core;
mod input;
mod render;

use std::io;

use crate::core::{Chess, Engine};
use crate::input::CrosstermInput;
use crate::render::tui::TuiRenderer;
use crate::render::Theme;

fn main() -> Result<(), io::Error> {
    let game = Chess::default();

    let theme = Theme::default();
    let renderer = TuiRenderer::new(theme);

    // TODO: when playing over the network, the second player
    // will use a NetworkInput type.
    //
    let p1 = CrosstermInput;
    let p2 = CrosstermInput;

    let engine = Engine::new(game, renderer, p1, p2);

    engine.run();

    Ok(())
}
