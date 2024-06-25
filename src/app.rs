use std::{path::{Path, PathBuf}, fs};
use ratatui::widgets::ListState;
use crate::todo::TodoItem;
use crate::input_mode::InputMode;

pub struct App {
    pub todos: Vec<TodoItem>,
    pub input: String,
    pub input_mode: InputMode,
    pub list_state: ListState,
    pub save_dir: PathBuf,
    pub file_list: Vec<String>,
    pub file_list_state: ListState,
    pub current_file: Option<String>,
    pub message: String,
}

impl App {
    pub fn new() -> App {
        let save_dir = PathBuf::from("./todo_lists");
        fs::create_dir_all(&save_dir).unwrap_or_else(|_| {});
        let file_list = App::get_file_list(&save_dir);
        App {
            todos: Vec::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            list_state: ListState::default(),
            save_dir,
            file_list,
            file_list_state: ListState::default(),
            current_file: None,
            message: String::new(),
        }
    }

    pub fn add_todo(&mut self) {
        if !self.input.is_empty() {
            self.todos.push(TodoItem {
                text: self.input.drain(..).collect(),
                completed: false,
            });
        }
    }

    pub fn toggle_todo(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i < self.todos.len() {
                self.todos[i].completed = !self.todos[i].completed;
            }
        }
    }

    pub fn remove_todo(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i < self.todos.len() {
                self.todos.remove(i);
                if i >= self.todos.len() {
                    self.list_state.select(if self.todos.is_empty() { None } else { Some(self.todos.len() - 1) });
                }
            }
        }
    }

    pub fn move_selection(&mut self, delta: i32) {
        let len = self.todos.len();
        if len == 0 {
            self.list_state.select(None);
        } else {
            let new_index = match self.list_state.selected() {
                Some(i) => (i as i32 + delta).rem_euclid(len as i32) as usize,
                None => if delta > 0 { 0 } else { len - 1 },
            };
            self.list_state.select(Some(new_index));
        }
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let path = self.save_dir.join(filename);
        let mut content = String::new();
        for todo in &self.todos {
            let status = if todo.completed { "√" } else { " " };
            content.push_str(&format!("[{}] {}\n", status, todo.text));
        }
        fs::write(path, content)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str, save_dir: &Path) -> std::io::Result<Self> {
        let path = save_dir.join(filename);
        let content = fs::read_to_string(path)?;
        let mut app = App::new();
        app.current_file = Some(filename.to_string());
        for line in content.lines() {
            if line.len() > 4 && (line.starts_with("[ ]") || line.starts_with("[√]")) {
                let completed = line.starts_with("[√]");
                let text = line[4..].to_string();
                app.todos.push(TodoItem { text, completed });
            }
        }
        Ok(app)
    }

    fn get_file_list(dir: &Path) -> Vec<String> {
        fs::read_dir(dir)
            .unwrap()
            .filter_map(|entry| {
                entry.ok().and_then(|e| 
                    e.path().file_name()
                        .and_then(|n| n.to_str().map(String::from))
                )
            })
            .filter(|name| name.ends_with(".list"))
            .collect()
    }

    pub fn refresh_file_list(&mut self) {
        self.file_list = App::get_file_list(&self.save_dir);
    }

    pub fn start_saving(&mut self) {
        if let Some(filename) = &self.current_file {
            match self.save_to_file(filename) {
                Ok(_) => {
                    self.message = format!("Todo list saved to {}", filename);
                }
                Err(e) => {
                    self.message = format!("Failed to save todo list: {}", e);
                }
            }
        } else {
            self.input_mode = InputMode::Saving;
            self.input.clear();
        }
    }

    pub fn start_loading(&mut self) {
        self.input_mode = InputMode::Loading;
        self.refresh_file_list();
        if !self.file_list.is_empty() {
            self.file_list_state.select(Some(0));
        }
    }

    pub fn save_todo_list(&mut self) {
        let filename = if self.input.is_empty() {
            "todo.list".to_string()
        } else {
            if !self.input.ends_with(".list") {
                self.input.push_str(".list");
            }
            self.input.clone()
        };
        match self.save_to_file(&filename) {
            Ok(_) => {
                self.message = format!("Todo list saved to {}", filename);
                self.current_file = Some(filename);
            }
            Err(e) => {
                self.message = format!("Failed to save todo list: {}", e);
            }
        }
        self.input_mode = InputMode::Normal;
    }

    pub fn load_todo_list(&mut self) {
        if let Some(index) = self.file_list_state.selected() {
            let filename = &self.file_list[index];
            match App::load_from_file(filename, &self.save_dir) {
                Ok(loaded_app) => {
                    self.todos = loaded_app.todos;
                    self.current_file = loaded_app.current_file;
                    self.list_state = ListState::default();
                    if !self.todos.is_empty() {
                        self.list_state.select(Some(0));
                    }
                    self.message = format!("Todo list loaded from {}", filename);
                }
                Err(e) => {
                    self.message = format!("Failed to load todo list: {}", e);
                }
            }
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn move_file_selection(&mut self, delta: i32) {
        let len = self.file_list.len();
        if len > 0 {
            let i = self.file_list_state.selected().unwrap_or(0) as i32;
            let new_index = (i + delta).rem_euclid(len as i32) as usize;
            self.file_list_state.select(Some(new_index));
        }
    }
}
