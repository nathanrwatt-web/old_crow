mod app;
mod editor;
mod todo_list;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}
