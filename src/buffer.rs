use crate::styles::Style;
use crate::values::{Point, Rect};
pub use crossterm::style::{Attribute, Color, ContentStyle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub content: char,
    pub style: Option<Style>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

pub enum DiffResult {
    NoChange,
    Invalid,
    Changed(Vec<(Point, Cell)>),
}

pub const H_LINE: char = '─';
pub const V_LINE: char = '│';
pub const DOWN_LEFT: char = '┐';
pub const DOWN_RIGHT: char = '┌';
pub const UP_RIGHT: char = '└';
pub const UP_LEFT: char = '┘';
pub const VERTICAL_RIGHT: char = '├';
pub const VERTICAL_LEFT: char = '┤';
pub const UP_HORIZONTAL: char = '┴';
pub const DOWN_HORIZONTAL: char = '┬';
pub const VERTICAL_HORIZONTAL: char = '┼';
pub const ARC_DOWN_RIGHT: char = '╭';
pub const ARC_DOWN_LEFT: char = '╮';
pub const ARC_UP_LEFT: char = '╯';
pub const ARC_UP_RIGHT: char = '╰';

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let default = Cell {
            content: ' ',
            style: None,
        };
        let row = vec![default; width];
        let cells = vec![row; height];
        Buffer {
            width,
            height,
            cells,
        }
    }

    pub fn diff(&self, other: &Buffer) -> DiffResult {
        if (self.width, self.height) != (other.width, other.height) {
            DiffResult::Invalid
        } else if self == other {
            DiffResult::NoChange
        } else {
            let mut results = vec![];
            for (y, row) in self.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    let update = &other.cells[y][x];
                    if cell != update {
                        results.push((Point { x, y }, update.clone()))
                    }
                }
            }

            DiffResult::Changed(results)
        }
    }

    /// A convenience for taking a sized buffer and merging it into another.
    /// The intention is to allow sub-layouts to be generated independently,
    /// then merged with a larger layout.
    pub fn merge(&mut self, at: Point, other: Buffer) {
        // TODO: This is a slow way of doing it. Just merging vectors might
        // be quicker
        for (y, cells) in other.cells.iter().enumerate() {
            for (x, cell) in cells.iter().enumerate() {
                self.draw_cell(
                    Point {
                        x: at.x + x,
                        y: at.y + y,
                    },
                    cell.content,
                    cell.style,
                );
            }
        }
    }

    // Draws a character at the specified point, with the optional styling.
    pub fn draw_cell(&mut self, at: Point, char: char, style: Option<Style>) {
        let cell = self.mut_cell(at.x, at.y);
        cell.content = char;
        cell.style = style;
    }

    pub fn draw_text(&mut self, at: &Point, text: &str, style: &Option<Style>) {
        let pairs = (at.x..(text.len() + at.x)).zip(text.chars());
        for (x, c) in pairs {
            self.draw_cell(Point { x, y: at.y }, c, style.clone())
        }
    }

    pub fn draw_multiline_text(&mut self, within: &Rect, text: &str, style: &Option<Style>) {
        for (line_number, line) in text.lines().enumerate() {
            if line_number < within.dimensions.height {
                for (entry_number, entry) in line.chars().enumerate() {
                    if entry_number < within.dimensions.width {
                        self.draw_cell(
                            Point {
                                x: within.origin.x + entry_number,
                                y: within.origin.y + line_number,
                            },
                            entry,
                            style.clone(),
                        )
                    }
                }
            }
        }
    }

    pub fn draw_h_rule(&mut self, at: &Point, length: usize, style: &Option<Style>) {
        for x in at.x..(at.x + length) {
            self.draw_cell(Point { x, y: at.y }, H_LINE, style.clone())
        }
    }

    pub fn mut_cell(&mut self, x: usize, y: usize) -> &mut Cell {
        if x > (self.width - 1) || y > (self.height - 1) {
            panic!("Provided point is outside of cells available in buffer")
        } else {
            &mut self.cells[y][x]
        }
    }

    pub fn draw_v_rule(&mut self, at: &Point, length: usize, style: &Option<Style>) {
        for y in at.y..(at.y + length) {
            self.draw_cell(Point { x: at.x, y }, V_LINE, style.clone())
        }
    }

    pub fn draw_box(&mut self, rect: Rect, rounded: bool, style: Option<Style>) {
        let last = rect.origin.x + rect.dimensions.width;
        let last_inset = last - 1;
        let bottom_y = rect.origin.y + rect.dimensions.height;
        let y_inset = bottom_y - 1;

        for x in rect.origin.x..last {
            if x == rect.origin.x {
                let (top, bottom) = if rounded {
                    (ARC_DOWN_RIGHT, ARC_UP_RIGHT)
                } else {
                    (DOWN_RIGHT, UP_RIGHT)
                };
                self.draw_cell(
                    Point {
                        x: x,
                        y: rect.origin.y,
                    },
                    top,
                    style.clone(),
                );
                self.draw_cell(Point { x: x, y: y_inset }, bottom, style.clone());
                for y in (rect.origin.y + 1)..y_inset {
                    self.draw_cell(Point { x: x, y: y }, V_LINE, style.clone());
                }
            } else if x == last_inset {
                let (top, bottom) = if rounded {
                    (ARC_DOWN_LEFT, ARC_UP_LEFT)
                } else {
                    (DOWN_LEFT, UP_LEFT)
                };
                self.draw_cell(
                    Point {
                        x: x,
                        y: rect.origin.y,
                    },
                    top,
                    style.clone(),
                );
                self.draw_cell(Point { x: x, y: y_inset }, bottom, style.clone());
                for y in (rect.origin.y + 1)..y_inset {
                    self.draw_cell(Point { x: x, y: y }, V_LINE, style.clone());
                }
            } else {
                self.draw_cell(
                    Point {
                        x: x,
                        y: rect.origin.y,
                    },
                    H_LINE,
                    style.clone(),
                );
                self.draw_cell(Point { x: x, y: y_inset }, H_LINE, style.clone());
            }
        }
    }

    pub fn read_cell(&self, point: &Point) -> &Cell {
        &self.cells[point.y][point.x]
    }
}
