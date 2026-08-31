use std::io::{self, Write};
use std::os::fd::RawFd;
use std::time::Duration;

use libc::{c_int, pollfd, termios, POLLIN, STDIN_FILENO, STDOUT_FILENO, TCSANOW, TIOCGWINSZ};

use crate::{sgr, Event, Frame, Key, Size, Style};

const ESC: u8 = 0x1b;

pub struct TerminalSession {
    input: RawFd,
    output: RawFd,
    original: termios,
    size: Size,
    previous: Option<Frame>,
    active: bool,
}

impl TerminalSession {
    pub fn enter() -> io::Result<Self> {
        let input = STDIN_FILENO;
        let output = STDOUT_FILENO;
        if !is_tty(input) || !is_tty(output) {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "stdin/stdout are not TTYs",
            ));
        }
        let original = read_termios(input)?;
        let mut raw = original;
        raw.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
        raw.c_oflag &= !libc::OPOST;
        raw.c_cflag |= libc::CS8;
        raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
        raw.c_cc[libc::VMIN] = 0;
        raw.c_cc[libc::VTIME] = 0;
        set_termios(input, &raw)?;
        let session = Self {
            input,
            output,
            original,
            size: terminal_size(output)?,
            previous: None,
            active: true,
        };
        session.write_all("\x1b[?1049h\x1b[?25l\x1b[2J\x1b[H")?;
        Ok(session)
    }

    pub fn size(&self) -> Size {
        self.size
    }

    /// Draws a structured frame and retains the frame only after all writes succeed. Resizing
    /// forces a clear/full redraw because old coordinates no longer describe the terminal.
    pub fn draw(&mut self, frame: &Frame) -> io::Result<()> {
        let resized = frame.size != self.size;
        let old = if resized {
            None
        } else {
            self.previous.as_ref()
        };
        let mut output = String::new();
        let mut previous_style: Option<Style> = None;
        if resized {
            output.push_str("\x1b[2J\x1b[H");
        }
        for (point, cell) in frame.cells() {
            let changed = old.and_then(|frame| frame.get(point.column, point.row)) != Some(cell);
            if !changed {
                continue;
            }
            output.push_str(&format!("\x1b[{};{}H", point.row + 1, point.column + 1));
            output.push_str(&sgr(cell.style, previous_style));
            output.push(cell.glyph);
            previous_style = Some(cell.style);
        }
        if !output.is_empty() {
            self.write_all(&output)?;
        }
        self.previous = Some(frame.clone());
        self.size = frame.size;
        Ok(())
    }

    pub fn read_event(&mut self, timeout: Duration) -> io::Result<Event> {
        let current_size = terminal_size(self.output)?;
        if current_size != self.size {
            self.size = current_size;
            return Ok(Event::Resize(current_size));
        }
        let milliseconds = timeout.as_millis().min(i32::MAX as u128) as c_int;
        let mut descriptor = pollfd {
            fd: self.input,
            events: POLLIN,
            revents: 0,
        };
        // SAFETY: descriptor points to one initialized pollfd and the timeout is bounded.
        let ready = unsafe { libc::poll(&mut descriptor, 1, milliseconds) };
        if ready < 0 {
            return Err(io::Error::last_os_error());
        }
        if ready == 0 {
            return Ok(Event::Tick);
        }
        let mut bytes = [0_u8; 16];
        // SAFETY: bytes is a valid writable buffer and input is a live TTY fd owned by the process.
        let count = unsafe { libc::read(self.input, bytes.as_mut_ptr().cast(), bytes.len()) };
        if count < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(parse_input(&bytes[..count as usize]))
    }

    fn write_all(&self, value: &str) -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        stdout.write_all(value.as_bytes())?;
        stdout.flush()
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let _ = self.write_all("\x1b[0m\x1b[?25h\x1b[?1049l");
        let _ = set_termios(self.input, &self.original);
        self.active = false;
    }
}

fn is_tty(fd: RawFd) -> bool {
    // SAFETY: isatty only observes the validity of the process-owned descriptor.
    unsafe { libc::isatty(fd) == 1 }
}

fn read_termios(fd: RawFd) -> io::Result<termios> {
    let mut value = std::mem::MaybeUninit::uninit();
    // SAFETY: tcgetattr initializes the supplied termios for a valid TTY.
    let result = unsafe { libc::tcgetattr(fd, value.as_mut_ptr()) };
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(unsafe { value.assume_init() })
    }
}

fn set_termios(fd: RawFd, value: &termios) -> io::Result<()> {
    // SAFETY: value points to a valid termios and fd is the TTY captured by this session.
    let result = unsafe { libc::tcsetattr(fd, TCSANOW, value) };
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn terminal_size(fd: RawFd) -> io::Result<Size> {
    let mut window = std::mem::MaybeUninit::<libc::winsize>::zeroed();
    // SAFETY: ioctl writes a winsize into the supplied buffer for a TTY descriptor.
    let result = unsafe { libc::ioctl(fd, TIOCGWINSZ, window.as_mut_ptr()) };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }
    let window = unsafe { window.assume_init() };
    Ok(Size {
        columns: window.ws_col.max(1),
        rows: window.ws_row.max(1),
    })
}

fn parse_input(bytes: &[u8]) -> Event {
    match bytes {
        [3] => Event::Key(Key::Ctrl('c')),
        [ESC, b'[', b'A'] => Event::Key(Key::Up),
        [ESC, b'[', b'B'] => Event::Key(Key::Down),
        [ESC, b'[', b'C'] => Event::Key(Key::Right),
        [ESC, b'[', b'D'] => Event::Key(Key::Left),
        [ESC, b'[', b'5', b'~'] => Event::Key(Key::PageUp),
        [ESC, b'[', b'6', b'~'] => Event::Key(Key::PageDown),
        [ESC, b'[', b'H'] | [ESC, b'[', b'1', b'~'] => Event::Key(Key::Home),
        [ESC, b'[', b'F'] | [ESC, b'[', b'4', b'~'] => Event::Key(Key::End),
        [ESC] => Event::Key(Key::Escape),
        [b'\r'] | [b'\n'] => Event::Key(Key::Enter),
        [b'\t'] => Event::Key(Key::Tab),
        [0x7f] | [8] => Event::Key(Key::Backspace),
        [byte] if *byte < 0x80 => Event::Key(Key::Character(*byte as char)),
        _ => Event::Unknown,
    }
}
