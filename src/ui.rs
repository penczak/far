use ratatui::{
    buffer::Buffer, layout::{Alignment, Rect}, style::{Color, Style, Stylize}, text::{Line, Span}, widgets::{Block, BorderType, Paragraph, Widget}
};

use crate::app::App;

impl Widget for &App<'_> {
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let height: u8 = self.terminal_size.height.try_into().unwrap();
        // let width: u8 = self.terminal_size.width.try_into().unwrap();

        let block = Block::bordered()
            .title(" far ")
            .title_alignment(Alignment::Center)
            .title_bottom(format!(
                "Cursor: {}",
                self.cursor
            ))
            .border_type(BorderType::Plain);


        // let mut paragraph_text = String::new();
        let mut paragraph_lines: Vec<Line> = Vec::new();

        let top_white_space: u8 = (height / 2).saturating_sub(self.cursor);
        for _ in 0..top_white_space {
            // paragraph_text.push('\n');
            paragraph_lines.push(Line::from("\n"));
        }
        
        let lines_to_skip: u8 = if top_white_space > 0 { 0 } else { (self.cursor - (height / 2)).into() };

        let lines_to_use = self.hits.iter()
            .skip(lines_to_skip.into())
            .take(height.into());

        lines_to_use
            .for_each(|h| {
                let spans: Vec<Span> = h.spans.clone();
                // let mut line: Line = h.display.clone();
                if h.line_number == self.cursor {
                    let line = Line::from(
                        spans.iter()
                            .map(|s| { s.clone().red() })
                            .collect::<Vec<Span>>()
                    );
                    paragraph_lines.push(line);
                } else {
                    paragraph_lines.push(h.display.clone());
                }
            });

        // let mut lines_to_use = self.hits.iter()
        //     .skip(lines_to_skip.into())
        //     .take(height.into());

        // for i in 0..height {
        //     let hit = lines_to_use.next().unwrap();
        //     let line: Line = if i == self.cursor {
        //         hit.display.clone().red()
        //     } else {
        //         hit.display.clone()
        //     };
        //     paragraph_lines.push(line);
        // }

        // let paragraph = Paragraph::new(paragraph_text)
        let paragraph = Paragraph::new(paragraph_lines)
            .block(block)
            // .fg(Color::Cyan)
            // .bg(Color::Black)
            // .centered()
            ;

        // self.top_view = String::new();

        
        // let lines_to_skip: u8 = if top_white_space > 0 { 0 } else { self.cursor.into() };

        // let content: String = self.buffer.lines()
        //     .skip(lines_to_skip.into())
        //     .take(99)
        //     .fold(String::new(), |mut acc, line| {
        //         acc.push_str(line);
        //         acc.push('\n');
        //         acc
        //     });

        // self.top_view.push_str(&content);

        paragraph.render(area, buf);
    }
}
