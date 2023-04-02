use tui::style::Color;

pub struct Theme {
    pub black_piece: Color,
    pub black_tile: Color,
    pub black_tile_highlight: Color,
    pub white_piece: Color,
    pub white_tile: Color,
    pub white_tile_highlight: Color,
    pub cursor: Color,
    pub cursor_valid: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            black_piece: Color::Rgb(0, 170, 255),
            black_tile: Color::Rgb(10, 20, 40),
            black_tile_highlight: Color::Rgb(150, 150, 90),
            white_piece: Color::White,
            white_tile: Color::Rgb(40, 40, 40),
            white_tile_highlight: Color::Rgb(180, 180, 120),
            cursor: Color::Rgb(100, 100, 100),
            cursor_valid: Color::Rgb(100, 130, 100),
        }
    }
}
