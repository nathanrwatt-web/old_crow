use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::ListState;

pub struct TodoItem {
    pub item_name: String,
}

pub struct TodoList {
    pub item_list: Vec<TodoItem>,
    pub state: ListState,
}

pub enum TodoListAction {
    None, 
    Quit,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            item_list: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn push(&mut self, todo: String) {
        let new_item = TodoItem { item_name: todo };
        self.item_list.push(new_item);
        // if no item, autoselect the first added item
        if self.state.selected().is_none() {
            self.state.select(Some(0));
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
            KeyCode::Esc => {
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
            _ => { TodoListAction::None }  // something? 
        };
        Ok(action)
    }
}
