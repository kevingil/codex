use crate::render::line_utils::push_owned_lines;
use crate::render::renderable::ColumnRenderable;
use crate::render::renderable::Renderable;
use crate::wrapping::RtOptions;
use crate::wrapping::word_wrap_line;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::style::Stylize as _;
use ratatui::text::Line;
use unicode_width::UnicodeWidthStr;

pub(crate) fn selection_option_row(
    index: usize,
    label: String,
    is_selected: bool,
) -> Box<dyn Renderable> {
    let prefix_text = if is_selected {
        format!("› {}. ", index + 1)
    } else {
        format!("  {}. ", index + 1)
    };
    let style = if is_selected {
        Style::default().cyan()
    } else {
        Style::default()
    };
    let prefix_width = UnicodeWidthStr::width(prefix_text.as_str());

    // Create a wrapper that wraps the text at render time with the correct width
    Box::new(SelectionOptionRenderable {
        prefix_text,
        prefix_width,
        label,
        style,
    })
}

struct SelectionOptionRenderable {
    prefix_text: String,
    prefix_width: usize,
    label: String,
    style: Style,
}

impl SelectionOptionRenderable {
    fn wrap_text(&self, width: usize) -> Vec<Line<'static>> {
        let label_width = width.saturating_sub(self.prefix_width).max(1);

        // Create initial and subsequent indents
        let initial_indent = Line::from(self.prefix_text.clone()).style(self.style);
        let subsequent_indent = Line::from(" ".repeat(self.prefix_width)).style(self.style);

        // Wrap the label text
        let line = Line::from(self.label.clone()).style(self.style);
        let opts = RtOptions::new(label_width)
            .initial_indent(initial_indent)
            .subsequent_indent(subsequent_indent);

        // Convert borrowed lines to owned lines
        let wrapped = word_wrap_line(&line, opts);
        let mut owned_lines = Vec::new();
        push_owned_lines(&wrapped, &mut owned_lines);
        owned_lines
    }
}

impl Renderable for SelectionOptionRenderable {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let available_width = area.width as usize;
        if available_width == 0 {
            return;
        }

        let wrapped = self.wrap_text(available_width);

        // Render the wrapped lines
        let mut column = ColumnRenderable::new();
        for wrapped_line in wrapped {
            column.push(wrapped_line);
        }
        column.render(area, buf);
    }

    fn desired_height(&self, width: u16) -> u16 {
        let available_width = width as usize;
        if available_width == 0 {
            return 1;
        }

        let wrapped = self.wrap_text(available_width);
        wrapped.len() as u16
    }
}
