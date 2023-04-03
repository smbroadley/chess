mod chess;
mod core;
mod gamestate;
mod net;
mod theme;
mod timer;
mod vec;

use crossterm::{
    event::{
        poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gamestate::Mode;
use std::{io, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::Style,
    text::{Span, Spans},
    widgets::Paragraph,
    Terminal,
};
use vec::Vec2;

use crate::chess::Chess;
use crate::core::Player;
use crate::gamestate::GameState;
use crate::theme::Theme;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut game = GameState::default();
    let theme = Theme::default();

    game.start();

    loop {
        // render the game
        //
        terminal.draw(|frame| {
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

            let chess = Chess::new(&theme);

            frame.render_stateful_widget(chess, layout[1], &mut game);

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
                    core::MoveResult::Cancel => "Cancel",
                    core::MoveResult::Nothing => "Move",
                    core::MoveResult::Capture(_) => "Capture",
                    core::MoveResult::Castle => "Castle",
                    core::MoveResult::Promotion(_) => "Promote",
                    core::MoveResult::Invalid => "INVALID",
                };

                let status = Paragraph::new(msg);

                frame.render_widget(status, layout[3]);
            }
        })?;

        // handle input
        //
        if poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            }) = read()?
            {
                match code {
                    KeyCode::Esc => break,
                    KeyCode::Char('h') => game.move_cursor(Vec2::LEFT),
                    KeyCode::Char('j') => game.move_cursor(Vec2::DOWN),
                    KeyCode::Char('k') => game.move_cursor(Vec2::UP),
                    KeyCode::Char('l') => game.move_cursor(Vec2::RIGHT),
                    KeyCode::Char(' ') => game.action(),
                    _ => {}
                }
            }
        }
    }

    // restore terminal
    //
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
