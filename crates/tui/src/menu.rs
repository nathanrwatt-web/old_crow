use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::ListState;


pub enum MenuAction {
    EnterApp,
    QuitApplication, 
    None,
}


pub struct Menu {
   pub application_list: Vec<String>,
   pub state: ListState,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            application_list: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn handle_events(&mut self) -> Result<MenuAction> {
        
        if !event::poll(std::time::Duration::from_millis(100))? {
            return Ok(MenuAction::None);
        }
        
        let Event::Key(key) = event::read()? else {
            return Ok(MenuAction::None);
        };

        if key.kind != KeyEventKind::Press {
            return Ok(MenuAction::None);
        }

        let action = match key.code {
            KeyCode::Char('q') => {
                MenuAction::QuitApplication
            },
            KeyCode::Up | KeyCode::Char('i') => {
                self.state.select_previous();
                MenuAction::None
            },
            KeyCode::Down | KeyCode::Char('k') => {
                self.state.select_next();
                MenuAction::None
            },
            KeyCode::Enter => {
                MenuAction::EnterApp
            }
        };
        Ok(action)
    }
}
