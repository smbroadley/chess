use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::Style,
    text::{Span, Spans},
    widgets::Paragraph,
    Terminal,
};

use crate::core::{Chess, Mode, MoveResult, Player};
use crate::render::{Error, Renderer, Theme};

use super::widgets::ChessWidget;

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        todo!()
    }
}

pub struct TuiRenderer {
    terminal: Option<Terminal<CrosstermBackend<io::Stdout>>>,
    theme: Theme,
}

impl TuiRenderer {
    pub fn new(theme: Theme) -> Self {
        Self {
            terminal: None,
            theme,
        }
    }
}

impl Renderer for TuiRenderer {
    fn init(&mut self) -> Result<(), Error> {
        // enter raw mode: this disables Crtl-C and
        // input echoing on the terminal.
        //
        enable_raw_mode()?;

        // create the Tui Terminal object
        //
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);

        self.terminal = Some(Terminal::new(backend)?);

        Ok(())
    }

    fn render(&mut self, game: &mut Chess) -> Result<(), Error> {
        // render the game
        //
        self.terminal.as_mut().unwrap().draw(|frame| {
            let layout = Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .margin(0)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(8),
                    Constraint::Min(1),
                    Constraint::Min(1),
                ])
                .split(frame.size());

            let widget = ChessWidget::new(&self.theme);

            frame.render_stateful_widget(widget, layout[1], game);

            let theme = &self.theme;

            // render clocks
            //
            let white_clock = if game.turn == Player::White {
                theme.white.piece
            } else {
                theme.white.tile
            };

            let black_clock = if game.turn == Player::Black {
                theme.black.piece
            } else {
                theme.black.tile
            };

            let p1 = Paragraph::new(Spans::from(vec![Span::styled(
                format!("{}", game.timers[0]),
                Style::default().fg(white_clock),
            )]));

            frame.render_widget(p1, layout[2]);

            let p2 = Paragraph::new(Spans::from(vec![
                Span::from(" "),
                Span::styled(
                    format!("{}", game.timers[1]),
                    Style::default().fg(black_clock),
                ),
            ]));

            frame.render_widget(p2, layout[0]);

            // render status line
            //
            if let Mode::Moving(from) = game.mode {
                let msg = match game.get_move_result(from, game.cursor) {
                    MoveResult::Cancel => "Cancel",
                    MoveResult::Nothing => "Move",
                    MoveResult::Capture(_) => "Capture",
                    MoveResult::Castle => "Castle",
                    MoveResult::Promotion(_) => "Promote",
                    MoveResult::Invalid => "",
                };

                let status = Paragraph::new(msg);

                frame.render_widget(status, layout[3]);
            }
        })?;

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Error> {
        if let Some(terminal) = &mut self.terminal {
            disable_raw_mode()?;

            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;

            terminal.show_cursor()?;
        }

        Ok(())
    }

    fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
}
