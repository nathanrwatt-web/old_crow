use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::ListState;

pub enum Priority {
    Low, 
    Medium, 
    High,
}

pub struct TodoItem {
    pub item_name: String,
    pub date: String, 
    pub priority: Priority,
}

pub struct TodoList {
    pub item_list: Vec<TodoItem>,
    pub state: ListState,
}

pub enum TodoListAction {
    None, 
    Quit,
    Delete,
    Edit,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            item_list: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn push(&mut self, name: String, date_input: String, priority_input : Priority) {
        let new_item = TodoItem { item_name: name, date: date_input, priority: priority_input};
        self.item_list.push(new_item);
        // if no item, autoselect the first added item
        if self.state.selected().is_none() {
            self.state.select(Some(0));
        }
    }

    pub fn edit_selected(&mut self, index: usize, name: String, date_input: String, priority_input : Priority) {
        if !self.item_list.is_empty() {
            self.item_list[index].item_name = name;
            self.item_list[index].date = date_input;
            self.item_list[index].priority = priority_input;
        } else {
            self.push(name, date_input, priority_input);
        }
    }

    pub fn handle_events(&mut self) -> Result<TodoListAction> {

        if !event::poll(std::time::Duration::from_millis(100))? {
            return Ok(TodoListAction::None);
        }
        let Event::Key(key) = event::read()? else {
            return Ok(TodoListAction::None);
        };

        if key.kind != KeyEventKind::Press {
            return Ok(TodoListAction::None);
        }

        let action = match key.code {
            KeyCode::Char('q') => {
                TodoListAction::Quit
            },
            KeyCode::Up | KeyCode::Char('i') => {
                self.state.select_previous(); // list it bottom up stack
                TodoListAction::None
            },
            KeyCode::Down | KeyCode::Char('k') => {
                self.state.select_next();
                TodoListAction::None
            },
            KeyCode::Backspace => {
                TodoListAction::Delete
            },
            KeyCode::Char('e') => {
                TodoListAction::Edit
            }
            _ => { TodoListAction::None }  // something? 
        };
        Ok(action)
    }
}
