use crossterm::event::{Event, KeyCode, KeyEventKind, KeyEvent};
use ratatui::{
    Frame, layout::{Constraint, Rect, Layout},
    style::{Style, Stylize, Color},
    widgets::{Block, List, ListItem, ListState, Paragraph}
};
use crate::screen::{Screen, Transition};
use crate::text_field::TextField;

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
    pub form: Option<TodoForm>,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            item_list: Vec::new(),
            state: ListState::default(),
            form: None,
        }
    }

    fn commit_form(&mut self, form: &mut TodoForm) {
        let new_item = TodoItem {
            item_name: form.name.take(), 
            date: form.date.take(),
            priority: match form.priority {
                Priority::Low => Priority::Low,
                Priority::High => Priority::High,
                Priority::Medium => Priority::Medium,
            },
        };
        form.name.clear();
        form.date.clear();
        self.item_list.push(new_item);

        if self.state.selected().is_none() { self.state.select(Some(0)); }
    }
}

// ===== Finite State for editing vs Scrolling =====
#[derive(PartialEq)]
enum FormFocus { Name, Date, Priority }
enum FormResult { Stay, Cancel, Submit }

struct TodoForm {
    name: TextField, 
    date: TextField, 
    priority: Priority,
    focus: FormFocus, 
    editing_existing: Option<usize>,
}

impl TodoForm {
    fn new() -> Self {
        Self {
            name: TextField::new(),
            date: TextField::new(),
            priority: Priority::Low,
            focus: FormFocus::Name,
            editing_existing: None,
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> FormResult {
        match key.code {
            KeyCode::Esc => return FormResult::Cancel,
            KeyCode::Tab => { 
                self.focus = self.next_focus();
                return FormResult::Stay;
            },
            KeyCode::Enter => { return FormResult::Submit; },
            _ => { }
        }

        match self.focus {
            FormFocus::Name => { self.name.handle_key(key); },
            FormFocus::Date => { self.date.handle_key(key); },
            FormFocus::Priority => {
                match key.code {
                    KeyCode::Left | KeyCode::Right => self.cycle_priority(),
                    _ => { }
                }
            }
        }
        FormResult::Stay
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let [name_area, date_area, priority_area] = Layout::vertical([
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
        ]).areas(area);

        self.name.draw(frame, name_area, "Name", self.focus == FormFocus::Name);
        self.date.draw(frame, date_area, "Date", self.focus == FormFocus::Date);

        let border_style = if self.focus == FormFocus::Priority {
            Style::new().fg(Color::LightBlue) 
        } else {
            Style::new().fg(Color::DarkGray)
        };

        let block = Block::bordered().title("Priority").border_style(border_style);
        let para = Paragraph::new( match self.priority {
            Priority::Low => "Low", Priority::Medium => "Medium", Priority::High => "High",
        }).block(block);
        frame.render_widget(para, priority_area);
    }

    fn cycle_priority(&mut self) {
        self.priority = match self.priority {
            Priority::Low => Priority::Medium,
            Priority::Medium => Priority::High,
            Priority::High => Priority::Low,
        };

    }

    fn next_focus(&mut self) -> FormFocus {
        match self.focus {
            FormFocus::Name => FormFocus::Date,
            FormFocus::Date => FormFocus::Priority,
            FormFocus::Priority => FormFocus::Date,
        }
    }

}

impl Screen for TodoList {
    fn handle_event(&mut self, event: Event) -> Transition {
        let Event::Key(key) = event else { return Transition::Stay; };
        if key.kind != KeyEventKind::Press { return Transition::Stay; }

        // see if form exists 
        if let Some(form) = &mut self.form {
            match form.handle_key(key) {
                FormResult::Stay => {},
                FormResult::Cancel => self.form = None,
                FormResult::Submit => {
                    let mut f = self.form.take().unwrap(); // potentially add check for none?
                    self.commit_form(&mut f);
                }
            }
            return Transition::Stay;
        }

        match key.code {
            KeyCode::Char('q') => Transition::Pop,
            KeyCode::Char('n') => {
                self.form = Some(TodoForm::new());
                Transition::Stay
            },
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
            KeyCode::Char('e') => {
                self.form = Some(TodoForm::new());
                Transition::Stay
            }
            _ => Transition::Stay
        }
    }
    
    fn draw(&mut self, frame: &mut Frame, area: Rect) {

        if let Some(form) = &mut self.form {
            form.draw(frame, area);
            return
        }
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
