use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use std::io;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: u8,
    // Current position in buffer
    pub cursor: u8,
    // File being scrolled
    pub buffer: String,
    // Currently visable window
    pub view: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            cursor: 0,
            buffer: String::new(),
            view: String::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(args: Vec<String>) -> Self {
        let mut s = Self::default();
        s.buffer = std::fs::read_to_string(args[1].clone()).expect("couldnt read file");
        s
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        let height = terminal.size().unwrap().height;
        let width = terminal.size().unwrap().width;

        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
            self.recalculate_view(height, width);
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match crossterm::event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            crossterm::event::Event::Key(key)
                if key.kind == crossterm::event::KeyEventKind::Press => { self.on_key_event(key); }
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn on_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.quit(),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.quit();
            }
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Up => self.move_cursor_up(),
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor = self.cursor.saturating_add(1);
    }
    pub fn move_cursor_up(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }
    
    fn recalculate_view(&mut self, height: u16, width: u16) {

        let height: u8 = height.try_into().unwrap();
        let top_white_space: u8 = (height / 2).saturating_sub(self.cursor * 2);

        self.view = String::new();

        for _ in 0..top_white_space {
            self.view.push('\n');
        }
        
        let lines_to_skip: u8 = if top_white_space > 0 { 0 } else { self.cursor.into() };

        let content: String = self.buffer.lines()
            .skip(lines_to_skip.into())
            .take(99)
            .fold(String::new(), |mut acc, line| {
                acc.push_str(line);
                acc.push('\n');
                acc
            });

        self.view.push_str(&content);
    }
}
