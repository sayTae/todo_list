use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::App;
use crate::input_mode::InputMode;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let input_title = match app.input_mode {
        InputMode::Normal => "New Todo (Press 'i' to edit, 's' to save, 'l' to load)",
        InputMode::Editing => "Editing (Press Esc to stop editing)",
        InputMode::Saving => "Enter filename to save (Press Enter to save)",
        InputMode::Loading => "Select a file to load (Press Enter to load)",
    };

    let input_message = if app.input_mode == InputMode::Normal {
        app.message.as_str()
    } else {
        app.input.as_str()
    };

    let input = Paragraph::new(input_message)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title(input_title));
    f.render_widget(input, chunks[0]);

    match app.input_mode {
        InputMode::Loading => {
            let items: Vec<ListItem> = app.file_list
                .iter()
                .map(|name| {
                    ListItem::new(Text::from(name.clone()))
                })
                .collect();

            let files = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Saved Files"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            f.render_stateful_widget(files, chunks[1], &mut app.file_list_state);
        }
        _ => {
            let items: Vec<ListItem> = app.todos
                .iter()
                .enumerate()
                .map(|(i, todo)| {
                    let content = format!("[{}] {}", if todo.completed { "√" } else { " " }, todo.text);
                    let style = if Some(i) == app.list_state.selected() {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Text::from(Span::styled(content, style)))
                })
                .collect();

            let todos = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Todos"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            f.render_stateful_widget(todos, chunks[1], &mut app.list_state);
        }
    }
}
