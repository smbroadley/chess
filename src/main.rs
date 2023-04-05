mod core;
mod input;
mod render;

use crate::core::{engine::Engine, Chess};
use crate::input::CrosstermInput;
use crate::render::tui::TuiRenderer;
use crate::render::Theme;

fn main() -> Result<(), core::engine::Error> {
    let game = Chess::default();

    let theme = Theme::default();
    let renderer = TuiRenderer::new(theme);

    // TODO: when playing over the network, the second player
    // will use a NetworkInput type.
    //
    let p1 = CrosstermInput;
    let p2 = CrosstermInput;

    let engine = Engine::new(game, renderer, p1, p2);

    engine.run()?;

    Ok(())
}
