use crate::app::App;
use std::env;

pub mod app;
pub mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let args: Vec<String> = env::args().collect();

    let buffer = std::fs::read_to_string(args[1].clone()).expect("couldnt read file");

    let result = App::new(&buffer).run(terminal);
    
    ratatui::restore();
    
    result
}
