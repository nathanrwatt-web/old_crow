use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

pub enum Transition {
    Stay, 
    Pop,
    Push(Box<dyn Screen>),
    Quit,
}

pub trait Screen {
    fn handle_event(&mut self, ev: Event) -> Transition;
    fn draw(&mut self, frame: &mut Frame, area:Rect);
    fn footer_hint(&self) -> &str;
}
