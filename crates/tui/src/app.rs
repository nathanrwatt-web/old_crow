use anyhow::Result;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

use crate::editor::{Editor, EditorAction, InputMode};
use crate::todo_list::{TodoList, TodoListAction, Priority};
use crate::menu::{Menu, MenuAction };

enum Focus {
    Menu,
    TodoList,
    Editor,
}

pub struct App {
    menu: Menu,
    todo_list: TodoList,
    editor: Editor,
    focus: Focus,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            todo_list: TodoList::new(),
            editor: Editor::new(),
            menu: Menu::new(),
            focus: Focus::Menu,
            should_quit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            match self.focus {
                Focus::Menu => {
                    match self.menu.handle_events()? {
                        MenuAction::QuitApplication => {
                            self.should_quit = true;
                        },
                        MenuAction::EnterApp => {
                            match self.menu.state.selected() {
                                Some(0) => { self.focus = Focus::TodoList; },
                                _ => { self.focus = Focus::Menu; }
                                }
                            },
                        _ => { self.focus = Focus::Menu; }
                    }
                },
                Focus::Editor => {
                    match self.editor.handle_events()? {
                        EditorAction::None => {},
                        EditorAction::TodoList => { self.focus = Focus::TodoList; }
                        EditorAction::Sumbit(text) => self.todo_list.push(text, String::from("Date under work"), Priority::Low),
                        EditorAction::Quit => self.should_quit = true,
                    }
                },
                Focus::TodoList => {
                    match self.todo_list.handle_events()? {
                        TodoListAction::None => {},
                        TodoListAction::Quit => {self.focus = Focus::Menu;}
                        TodoListAction::Delete => {
                            if let Some(selected) = self.todo_list.state.selected() {
                                if selected < self.todo_list.item_list.len() {
                                    self.todo_list.item_list.remove(selected);
                                }

                                if self.todo_list.item_list.is_empty() {
                                    self.todo_list.state.select(None);
                                }
                                else if selected >= self.todo_list.item_list.len() {
                                    self.todo_list.state.select(Some(self.todo_list.item_list.len() -1));
                                }
                                // else selected stays same, points to new item
                            }
                        }
                        TodoListAction::Edit => { self.focus = Focus::Editor; },
                    }
                },
            }
        }
        Ok(())
    }
                    
                    

    fn draw(&mut self, frame: &mut Frame) {
        let [body, editor_area] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(4),
        ])
        .areas(frame.area());

        let [menu_area, cur_app] = Layout::horizontal([
            Constraint::Length(20),
            Constraint::Min(0),
        ])
        .areas(body);

        let edit_stuff = Paragraph::new(self.editor.input.as_str()).block(Block::bordered());
        frame.render_widget(edit_stuff, editor_area);
        
        let Menu { application_list, state } = &mut self.menu;
        let menu_items: Vec<ListItem> = application_list.iter().map(|item| ListItem::new(item.as_str())).collect();
        let menu_content = List::new(menu_items)
            .highlight_style(Style::new().reversed())
            .highlight_symbol("> ")
            .scroll_padding(1)
            .block(Block::bordered());

        frame.render_stateful_widget(menu_content, menu_area, state);
        
        // destructure todo list for borrowing mutable refereinces  
        let TodoList { item_list, state } = &mut self.todo_list;

        let items: Vec<ListItem> = item_list
            .iter()
            .map(|item| ListItem::new(item.item_name.as_str()))
            .collect();

        let content = List::new(items)
            .highlight_style(Style::new().reversed())
            .highlight_symbol("> ")
            .scroll_padding(1);

        frame.render_stateful_widget(content, cur_app, state);
    }
}

