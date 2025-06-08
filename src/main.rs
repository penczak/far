use crate::app::App;
use std::env;

pub mod app;
pub mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let args: Vec<String> = env::args().collect();

    let result = App::new(args).run(terminal);
    ratatui::restore();
    result
}
