mod app;
mod text_field;
mod todo_list;
mod menu;
mod screen;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}
