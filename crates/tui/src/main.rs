use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    style::Stylize,
    widgets::{Block, Paragraph,
            List, ListItem, ListDirection, ListState},
    DefaultTerminal, Frame,
};


struct App {
    todo_list: Vec<String>,
    editor: Editor,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self { todo_list: vec![String::from("List Item 1"), String::from("List Item 2")], should_quit: false }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        use ratatui::layout::{Constraint, Layout};

        let [header, body, footer] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let title = Paragraph::new(" productivity ")
            .bold()
            .block(Block::bordered());
        frame.render_widget(title, header);
        
        let items: Vec<ListItem> = self.todo_list.iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect();
        let content = List::new(items);
        
        frame.render_widget(content, body);

        let help = Paragraph::new("[j] down [k] up [q] quit").dim();
        frame.render_widget(help, footer);
    }

    fn handle_events(&mut self) -> Result<()> {
        if !event::poll(std::time::Duration::from_millis(100))? {
            return Ok(());
        }
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }
            match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                _ => {}
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}

struct Editor {
    input: String, 
    character_index : usize,
    input_mode: InputMode
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

    fn sumbit_list(&mut self, app: &mut App) {
        app.todo_list.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }
}
