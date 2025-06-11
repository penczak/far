use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers}, layout::Size, DefaultTerminal
};
use regex::Regex;
use walkdir::WalkDir;

pub struct App {
    pub running: bool,
    pub cursor: usize,
    pub backup_cursor: usize,
    pub hits: Vec<Hit>,
    pub files: Vec<FarFile>,
    pub terminal_size: Size,
    pub expansion: Option<Expansion>,
}

pub struct Expansion {
    pub file_name: String,
    pub relative_file_name: String,
    pub content: String,
}

impl Clone for Expansion {
    fn clone(&self) -> Self {
        Self { file_name: self.file_name.clone(), relative_file_name: self.relative_file_name.clone(), content: self.content.clone() }
    }
}

pub struct FarFile {
    pub file_name: String,
    pub relative_file_name: String,
    pub content: String,
}

pub struct Hit {
    pub index: usize,
    pub state: FarState,

    pub full_line: String,
    pub line_before_match: String,
    pub matched_text: String,
    pub line_after_match: String,

    pub file_name: String,
    pub relative_file_name: String,
    pub line_number: usize,
    pub input_pattern: InputPattern,
}

impl Clone for Hit {
    fn clone(&self) -> Self {
        Self { index: self.index.clone(), state: self.state.clone(), full_line: self.full_line.clone(), line_before_match: self.line_before_match.clone(), matched_text: self.matched_text.clone(), line_after_match: self.line_after_match.clone(), file_name: self.file_name.clone(), relative_file_name: self.relative_file_name.clone(), line_number: self.line_number.clone(), input_pattern: self.input_pattern.clone() }
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
    pub find_pattern: String,
    pub replace: String,
}

impl InputPattern {
    pub fn new(find_pattern: String, replace: String) -> Self {
        Self { find_pattern, replace }
    }
}

impl Clone for InputPattern {
    fn clone(&self) -> Self {
        Self { find_pattern: self.find_pattern.clone(), replace: self.replace.clone() }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(args: Vec<String>) -> Self {

        let mut input_patterns: Vec<InputPattern> = Vec::new();

        let mut args_iter = args.iter().skip(1); // skip executable name
        while let Some(arg) = args_iter.next() {
            let parts: Vec<&str> = arg.split(':').collect();

            if parts.len() < 2 {
                // return error
            }

            input_patterns.push(
                InputPattern::new(
                    parts[0].to_string(),
                    parts[1].to_string()
                )
            );
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
        
        let mut hits: Vec<Hit> = Vec::new();
        let mut files: Vec<FarFile> = Vec::new();

        // let mut results = vec![];
        for file_path in files_to_check {
            // println!("FILEPATH: {}", &file_path);
            let content = std::fs::read_to_string(&file_path);
            if !content.is_ok() {
                continue; // may be a non-text file like an executable
            }
            let content = content.unwrap();
            // println!("CONTENT: {}", content);

            files.push(FarFile {
                file_name: file_path.clone(),
                relative_file_name: file_path.replace(&root.to_str().unwrap(), ""),
                content: content.clone(),
            });

            for (i, line) in content.lines().enumerate() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // println!("L: {line}");
                for pattern in input_patterns.iter() {
                    let regex = Regex::new(&pattern.find_pattern).unwrap();
                    for re_match in regex.find_iter(line) {
                        // println!("match: {}, {}", re_match.start(), re_match.end());
                        hits.push(Hit {
                                state: FarState::Undecided,
                                full_line: line.to_string(),
                                line_before_match: line[..re_match.start()].to_string(),
                                matched_text: line[re_match.start()..re_match.end()].to_string(),
                                line_after_match: line[re_match.end()..].to_string(),

                                input_pattern: pattern.clone(),

                                line_number: i + 1,
                                file_name: file_path.clone(),
                                relative_file_name: file_path.replace(&root.to_str().unwrap(), ""),
                                index: hits.len(),
                            }
                        )
                    }   
                }
            }
        }
        Self {
            running: true,
            cursor: 0,
            backup_cursor: 0,
            hits: hits,
            files: files,
            expansion: None,
            terminal_size: Size::ZERO
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.terminal_size = terminal.size().unwrap();

        while self.running {
            // if self.needs_clear {
            //     terminal.draw(|frame| frame.render_widget(Clear, frame.area()))?;
            //     self.needs_clear = false;
            // }
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match crossterm::event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            crossterm::event::Event::Key(key)
                if key.kind == crossterm::event::KeyEventKind::Press => { _ = self.on_key_event(key); }
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
            KeyCode::Char('r') => self.reset_current_hit(),
            KeyCode::Char('e') => self.handle_expansion(),
            KeyCode::Enter => self.apply_changes(),
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
        let hit = self.get_current_hit_mut();
        hit.state = FarState::Take;
        self.move_cursor_down();
    }
    
    fn skip_current_hit(&mut self) {
        let hit = self.get_current_hit_mut();
        hit.state = FarState::Skip;
        self.move_cursor_down();
    }
    
    fn reset_current_hit(&mut self) {
        let hit = self.get_current_hit_mut();
        hit.state = FarState::Undecided;
    }
    
    fn handle_expansion(&mut self) {
        if self.expansion.is_some() {
            self.cursor = self.backup_cursor;
            self.backup_cursor = 0;
            self.close_expansion();
        } else {
            self.backup_cursor = self.cursor;
            self.expand_current_hit();
            self.cursor = self.get_current_hit().line_number;
        }
    }

    fn expand_current_hit(&mut self) {
        let hit = self.get_current_hit();
        self.expansion = Some(Expansion {
            file_name: hit.file_name.clone(),
            relative_file_name: hit.relative_file_name.clone(),
            content: self.files.iter().find(|f| f.file_name == hit.file_name).map_or("".to_string(), |f| f.content.clone()),
        });
    }

    fn close_expansion(&mut self) {
        self.expansion = None;
    }

    fn get_current_hit_mut(&mut self) -> &mut Hit {
        self.hits.get_mut(self.cursor as usize).unwrap()
    }

    fn get_current_hit(&self) -> &Hit {
        self.hits.get(self.cursor as usize).unwrap()
    }

    fn apply_changes(&self) {
        // get all hits that are taken
        // group by file
        // for each file, load content
        // iterate over lines that match a line number on one of the hits
        // set line to before + replace + after
    }
}
