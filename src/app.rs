use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers}, layout::Size, style::Stylize, symbols::line, text::{Line, Span}, DefaultTerminal
};
use regex::Regex;
use walkdir::WalkDir;

pub struct App {
    pub running: bool,
    pub cursor: u8,
    pub hits: Vec<Hit>,
    pub terminal_size: Size,
}

pub struct Hit {
    pub index: u8,
    pub state: FarState,

    pub full_line: String,
    pub line_before_match: String,
    pub matched_text: String,
    pub line_after_match: String,

    pub file_name: String,
    pub line_number: u8,

}

impl Clone for Hit {
    fn clone(&self) -> Self {
        Self { index: self.index.clone(), state: self.state.clone(), full_line: self.full_line.clone(), line_before_match: self.line_before_match.clone(), matched_text: self.matched_text.clone(), line_after_match: self.line_after_match.clone(), file_name: self.file_name.clone(), line_number: self.line_number.clone() }
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

pub struct InputPattern {
    pub key: u8,
    pub find_pattern: String,
    pub replace: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(args: Vec<String>) -> Self {
        let mut hits: Vec<Hit> = Vec::new();

        let mut input_patterns: Vec<InputPattern> = Vec::new();

        let mut key = 0;
        let mut args_iter = args.iter().skip(1); // skip executable name
        while let Some(arg) = args_iter.next() {
            let parts: Vec<&str> = arg.split(':').collect();

            if parts.len() < 2 {
                // return error
            }

            input_patterns.push(InputPattern {
                key: key,
                find_pattern: parts[0].to_string(),
                replace: parts[1].to_string(),
            });
            key = key + 1;
        }

        // get every file (r) in directory
        let mut files_to_check: Vec<String> = Vec::new();
        let root = std::env::current_dir().expect("program should be able to get current dir");

        for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                // println!("{}", entry.path().to_str().unwrap().to_string());
                let entry_path = entry.path().to_str().expect("files' paths should be UTF8 characters");
                files_to_check.push(entry_path.to_string());
            }
        }

        let regex = Regex::new(r"some").unwrap();

        // let mut results = vec![];
        for file_path in files_to_check {
            // println!("FILEPATH: {}", &file_path);
            let content = std::fs::read_to_string(&file_path);
            if !content.is_ok() {
                continue; // may be a non-text file like an executable
            }
            let content = content.unwrap();
            // println!("CONTENT: {}", content);

            for (i, line) in content.lines().enumerate() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // println!("L: {line}");

                for re_match in regex.find_iter(line) {
                    // println!("match: {}, {}", re_match.start(), re_match.end());
                    hits.push(Hit {
                            state: FarState::Undecided,
                            full_line: line.to_string(),
                            line_before_match: line[..re_match.start()].to_string(),
                            matched_text: line[re_match.start()..re_match.end()].to_string(),
                            line_after_match: line[re_match.end()..].to_string(),
                            line_number: i.try_into().unwrap(),
                            file_name: file_path.clone(),
                            index: hits.len().try_into().unwrap(),
                        }
                    )
                }
            }
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

    fn get_current_hit(&mut self) -> &mut Hit {
        self.hits.get_mut(self.cursor as usize).unwrap()
    }
}
