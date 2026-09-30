use core::fmt;
use std::mem;

use vexide::color::Color;
use vexide::display::Text;
use vexide::display::{Font, FontFamily, FontSize};
use vexide::math::Point2;
use vexide::prelude::*;

pub struct ScreenModule {
    display: Display,
    position: Point2<i16>,
    color: Color,
    font: Font,
    line_height: i16,
    line_buffer: String,
    margin: i16,
}

impl ScreenModule {
    #[must_use]
    pub fn new(display: Display) -> Self {
        let font = Font::new(FontSize::SMALL, FontFamily::Monospace);
        let margin = 4;

        Self {
            display,
            position: Point2 {
                x: margin,
                y: margin,
            },
            color: Color::WHITE,
            font,
            line_height: Self::find_line_height(font),
            line_buffer: String::new(),
            margin,
        }
    }

    #[inline]
    /// Higher level printing method than is given by the vexide crate
    pub fn print(&mut self, text: impl Into<String>) {
        self.commit(text.into());
    }

    /// Print text at a specified coordinate. Used for runtime variables.
    /// Does NOT move the cursor.
    pub fn print_at(&mut self, position: impl Into<Point2<i16>>, text: impl Into<String>) {
        let widget = Text::from_string(text, self.font, position);
        self.display.draw_text(&widget, self.color, None);
    }

    #[inline]
    pub fn newline(&mut self) {
        self.advance();
    }

    pub fn set_font(&mut self, font: Font) -> &mut Self {
        self.font = font;
        self.line_height = Self::find_line_height(font);
        self
    }

    pub fn set_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.color = color.into();
        self
    }

    pub fn clear_screen(&mut self) {
        self.display.erase(Color::BLACK);
        self.position = Point2 {
            x: self.margin,
            y: self.margin,
        };
    }

    /// Draws any buffered text by the write! macro that hasn't been flushed to the VGA by a newline
    pub fn flush(&mut self) {
        if !self.line_buffer.is_empty() {
            let text = mem::take(&mut self.line_buffer);
            self.commit(text);
        }
    }

    /// Actually does the text rendering so the other functions can look pretty
    fn commit(&mut self, text: String) {
        let widget = Text::from_string(text, self.font, self.position);
        self.display.draw_text(&widget, self.color, None);
        self.advance();
    }

    fn advance(&mut self) {
        self.position.x = self.margin;
        self.position.y += self.line_height;

        // Wraps back to the top incase the text reaches the bottom edge.
        if self.position.y + self.line_height > Display::VERTICAL_RESOLUTION {
            self.position.y = self.margin;
        }
    }

    /// Measures the height of a specific font and gives 2 extra pixels of breathing room
    fn find_line_height(font: Font) -> i16 {
        let probe = Text::from_string("Ag", font, Point2 { x: 0, y: 0 });
        let height = probe.height().max(1) as i16;
        height + 2
    }
}

impl fmt::Write for ScreenModule {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for ch in s.chars() {
            if ch == '\n' {
                let text = mem::take(&mut self.line_buffer);
                self.commit(text);
            } else {
                self.line_buffer.push(ch);
            }
        }
        Ok(())
    }
}
