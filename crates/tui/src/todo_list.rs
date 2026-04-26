use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    style::{Style, Stylize},
    layout::Rect,
    widgets::{ListState, ListItem, List, Block},
    Frame,
    
};
use crate::screen::{Screen, Transition};
use crate::editor::Editor;


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
}

impl Screen for TodoList {
    fn handle_event(&mut self, event: Event) -> Transition {
        let Event::Key(key) = event else { return Transition::Stay; };
        if key.kind != KeyEventKind::Press { return Transition::Stay; }

        match key.code {
            KeyCode::Char('q') => Transition::Pop,
            KeyCode::Up | KeyCode::Char('i') => {
                self.state.select_previous();
                Transition::Stay
            },
            KeyCode::Down | KeyCode::Char('k') => {
                self.state.select_next();
                Transition::Stay
            },
            KeyCode::Backspace => {
                self.item_list.remove(self.state.selected().unwrap());
                Transition::Stay
            },
            /*
            KeyCode::Char('e') => {
                Transition::Push(Box::new(Editor::new()))
            }
            */
            _ => Transition::Stay
        }
    }
    
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.item_list.iter()
            .map(|item| ListItem::new(
                    format!("{}{}{}", item.item_name, item.date, match item.priority {
                        Priority::Low => "Low Priority",
                        Priority::Medium => "Medium Priority",
                        Priority::High => "High Priority",
                    })
                    )
                )
            .collect();
        let list = List::new(items)
            .highlight_style(Style::new().reversed())
            .highlight_symbol("> ")
            .block(Block::bordered());
        frame.render_stateful_widget(list, area, &mut self.state);
    }

    fn footer_hint(&self) -> &str {
        "TodoList: <q> quit | <i/j> move | <e> editor | <Backspace> delete"
    }

}
