use ratatui::{
    buffer::Buffer, layout::{Alignment, Rect}, style::{Style, Stylize}, text::{Line, Span}, widgets::{Block, BorderType, Paragraph, Widget, Wrap}
};

use crate::app::{App, Hit};

fn format_hit_to_spans<'a>(app: &'a App, hit: &'a Hit) -> Line<'a> {
    // file_name.txt:15 ( ) this is my |match|->|newvalue|ing text
    let mut spans: Vec<Span> = Vec::new();
    let mut i = 0;
    
    let file_name_style = match hit.state {
        crate::app::FarState::Undecided => Style::default(),
        crate::app::FarState::Take => Style::new().green(),
        crate::app::FarState::Skip => Style::new().red(),
    };
    spans.insert(i, Span::from(hit.relative_file_name.clone()).style(file_name_style)); i += 1;
    spans.insert(i, Span::from(":").style(file_name_style)); i += 1;
    spans.insert(i, Span::from(hit.line_number.to_string()).style(file_name_style)); i += 1;

    spans.insert(i, Span::from(" ")); i += 1;
    
    let indicator = match hit.state {
        crate::app::FarState::Undecided => Span::from("( )"),
        crate::app::FarState::Take => Span::from("(t)").green(),
        crate::app::FarState::Skip => Span::from("(s)").red(),
    };
    spans.insert(i, Span::from(indicator)); i += 1;
    spans.insert(i, Span::from(" ")); i += 1;
    spans.insert(i, Span::from(hit.line_before_match.clone())); i += 1;
    match hit.state {
        crate::app::FarState::Undecided => {
            spans.insert(i, Span::from(hit.matched_text.clone()).black().on_dark_gray()); i += 1;
            spans.insert(i, Span::from("->")); i += 1;
            spans.insert(i, Span::from(hit.input_pattern.replace.clone()).black().on_white()); i += 1;
        },
        crate::app::FarState::Take => {
            spans.insert(i, Span::from(hit.input_pattern.replace.clone()).on_green()); i += 1;
        },
        crate::app::FarState::Skip => {
            spans.insert(i, Span::from(hit.matched_text.clone()).on_red()); i += 1;
        },
    };
    spans.insert(i, Span::from(hit.line_after_match.clone())); // i += 1;

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
                " Cursor: {} Backup: {} Expand: {}",
                self.cursor,
                self.backup_cursor,
                self.expansion.is_some(),
            ))
            .border_type(BorderType::Plain);

        let mut paragraph_lines: Vec<Line> = Vec::new();

        let top_white_space: usize = (height / 2).saturating_sub(self.cursor);
        
        for _ in 0..top_white_space {
            paragraph_lines.push(Line::from("\n"));
        }

        let lines_to_skip: usize = if top_white_space > 0 { 0 } else { self.cursor - (height / 2) };

        if self.expansion.is_some() {
            let expansion = self.expansion.clone().unwrap();
            let lines_to_use = expansion.content.lines()
                .skip(lines_to_skip)
                .take(height);
    
            for (i, str) in lines_to_use.enumerate() {
                let mut spans: Vec<Span> = Vec::new();
                spans.push(Span::from(expansion.relative_file_name.clone()));
                spans.push(Span::from(":"));
                spans.push(Span::from((i + lines_to_skip).to_string()));
                spans.push(Span::from(" "));
                spans.push(Span::from(str.to_string().clone()));
                let line = Line::from(spans);

                paragraph_lines.push(line);
            }
        } else {
            let lines_to_use = self.hits.iter()
                .skip(lines_to_skip)
                .take(height);
    
            lines_to_use
                .for_each(|h| {
                    // let mut spans: Vec<Span> = h.spans.clone();
                    let line = format_hit_to_spans(&self, h);
    
                    paragraph_lines.push(line);
                });
        }

        let paragraph = Paragraph::new(paragraph_lines)
            .block(block)
            .wrap(Wrap {
                trim: false
            })
            // .fg(Color::Cyan)
            // .bg(Color::Black)
            // .centered()
            ;

        // Clear::render(Clear::default(), area, buf);
        paragraph.render(area, buf);
    }
}
