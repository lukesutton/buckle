use crate::buffer::{Buffer, DiffResult};
use crate::values::Dimensions;
use crossterm::{cursor, execute, queue, style, terminal};
use std::io::{stdout, Stdout, Write};

pub struct Terminal {
    stdout: Stdout,
    buffer: Buffer,
}

impl Terminal {
    pub fn new() -> Terminal {
        let mut out = stdout();
        execute!(out, terminal::EnterAlternateScreen).unwrap();
        let (cols, rows) = terminal::size().unwrap();
        let buffer = Buffer::new(Dimensions::new(cols as usize, rows as usize));

        Terminal {
            stdout: out,
            buffer: buffer,
        }
    }

    pub fn resize(&mut self) -> bool {
        let (cols, rows) = terminal::size().unwrap();
        if cols != self.buffer.dimensions.width as u16
            || rows != self.buffer.dimensions.height as u16
        {
            self.buffer = Buffer::new(Dimensions::new(cols as usize, rows as usize));
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    pub fn prepare_buffer(&self) -> Buffer {
        let (cols, rows) = terminal::size().unwrap();
        Buffer::new(Dimensions::new(cols as usize, rows as usize))
    }

    pub fn update(&mut self, updates: Buffer) {
        // TODO: Handle the results from queue and flush.
        match self.buffer.diff(&updates) {
            DiffResult::NoChange => (),
            DiffResult::Invalid => (), // Log this as a problem
            DiffResult::Changed(changes) => {
                for (point, cell) in changes {
                    let content = if let Some(style) = cell.style {
                        style::StyledContent::new(style.style, cell.content)
                    } else {
                        style::style(cell.content)
                    };

                    queue!(
                        self.stdout,
                        cursor::MoveTo(point.x as u16, point.y as u16),
                        style::PrintStyledContent(content)
                    )
                    .unwrap();
                }
                self.stdout.flush().unwrap();
                self.buffer = updates;
            }
        }
    }
}

pub enum TerminalEvent {}
