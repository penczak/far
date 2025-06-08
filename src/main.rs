use crate::app::App;
use std::env;

pub mod app;
pub mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let args: Vec<String> = env::args().collect();

    let mut buffer = std::fs::read_to_string(args[1].clone()).expect("couldnt read file");

    buffer = buffer.lines().filter(|line| {
        !line.trim().is_empty()
    }).fold(String::new(), |mut acc, str| {
        acc.push_str(str);
        acc.push('\n');
        acc
    });

    let result = App::new(&buffer).run(terminal);
    
    ratatui::restore();
    
    result
}
