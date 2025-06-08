use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers}, layout::Size, style::Stylize, text::{Line, Span}, DefaultTerminal
};

/// Application.
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    // Current position in buffer
    pub cursor: u8,
    // Currently visable window
    pub hits: Vec<Hit<'a>>,
    pub terminal_size: Size,
}

pub struct Hit<'a> {
    pub state: FarState,
    pub display: Line<'a>,
    pub spans: Vec<Span<'a>>,
    pub content: &'a str,
    pub line_number: u8,
}

pub enum FarState {
    Undecided,
    Take,
    Skip,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(buffer: &'a str) -> Self {
        let mut hits: Vec<Hit> = Vec::new();
        let mut lines = buffer.lines();

        for i in 0..lines.clone().count() {
            let line: &str = lines.next().unwrap();
            let spans: Vec<Span> = line.chars().map(|char| {
                if char.eq_ignore_ascii_case(&'o') {
                    char.to_string().black().on_white()
                } else {
                    char.to_string().white().on_black()
                }
            }).collect();

            hits.push(
                Hit {
                    state: FarState::Undecided,
                    content: line,
                    display: Line::from(spans.clone()),
                    spans: spans,
                    line_number: i.try_into().unwrap(),
                }
            )
        }
        // let hits = buffer.lines()
        //     .map(|ix, line| {
        //         let spans: Vec<Span> = line.chars().map(|char| {
        //             if char.eq_ignore_ascii_case(&'o') {
        //                 char.to_string().black().on_white()
        //             } else {
        //                 char.to_string().white().on_black()
        //             }
        //         }).collect();

        //         Hit {
        //             state: FarState::Undecided,
        //             content: line,
        //             display: Line::from(spans),
        //             line_number: 
        //         }
        //     })
        //     .collect();
        Self {
            running: true,
            cursor: 0,
            hits: hits,
            terminal_size: Size::ZERO
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.terminal_size = terminal.size().unwrap();

        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
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
}
