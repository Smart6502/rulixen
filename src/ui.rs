use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    terminal::Frame,
    widgets::{Block, Borders}
};
use crate::app::App;

pub fn draw_chess_screen<B: Backend>(f: &mut Frame<B>, app: &App)
{
    let bg = Block::default()
        .style(Style::default().bg(Color::Rgb(22, 21, 18)));

    f.render_widget(bg, f.size());

    let main = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(90), Constraint::Min(5)].as_ref())
        .split(f.size());
        
    let game = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(20), Constraint::Percentage(50), Constraint::Percentage(20), Constraint::Percentage(5)].as_ref())
        .split(main[0]);

    let leftpane = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Length(1), Constraint::Min(5)].as_ref())
        .split(game[1]);

    let rightpane = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(60), Constraint::Percentage(20)].as_ref())
        .split(game[3]);

    let infoblock = Block::default()
        .style(Style::default().bg(Color::Rgb(38, 36, 33)))
        .title("Info");

    let chat = Block::default()
        .style(Style::default().bg(Color::Rgb(38, 36, 33)))
        .title("Chat Room");

    let clock = Block::default()
        .style(Style::default().bg(Color::Rgb(38, 36, 33)))
        .title("Clock");

    f.render_widget(clock, rightpane[1]);
    f.render_widget(infoblock, leftpane[0]);
    f.render_widget(chat, leftpane[2]);
}
