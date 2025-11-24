use crate::data::{Readout, ReadoutKey};
use crate::theme::components::{Palette, PaletteType};
use crate::theme::Theme;
use ratatui::buffer::Buffer;
use ratatui::layout::{Margin, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph, Widget};
use std::collections::HashMap;

pub struct ReadoutList<'a> {
    block: Option<Block<'a>>,
    style: Style,
    items: Vec<Readout<'a>>,
    theme: &'a Theme,
    block_inner_margin: Margin,
}

impl<'a> ReadoutList<'a> {
    pub fn new<T>(items: T, theme: &'a Theme) -> ReadoutList<'a>
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

    pub fn theme(mut self, theme: &'a Theme) -> ReadoutList<'a> {
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

impl<'a> Widget for ReadoutList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let list_area = match &self.block {
            Some(b) => {
                let inner_area = b.inner(area);
                inner_area.inner(self.block_inner_margin)
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
        let keys = self.keys_to_text(self.theme);
        let max_key_width = Self::get_max_key_width(&keys);
        let themed_separator = Self::get_themed_separator(
            self.theme.get_separator(),
            &self.theme.get_separator_color(),
        );

        let mut max_line_width: u16 = 0;

        for item in self.items.iter().filter(|&f| f.1.is_ok()) {
            //it's ok to unwrap, because we filtered out everything that is not a valid Option<T>.
            let readout_data = item.1.as_ref().unwrap();
            let readout_key = keys.get(&item.0).unwrap();

            let list_item_area = Rect {
                x: list_area.x,
                y: list_area.y + height,
                width: list_area.width,
                height: readout_data.height() as u16,
            };

            let constraints =
                self.create_item_constraints(max_key_width, &themed_separator, readout_data);
            let layout = Self::create_layout(&list_item_area, &constraints);

            let total_line_width = constraints.iter().sum::<u16>();
            if total_line_width > max_line_width {
                max_line_width = total_line_width;
            }

            let mut layout_iter = layout.iter();
            if self.theme.get_padding() > 0 {
                layout_iter.next();
            }

            Paragraph::new(readout_key.clone()).render(*layout_iter.next().unwrap(), buf);

            if !self.theme.get_separator().is_empty() {
                Paragraph::new(themed_separator.clone()).render(*layout_iter.next().unwrap(), buf);
            }

            layout_iter.next();
            Paragraph::new(readout_data.to_owned()).render(*layout_iter.next().unwrap(), buf);
            height += readout_data.height() as u16;
        }

        self.print_palette(
            buf,
            &list_area,
            &mut height,
            &mut max_line_width,
            self.theme.get_palette(),
        );

        Self::render_block(
            self.block,
            buf,
            area.x,
            area.y,
            height,
            max_line_width,
            &self.block_inner_margin,
        );
    }
}

impl<'a> ReadoutList<'a> {
    fn print_palette(
        &self,
        buf: &mut Buffer,
        list_area: &Rect,
        height: &mut u16,
        max_line_width: &mut u16,
        palette: &Palette,
    ) {
        if !palette.is_visible() {
            return;
        }

        let light_colors = [
            Color::DarkGray,
            Color::LightRed,
            Color::LightGreen,
            Color::LightYellow,
            Color::LightBlue,
            Color::LightMagenta,
            Color::LightCyan,
            Color::White,
        ];

        let dark_colors = [
            Color::Black,
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::Gray,
        ];

        let span_vector = |colors: &[Color]| -> Vec<_> {
            let mut spans: Vec<Span<'_>> = Vec::with_capacity(colors.len() * 2);
            let spacing_span = Span::raw(" ".repeat(palette.get_spacing()));
            if let Some(glyph) = palette.get_glyph() {
                for c in colors {
                    spans.push(Span::styled(
                        glyph.to_owned(),
                        Style::default().fg(c.to_owned()),
                    ));
                    spans.push(spacing_span.clone());
                }
            } else {
                for c in colors {
                    spans.push(Span::styled("   ", Style::default().bg(c.to_owned())));
                    spans.push(spacing_span.clone());
                }
            }
            spans.pop();
            spans
        };

        let spans = match palette.get_type() {
            PaletteType::Light => vec![Line::from(span_vector(&light_colors))],
            PaletteType::Dark => vec![Line::from(span_vector(&dark_colors))],
            PaletteType::Full => vec![
                Line::from(span_vector(&dark_colors)),
                Line::from(span_vector(&light_colors)),
            ],
        };

        let mut width_values = vec![];
        if self.theme.get_padding() > 0 {
            width_values.push(self.theme.get_padding())
        }
        if let Some(span_max_width) = spans.iter().map(|s| s.width()).max() {
            width_values.push(span_max_width);
        }
        let palette_max_line_width: usize = width_values.iter().sum();
        if palette_max_line_width as u16 > *max_line_width {
            *max_line_width = palette_max_line_width as u16;
        }

        let padding = self.theme.get_padding() as u16;

        let area = Rect::new(
            list_area.x + padding,
            list_area.y + *height + 1,
            list_area.width - padding,
            spans.len() as u16,
        );

        Paragraph::new(spans).render(area, buf);

        *height += area.height + 1;
    }

    fn keys_to_text(&self, theme: &Theme) -> HashMap<ReadoutKey, Text<'_>> {
        let style = Style::default().fg(theme.get_key_color());

        self.items
            .iter()
            .map(|i| (i.0, Text::styled(theme.key(&i.0).to_string(), style)))
            .collect()
    }

    fn get_max_key_width(keys: &HashMap<ReadoutKey, Text>) -> usize {
        keys.iter().map(|i| i.1.width()).max().unwrap()
    }

    fn render_block(
        block: Option<Block<'a>>,
        buf: &mut Buffer,
        x: u16,
        y: u16,
        content_height: u16,
        content_width: u16,
        margin: &Margin,
    ) {
        if let Some(block) = block {
            block.render(
                Rect {
                    x,
                    y,
                    width: content_width + 2 + margin.horizontal * 2,
                    height: content_height + 2 + margin.vertical * 2,
                },
                buf,
            );
        }
    }

    fn get_themed_separator(separator: &'a str, sep_color: &Color) -> Text<'a> {
        Text::styled(separator, Style::default().fg(*sep_color))
    }
}

impl<'a> ReadoutList<'a> {
    fn create_item_constraints(
        &self,
        max_key_width: usize,
        themed_separator: &Text,
        readout_data: &Text,
    ) -> Vec<u16> {
        let mut values = vec![];

        if self.theme.get_separator().is_empty() {
            values.push(max_key_width as u16);
        } else {
            values.push(max_key_width as u16 + self.theme.get_spacing() as u16);
            values.push(themed_separator.width() as u16);
        }

        values.push(self.theme.get_spacing() as u16);
        values.push(readout_data.width() as u16);

        if self.theme.get_padding() > 0 {
            values.insert(0, self.theme.get_padding() as u16)
        }

        values
    }
}

impl<'a> ReadoutList<'a> {
    fn create_layout(area: &Rect, constraints: &[u16]) -> Vec<Rect> {
        let mut layout: Vec<Rect> = Vec::with_capacity(constraints.len());
        layout.push(Rect {
            x: area.x,
            y: area.y,
            width: constraints[0],
            height: area.height,
        });

        for (i, &constraint) in constraints.iter().enumerate().skip(1) {
            let previous = layout[i - 1];
            layout.push(Rect {
                x: previous.x + previous.width,
                y: previous.y,
                width: constraint,
                height: area.height,
            });
        }

        layout
    }
}
