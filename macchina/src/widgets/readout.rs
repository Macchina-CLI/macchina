use crate::data::Readout;
use crate::theme::Theme;
use std::cmp::max;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Margin, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, Clear, Paragraph, Widget};

pub struct ReadoutList<'a> {
    block: Option<Block<'a>>,
    style: Style,
    items: Vec<Readout<'a>>,
    theme: Box<dyn Theme>,
    block_inner_margin: Margin,
}

impl<'a, 'b> ReadoutList<'a> {
    pub fn new<T>(items: T, theme: Box<dyn Theme>) -> ReadoutList<'a>
    where
        T: Into<Vec<Readout<'a>>>,
    {
        ReadoutList {
            block: None,
            style: Style::default(),
            items: items.into(),
            theme,
            block_inner_margin: Margin {
                horizontal: 0,
                vertical: 0,
            },
        }
    }

    pub fn block(mut self, block: Block<'a>) -> ReadoutList<'a> {
        self.block = Some(block);
        self
    }

    pub fn add_item(mut self, item: Readout<'a>) -> ReadoutList<'a> {
        self.items.push(item);
        self
    }

    pub fn theme(mut self, theme: Box<dyn Theme>) -> ReadoutList<'a> {
        self.theme = theme;
        self
    }

    pub fn style(mut self, style: Style) -> ReadoutList<'a> {
        self.style = style;
        self
    }

    pub fn block_inner_margin(mut self, margin: Margin) -> ReadoutList<'a> {
        self.block_inner_margin = margin;
        self
    }
}

impl<'a, 'b> Widget for ReadoutList<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let list_area = match &self.block {
            Some(b) => {
                let inner_area = b.inner(area);
                inner_area.inner(&self.block_inner_margin)
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        if self.items.is_empty() {
            return;
        }

        let mut height = 0;
        let max_key_width = self
            .items
            .iter()
            .map(|i| self.theme.key(&i.0, self.theme.default_abbreviation()))
            .map(|i| Text::raw(i).width())
            .max()
            .unwrap();

        let mut max_line_width: u16 = 0;

        for item in self.items.iter_mut() {
            let readout_text = &item.1;
            let readout_key = Text::styled(
                self.theme.key(&item.0, self.theme.default_abbreviation()),
                Style::default().fg(self.theme.get_color()),
            );

            let theme_separator = Text::styled(
                self.theme.get_separator(),
                Style::default().fg(self.theme.get_separator_color()),
            );

            let area = Rect {
                x: list_area.x,
                y: list_area.y + height,
                width: list_area.width,
                height: readout_text.height() as u16,
            };

            let constraints = [
                Constraint::Length(max_key_width as u16 + self.theme.get_padding() as u16),
                Constraint::Length(theme_separator.width() as u16),
                Constraint::Length(self.theme.get_padding() as u16),
                Constraint::Length(readout_text.width() as u16),
            ];

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints)
                .split(area);

            height += readout_text.height() as u16;

            Paragraph::new(readout_key).render(layout[0], buf);
            Paragraph::new(theme_separator).render(layout[1], buf);
            Paragraph::new(readout_text.to_owned()).render(layout[3], buf);

            let mut line_width: u16 = 0;
            for c in &constraints {
                if let Constraint::Length(l) = c {
                    line_width += l;
                }
            }
            if line_width > max_line_width {
                max_line_width = line_width;
            }
        }

        if let Some(b) = self.block.take() {
            b.render(
                Rect {
                    x: 0,
                    y: 0,
                    width: max_line_width + 2 + self.block_inner_margin.horizontal * 2,
                    height: height + 2 + self.block_inner_margin.vertical * 2,
                },
                buf,
            );
        }
    }
}
