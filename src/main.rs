use anyhow::Result;
use licorice::client::Lichess;
use termion::{
    event::Key,
    raw::IntoRawMode,
    screen::AlternateScreen
};
use tui::{
    backend::TermionBackend,
    Terminal
};
use std::{env, io};
mod app;
mod event;
mod ui;
use crate::event::{Event, Events};

fn main() -> Result<()>
{
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = app::App {
        conn: Lichess::new(env::var("RULIXEN_TOKEN")?)
    };

    let events = Events::new();

    loop
    {
        terminal.draw(|mut f| {
            ui::draw_chess_screen(&mut f, &app);
        })?;

        if let Event::Input(input) = events.next()?
        {
            match input
            {
                Key::Char('q') => break,
                _ => {}
            }
        }
    }

    Ok(())
}
