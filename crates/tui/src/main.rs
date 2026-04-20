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
