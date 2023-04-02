use crate::{
    gamestate::{GameState, Mode},
    theme::Theme,
    vec::Vec2,
};
use tui::{layout::Rect, widgets::StatefulWidget};

pub struct Chess<'a> {
    theme: &'a Theme,
}

impl<'a> Chess<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }
}

impl<'a> StatefulWidget for Chess<'a> {
    type State = GameState;

    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State) {
        let glyphs = [
            "\u{265a}", "\u{265b}", "\u{265d}", "\u{265e}", "\u{265c}", "\u{2659}",
        ];

        // render the board
        //
        for y in 0..8 {
            for x in 0..8 {
                // render background
                //
                let xy = Vec2::new(x, y);
                let is_cursor = xy == state.cursor;
                let is_white_tile = (x + y) % 2 == 1;

                let mut col = if is_white_tile {
                    self.theme.white.tile
                } else {
                    self.theme.black.tile
                };

                if is_cursor {
                    col = self.theme.get_player(state.turn).cursor;

                    match state.mode {
                        Mode::Selecting => {
                            if state.can_move_cursor_piece() {
                                col = self.theme.cursor_valid;
                            }
                        }
                        Mode::Moving(_) => {
                            if state.is_valid_move(xy) {
                                col = self.theme.cursor_valid;
                            }
                        }
                    };
                } else {
                    match state.mode {
                        Mode::Selecting => {}
                        Mode::Moving(_) => {
                            if state.is_valid_move(xy) {
                                if is_white_tile {
                                    col = self.theme.white.tile_highlight;
                                } else {
                                    col = self.theme.black.tile_highlight;
                                }
                            }
                        }
                    };
                }

                let sx = area.x + x as u16 * 2;
                let sy = area.y + y as u16;

                buf.get_mut(sx + 0, sy).set_bg(col);
                buf.get_mut(sx + 1, sy).set_bg(col);

                // render piece
                //
                if let Some(piece) = state.board.get(Vec2::new(x as i16, y as i16)) {
                    let glyph = glyphs[piece.ty as usize];
                    let col = self.theme.get_player(piece.player).piece;

                    buf.get_mut(sx, sy).set_symbol(glyph).set_fg(col);
                }
            }
        }
    }
}
