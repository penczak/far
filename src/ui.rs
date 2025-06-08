use ratatui::{
    buffer::Buffer, layout::{Alignment, Rect}, style::Stylize, text::{Line, Span}, widgets::{Block, BorderType, Paragraph, Widget, Wrap}
};

use crate::app::{App, Hit};

fn prepend_file_name<'a>(hit: Hit<'a>, spans: Vec<Span<'a>>) -> Vec<Span<'a>> {
    let mut spans = spans.clone();
    
    spans.insert(0, Span::from(hit.file_name.clone()).cyan());
    spans.insert(1, Span::from(" "));
    
    let indicator = match hit.state {
        crate::app::FarState::Undecided => Span::from("( )"),
        crate::app::FarState::Take => Span::from("(t)").green(),
        crate::app::FarState::Skip => Span::from("(s)").red(),
    };

    spans.insert(2, Span::from(indicator));
    spans
}

impl Widget for &App<'_> {
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let height: u8 = self.terminal_size.height.try_into().unwrap();
        // let width: u8 = self.terminal_size.width.try_into().unwrap();

        let block = Block::bordered()
            .title(" Find And Replace ")
            .title_alignment(Alignment::Center)
            .title_bottom(format!(
                "Cursor: {}",
                self.cursor
            ))
            .border_type(BorderType::Plain);

        let mut paragraph_lines: Vec<Line> = Vec::new();

        let top_white_space: u8 = (height / 2).saturating_sub(self.cursor);
        for _ in 0..top_white_space {
            paragraph_lines.push(Line::from("\n"));
        }
        
        let lines_to_skip: u8 = if top_white_space > 0 { 0 } else { (self.cursor - (height / 2)).into() };

        let lines_to_use = self.hits.iter()
            .skip(lines_to_skip.into())
            .take(height.into());

        lines_to_use
            .for_each(|h| {
                // let mut spans: Vec<Span> = h.spans.clone();
                let mut spans = prepend_file_name(h.clone(), h.spans.clone());

                let mut line;

                if h.line_number != self.cursor {
                    spans.insert(3, Span::from(" "));
                    line = Line::from(spans);
                    line = line.dim();
                } else {
                    spans.insert(3, Span::from(">"));
                    line = Line::from(spans);
                }
                paragraph_lines.push(line);
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
            .wrap(Wrap {
                trim: false
            })
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
