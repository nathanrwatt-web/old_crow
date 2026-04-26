use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    style::{Style, Stylize},
    layout::Rect,
    widgets::{ListState, ListItem, List, Block},
    Frame,
};

use crate::screen::{Screen, Transition};
use crate::todo_list::TodoList;


pub struct Menu {
   pub application_list: Vec<String>,
   pub state: ListState,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            application_list: vec!["TodoList".to_string()],
            state: ListState::default(),
        }
    }
}

impl Screen for Menu {
    fn handle_event(&mut self, ev: Event) -> Transition {
        let Event::Key(key) = ev else { return Transition::Stay; };
        if key.kind != KeyEventKind::Press { return Transition::Stay; }

        match key.code {
            KeyCode::Char('q') => Transition::Quit,
            KeyCode::Up | KeyCode::Char('i') => {
               self.state.select_previous();
               Transition::Stay
            },
            KeyCode::Down | KeyCode::Char('k') => {
                self.state.select_next();
                Transition::Stay
            },
            KeyCode::Enter => {
                match self.state.selected() {
                    Some(0) => Transition::Push(Box::new(TodoList::new())),
                    _ => Transition::Stay,
                }
            },
            _ => Transition::Stay,
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.application_list.iter()
            .map(|item| ListItem::new(item.as_str())).collect();
        let list = List::new(items)
            .highlight_style(Style::new().reversed())
            .highlight_symbol("> ")
            .block(Block::bordered().title("Menu"));
        frame.render_stateful_widget(list, area, &mut self.state);
    }

    fn footer_hint(&self) -> &str {
        "Menu: <q> quit | <i/k> move | <enter> select"
    }
}
