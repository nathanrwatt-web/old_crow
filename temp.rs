use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    style::Stylize,
    widgets::{Block, Paragraph,
            List, ListItem, ListDirection, ListState},
    DefaultTerminal, Frame,
};


struct App {
    todo_list: TodoList,
    editor: Editor,
    focus: Focus,
    running: bool,
}

// ======= TodoList =========

struct TodoList {
    item_list: Vec<String>,
}



// ======= Editor =========

struct Editor {
    input: String, 
    character_index: usize,
    input_mode: InputMode,
}

enum InputMode {
    Normal,
    Editing,
}

impl Editor {
    fn new() -> Self {
        Self {
            input: String::new(),
            character_index: 0,
            input_mode: InputMode::Normal,
        }
    }

    // moves index 
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    // insert character, move cursor
    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    // map to byte since characters are freaky 
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

            let current_index = self.character_index - 1;
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

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
           let key.kind == KeyEventKind::Release {
               continue;
           }

           match self.input_mode {
               InputMode::Normal => {
                   match key.code {
                       todo!() // do something 
                   }
               },
               InputMode::Editing => { 
                   match key.code {
                       todo!() // do something
                   }
               }
           }
        }
    }
}
