use tui::style::Color;

use crate::core::Player;

pub struct Theme {
    pub black: PlayerTheme,
    pub white: PlayerTheme,
    pub cursor_valid: Color,
}

pub struct PlayerTheme {
    pub cursor: Color,
    pub piece: Color,
    pub tile: Color,
    pub tile_highlight: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            black: PlayerTheme {
                cursor: Color::Rgb(40, 60, 120),
                piece: Color::Rgb(0, 170, 255),
                tile: Color::Rgb(10, 20, 40),
                tile_highlight: Color::Rgb(150, 150, 90),
            },
            white: PlayerTheme {
                cursor: Color::Rgb(120, 120, 120),
                piece: Color::White,
                tile: Color::Rgb(40, 40, 40),
                tile_highlight: Color::Rgb(180, 180, 120),
            },
            cursor_valid: Color::Rgb(100, 130, 100),
        }
    }
}

impl Theme {
    pub fn get_player(&self, p: Player) -> &PlayerTheme {
        match p {
            Player::Black => &self.black,
            Player::White => &self.white,
        }
    }
}
