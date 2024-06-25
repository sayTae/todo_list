mod app;
mod ui;
mod todo;
mod input_mode;

use std::{io, error::Error};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use crate::app::App;
use crate::input_mode::InputMode;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('i') => {
                        app.input_mode = InputMode::Editing;
                        app.message.clear();
                        app.input.clear();
                    }
                    KeyCode::Char('q') => break,
                    KeyCode::Up => app.move_selection(-1),
                    KeyCode::Down => app.move_selection(1),
                    KeyCode::Char(' ') => app.toggle_todo(),
                    KeyCode::Delete => app.remove_todo(),
                    KeyCode::Char('s') => app.start_saving(),
                    KeyCode::Char('l') => app.start_loading(),
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => app.add_todo(),
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => { app.input.pop(); }
                    KeyCode::Esc => app.input_mode = InputMode::Normal,
                    _ => {}
                },
                InputMode::Saving => match key.code {
                    KeyCode::Enter => app.save_todo_list(),
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => { app.input.pop(); }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                        app.input.clear();
                    }
                    _ => {}
                },
                InputMode::Loading => match key.code {
                    KeyCode::Enter => app.load_todo_list(),
                    KeyCode::Up => app.move_file_selection(-1),
                    KeyCode::Down => app.move_file_selection(1),
                    KeyCode::Esc => app.input_mode = InputMode::Normal,
                    _ => {}
                },
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
