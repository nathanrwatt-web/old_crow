use anyhow::Result;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

use crate::editor::{Editor, EditorAction, InputMode};
use crate::todo_list::{TodoList, TodoListAction};
use crate::menu::{Menu};

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
            focus: Focus::Editor,
            should_quit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            match self.focus {
                Focus::Menu => {
                    match self.menu.handle_events()? {
                        _ => todo!()
                    }
                }
                Focus::Editor => {
                    match self.editor.handle_events()? {
                        EditorAction::None => {},
                        EditorAction::TodoList => { self.focus = Focus::TodoList; }
                        EditorAction::Sumbit(text) => self.todo_list.push(text),
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
        
        let [header, body, editor_area, footer] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3), 
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let title = Paragraph::new(" productivity ").bold().block(Block::bordered());
        frame.render_widget(title, header);

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

        frame.render_stateful_widget(content, body, state);

        let edit_stuff = Paragraph::new(self.editor.input.as_str()).block(Block::bordered());
        frame.render_widget(edit_stuff, editor_area);

        let help = match self.editor.input_mode {
            InputMode::Normal => {
                Paragraph::new("<q> to quit, <e> to enter edit, <t> to enter tasks").dim()
            }
            InputMode::Editing => {
                Paragraph::new("<esc> to normal, <backspace> to delete").dim()

            },
        };
        frame.render_widget(help, footer);
    }
}

