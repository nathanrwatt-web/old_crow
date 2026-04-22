use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};

// ======= Editor ======== 
pub struct Editor {
    pub input: String, 
    character_index : usize,
    pub input_mode: InputMode,
} 

pub enum InputMode {
    Normal, 
    Editing,
}

pub enum EditorAction {
    None,
    TodoList,
    Sumbit(String),
    Quit,
}


impl Editor {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            character_index: 0,
            input_mode: InputMode::Normal,
        }
    }

    pub fn handle_events(&mut self) -> Result<EditorAction> {
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
                        },
                        KeyCode::Char('t') => EditorAction::TodoList,
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


