//! Host realization for the semantic `mncs-tui` framework.
//!
//! This crate owns only terminal effects: raw mode, alternate-screen lifecycle, input reads,
//! resize observation, and projection of a structured frame. Layout, widget state, and monitor
//! semantics remain upstream in MNCS/framework/application layers.

#[cfg(not(unix))]
use std::io;
#[cfg(not(unix))]
use std::time::Duration;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::TerminalSession;

#[cfg(not(unix))]
pub struct TerminalSession;

#[cfg(not(unix))]
impl TerminalSession {
    pub fn enter() -> io::Result<Self> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "mncs-tui-host currently requires Unix TTY semantics",
        ))
    }

    pub fn size(&self) -> Size {
        Size {
            columns: 80,
            rows: 24,
        }
    }
    pub fn draw(&mut self, _frame: &Frame) -> io::Result<()> {
        Ok(())
    }
    pub fn read_event(&mut self, _timeout: Duration) -> io::Result<Event> {
        Ok(Event::Tick)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    pub columns: u16,
    pub rows: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    pub column: u16,
    pub row: u16,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Style {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub bold: bool,
    pub dim: bool,
    pub underline: bool,
    pub reverse: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Indexed(u8),
    Rgb(u8, u8, u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell {
    pub glyph: char,
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            glyph: ' ',
            style: Style::default(),
        }
    }
}

/// A bounded, rectangular semantic frame. The host realizes it without knowing why a cell was
/// produced, and writes only changed cells after the first frame.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Frame {
    pub size: Size,
    cells: Vec<Cell>,
}

impl Frame {
    pub fn blank(size: Size) -> Self {
        let len = usize::from(size.columns).saturating_mul(usize::from(size.rows));
        Self {
            size,
            cells: vec![Cell::default(); len],
        }
    }

    pub fn get(&self, column: u16, row: u16) -> Option<Cell> {
        (column < self.size.columns && row < self.size.rows)
            .then(|| self.cells[self.index(column, row)])
    }

    pub fn set(&mut self, column: u16, row: u16, cell: Cell) {
        if column < self.size.columns && row < self.size.rows {
            let index = self.index(column, row);
            self.cells[index] = cell;
        }
    }

    pub fn write(&mut self, column: u16, row: u16, text: &str, style: Style) {
        for (offset, glyph) in text.chars().enumerate() {
            let Some(column) = column.checked_add(offset as u16) else {
                break;
            };
            if column >= self.size.columns {
                break;
            }
            self.set(column, row, Cell { glyph, style });
        }
    }

    pub fn fill_row(&mut self, row: u16, cell: Cell) {
        if row < self.size.rows {
            for column in 0..self.size.columns {
                self.set(column, row, cell);
            }
        }
    }

    pub fn cells(&self) -> impl Iterator<Item = (Point, Cell)> + '_ {
        self.cells.iter().enumerate().map(|(index, cell)| {
            let columns = usize::from(self.size.columns).max(1);
            let row = index / columns;
            let column = index % columns;
            (
                Point {
                    column: column as u16,
                    row: row as u16,
                },
                *cell,
            )
        })
    }

    fn index(&self, column: u16, row: u16) -> usize {
        usize::from(row) * usize::from(self.size.columns) + usize::from(column)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    Character(char),
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    Enter,
    Escape,
    Backspace,
    Tab,
    Delete,
    Ctrl(char),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    Key(Key),
    Resize(Size),
    Tick,
    Unknown,
}

pub(crate) fn sgr(style: Style, previous: Option<Style>) -> String {
    if previous == Some(style) {
        return String::new();
    }
    let mut codes = vec!["0".to_string()];
    if style.bold {
        codes.push("1".into());
    }
    if style.dim {
        codes.push("2".into());
    }
    if style.underline {
        codes.push("4".into());
    }
    if style.reverse {
        codes.push("7".into());
    }
    if let Some(color) = style.foreground {
        codes.push(color_code(color, false));
    }
    if let Some(color) = style.background {
        codes.push(color_code(color, true));
    }
    format!("\x1b[{}m", codes.join(";"))
}

fn color_code(color: Color, background: bool) -> String {
    let base = if background { 40 } else { 30 };
    match color {
        Color::Black => base.to_string(),
        Color::Red => (base + 1).to_string(),
        Color::Green => (base + 2).to_string(),
        Color::Yellow => (base + 3).to_string(),
        Color::Blue => (base + 4).to_string(),
        Color::Magenta => (base + 5).to_string(),
        Color::Cyan => (base + 6).to_string(),
        Color::White => (base + 7).to_string(),
        Color::BrightBlack => (base + 60).to_string(),
        Color::BrightRed => (base + 61).to_string(),
        Color::BrightGreen => (base + 62).to_string(),
        Color::BrightYellow => (base + 63).to_string(),
        Color::BrightBlue => (base + 64).to_string(),
        Color::BrightMagenta => (base + 65).to_string(),
        Color::BrightCyan => (base + 66).to_string(),
        Color::BrightWhite => (base + 67).to_string(),
        Color::Indexed(value) => format!("{}8;5;{value}", base),
        Color::Rgb(red, green, blue) => format!("{}8;2;{red};{green};{blue}", base),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_is_rectangular_and_clips_writes() {
        let size = Size {
            columns: 3,
            rows: 2,
        };
        let mut frame = Frame::blank(size);
        frame.write(2, 1, "abcd", Style::default());
        assert_eq!(frame.get(2, 1).unwrap().glyph, 'a');
        assert_eq!(frame.cells().count(), 6);
        assert!(frame.get(3, 1).is_none());
    }

    #[test]
    fn style_projection_is_stable() {
        let style = Style {
            bold: true,
            foreground: Some(Color::Cyan),
            ..Style::default()
        };
        assert_eq!(sgr(style, Some(style)), "");
        assert_eq!(sgr(style, None), "\x1b[0;1;36m");
    }
}
