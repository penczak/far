use ratatui::{
    buffer::Buffer, layout::{Alignment, Rect}, style::{Style, Stylize}, text::{Line, Span}, widgets::{Block, BorderType, Paragraph, Widget, Wrap}
};

use crate::app::{App, Hit};

fn format_hit_to_spans<'a>(app: &'a App, hit: &'a Hit) -> Line<'a> {
    // file_name.txt:15 ( ) this is my |match|ing text
    let mut spans: Vec<Span> = Vec::new();
    let mut i = 0;
    spans.insert(i, Span::from(hit.file_name.clone())); i += 1;
    spans.insert(i, Span::from(" ")); i += 1;
    
    let indicator = match hit.state {
        crate::app::FarState::Undecided => Span::from("( )"),
        crate::app::FarState::Take => Span::from("(t)").green(),
        crate::app::FarState::Skip => Span::from("(s)").red(),
    };
    spans.insert(i, Span::from(indicator)); i += 1;
    if hit.index != app.cursor {
        spans.insert(i, Span::from(" ")); i += 1;
    } else {
        spans.insert(i, Span::from(">")); i += 1;
    }
    spans.insert(i, Span::from(hit.line_before_match.clone())); i += 1;
    spans.insert(i, Span::from(hit.matched_text.clone()).black().on_white()); i += 1;
    spans.insert(i, Span::from(hit.line_after_match.clone())); i += 1;

    let mut style = Style::new();

    if hit.index != app.cursor {
        style = style.dim();
    }

    Line::from(spans).style(style)
}

impl Widget for &App {
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let height: usize = self.terminal_size.height.try_into().unwrap();
        // let width: usize = self.terminal_size.width.try_into().unwrap();

        let block = Block::bordered()
            .title(" Find And Replace ")
            .title_alignment(Alignment::Center)
            .title_bottom(format!(
                "Cursor: {}",
                self.cursor
            ))
            .border_type(BorderType::Plain);

        let mut paragraph_lines: Vec<Line> = Vec::new();

        let top_white_space: usize = (height / 2).saturating_sub(self.cursor);
        for _ in 0..top_white_space {
            paragraph_lines.push(Line::from("\n"));
        }
        
        let lines_to_skip: usize = if top_white_space > 0 { 0 } else { self.cursor - (height / 2) };

        let lines_to_use = self.hits.iter()
            .skip(lines_to_skip)
            .take(height);

        lines_to_use
            .for_each(|h| {
                // let mut spans: Vec<Span> = h.spans.clone();
                let line = format_hit_to_spans(&self, h);

                paragraph_lines.push(line);
            });

        let paragraph = Paragraph::new(paragraph_lines)
            .block(block)
            .wrap(Wrap {
                trim: false
            })
            // .fg(Color::Cyan)
            // .bg(Color::Black)
            // .centered()
            ;

        paragraph.render(area, buf);
    }
}
