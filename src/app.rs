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
    pub file_name: String,
    pub line_number: u8,
}

impl Clone for Hit<'_> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            display: self.display.clone(),
            spans: self.spans.clone(),
            content: self.content,
            file_name: self.file_name.clone(),
            line_number: self.line_number.clone(),
        }
    }
}

pub enum FarState {
    Undecided,
    Take,
    Skip,
}

impl Clone for FarState {
    fn clone(&self) -> Self {
        match self {
            Self::Undecided => Self::Undecided,
            Self::Take => Self::Take,
            Self::Skip => Self::Skip,
        }
    }
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
                    file_name: "rust_book.txt".to_string(),
                }
            )
        }
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
            KeyCode::Char('t') => self.take_current_hit(),
            KeyCode::Char('s') => self.skip_current_hit(),
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
    
    fn take_current_hit(&mut self) {
        let hit = self.get_current_hit();
        hit.state = FarState::Take;
        self.move_cursor_down();
    }
    
    fn skip_current_hit(&mut self) {
        let hit = self.get_current_hit();
        hit.state = FarState::Skip;
        self.move_cursor_down();
    }

    fn get_current_hit(&mut self) -> &mut Hit<'a> {
        self.hits.get_mut(self.cursor as usize).unwrap()
    }
}
