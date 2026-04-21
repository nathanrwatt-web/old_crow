use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    style::Stylize,
    widgets::{Block, Paragraph,
            List, ListItem, ListDirection, ListState},
    DefaultTerminal, Frame,
};

enum Focus {
    Editor,
    TodoList,
}

struct App {
    todo_list: TodoList,
    editor: Editor,
    focus: Focus,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            todo_list: TodoList::new(),
            editor: Editor::new(),
            focus: Focus::Editor,
            should_quit: false,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            match self.focus {
                // if in Editor
                Focus::Editor => {
                    match self.editor.handle_events()? {
                        EditorAction::None => {},
                        EditorAction::Sumbit(text) => self.todo_list.push(text),
                        EditorAction::Quit => self.should_quit = true,
                    }
                },
                // if in TodoList
                Focus::TodoList => {
                    self.focus = Focus::Editor; // for now just change focus back to editor 
                }
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        use ratatui::layout::{Constraint, Layout};

        let [header, body,editor_area, footer] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3), 
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let title = Paragraph::new(" productivity ").bold().block(Block::bordered());
        frame.render_widget(title, header);
        
        let items: Vec<ListItem> = self.todo_list.item_list.iter().map(|item| ListItem::new(item.item_name.as_str())).collect();
        let content = List::new(items);
        frame.render_widget(content, body);

        let edit_stuff = Paragraph::new(self.editor.input.as_str()).block(Block::bordered());
        frame.render_widget(edit_stuff, editor_area);

        let help = match self.editor.input_mode {
            InputMode::Normal => {
                Paragraph::new("<q> to quit, <e> to enter edit ").dim()
            }
            InputMode::Editing => {
                Paragraph::new("<esc> to normal, <backspace> to delete").dim()

            },
        };
        frame.render_widget(help, footer);
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}



// ======= Editor ======== 
struct Editor {
    input: String, 
    character_index : usize,
    input_mode: InputMode,
} 

enum InputMode {
    Normal, 
    Editing,
}

enum EditorAction {
    None,
    Sumbit(String),
    Quit,
}

impl Editor {
    fn new() -> Self {
        Self {
            input: String::new(),
            character_index: 0,
            input_mode: InputMode::Normal,
        }
    }

    fn handle_events(&mut self) -> Result<EditorAction> {
        // check for event for .1 seconds 
        if !event::poll(std::time::Duration::from_millis(100))?{
            return Ok(EditorAction::None)
        }

        let Event::Key(key) = event::read()? else {
            return Ok(EditorAction::None);
        };
        
        let action = match self.input_mode {
                // handle normal mode 
                InputMode::Normal => match key.code {
                        KeyCode::Char('q') => EditorAction::Quit,
                        KeyCode::Char('e') => {
                            self.input_mode = InputMode::Editing;
                            EditorAction::None
                        }
                        _ => EditorAction::None,
                    },
                // handle editing 
                InputMode::Editing => match key.code {
                        KeyCode::Esc => { 
                            self.input_mode = InputMode::Normal;
                            self.clear();
                            EditorAction::None
                        },
                        KeyCode::Backspace => {
                            self.delete_char();
                            EditorAction::None
                        },
                        KeyCode::Char(c) => {
                            self.enter_char(c);
                            EditorAction::None
                        },
                        KeyCode::Enter => {
                            let text = std::mem::take(&mut self.input);
                            self.reset_cursor();
                            EditorAction::Sumbit(text)
                        }
                        _ => EditorAction::None,
                   }
        };
        Ok(action)
    }

    fn clear(&mut self) {
        *self = Self::new();
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }


    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }
}


// ========= TodoList ============
struct TodoItem {
    item_name: String, 
}

struct TodoList {
    item_list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> Self {
        Self {
            item_list: Vec::new(),
        }
    }

    fn push(&mut self, todo: String) {
        let new_item = TodoItem {
            item_name: todo,
        };
        self.item_list.push(new_item)
    }
}
