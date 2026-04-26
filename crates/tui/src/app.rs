use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Color},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};
use std::time::Duration;

use crate::menu::Menu;
use crate::screen::{Screen, Transition};

#[derive(PartialEq)]
enum Focus {
    Menu, 
    Screen,
}

pub struct App {
    menu: Menu,
    stack: Vec<Box<dyn Screen>>,
    focus: Focus,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(),
            stack: Vec::new(),
            focus: Focus::Menu,
            should_quit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;

            if !event::poll(Duration::from_millis(100))? { continue; }
            let cur_event = event::read()?;

            if let Event::Key(key) = cur_event {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.code == KeyCode::Tab {
                    self.focus = match self.focus {
                        Focus::Menu => {
                            if self.stack.is_empty() {
                                Focus::Menu
                            } else {
                                Focus::Screen
                            }
                        }
                        Focus::Screen => Focus::Menu,
                    };
                    continue;
                }
            }

            let transition = match self.focus {
                Focus::Menu => self.menu.handle_event(cur_event),
                Focus::Screen => match self.stack.last_mut() {
                    Some(top) => top.handle_event(cur_event),
                    None => Transition::Stay,
                },
            };

            match transition {
                Transition::Stay => {},
                Transition::Quit => self.should_quit = true,
                Transition::Push(screen) => {
                    self.stack.push(screen);
                    self.focus = Focus::Screen;
                },
                Transition::Pop => {
                    self.stack.pop();
                    if self.stack.is_empty() {
                        self.focus = Focus::Menu;
                    }
                }
            }
        }
        Ok(())
    }
                    
                    

    fn draw(&mut self, frame: &mut Frame) {
        let [body, footer] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let [menu_area, screen_area] = Layout::horizontal([
            Constraint::Length(20),
            Constraint::Min(0),
        ])
        .areas(body);


        self.menu.draw(frame, menu_area);

        let hint = match self.stack.last_mut() {
            Some(top) => {
                top.draw(frame, screen_area);
                if self.focus == Focus::Screen {
                    top.footer_hint()
                } else {
                    self.menu.footer_hint()
                }
            },
            None => {
                let placeholder = Paragraph::new("Select an app from the menu")
                    .style(Style::new().fg(Color::DarkGray));
                frame.render_widget(placeholder, screen_area);
                self.menu.footer_hint()
            }
        };

        frame.render_widget(Paragraph::new(hint), footer);
    }
}

