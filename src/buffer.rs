use crate::styles::Style;
use crate::values::{Dimensions, Point, Rect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub content: char,
    pub style: Option<Style>,
}

impl Cell {
    /// An update that preserves the original background colour, unless the
    /// other style provides one.
    fn update(&mut self, content: char, other: &Option<Style>) {
        self.content = content;
        match (self.style, other) {
            (None, Some(_)) => self.style = other.clone(),
            (Some(mut existing), Some(update)) => existing.update(&update),
            _ => (),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    pub dimensions: Dimensions,
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
    pub fn new(dimensions: Dimensions) -> Self {
        let default = Cell {
            content: ' ',
            style: None,
        };
        let row = vec![default; dimensions.width];
        let cells = vec![row; dimensions.height];
        Buffer { dimensions, cells }
    }

    pub fn diff(&self, other: &Buffer) -> DiffResult {
        if (self.dimensions.width, self.dimensions.height)
            != (other.dimensions.width, other.dimensions.height)
        {
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
    pub fn merge(&mut self, at: &Point, other: &Buffer) {
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

    pub fn shrink(&mut self, from: Point, to: Point) {
        self.cells.truncate(to.y);
        self.cells.drain(0..from.y);

        for row in self.cells.iter_mut() {
            row.truncate(to.x);
            row.drain(0..from.x);
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
            self.mut_cell(x, at.y).update(c, style);
        }
    }

    pub fn draw_multiline_text(&mut self, within: &Rect, text: &str, style: &Option<Style>) {
        for (line_number, line) in text.lines().enumerate() {
            if line_number < within.dimensions.height {
                for (entry_number, entry) in line.chars().enumerate() {
                    if entry_number < within.dimensions.width {
                        self.mut_cell(
                            within.origin.x + entry_number,
                            within.origin.y + line_number,
                        )
                        .update(entry, style);
                    }
                }
            }
        }
    }

    pub fn draw_h_rule(&mut self, at: &Point, length: usize, style: &Option<Style>) {
        for x in at.x..(at.x + length) {
            self.mut_cell(x, at.y).update(H_LINE, style);
        }
    }

    pub fn mut_cell(&mut self, x: usize, y: usize) -> &mut Cell {
        if x > (self.dimensions.width - 1) || y > (self.dimensions.height - 1) {
            panic!("Provided point is outside of cells available in buffer")
        } else {
            &mut self.cells[y][x]
        }
    }

    pub fn draw_char(&mut self, x: usize, y: usize, content: char, style: &Option<Style>) {
        self.mut_cell(x, y).update(content, style);
    }

    pub fn draw_v_rule(&mut self, at: &Point, length: usize, style: &Option<Style>) {
        for y in at.y..(at.y + length) {
            self.mut_cell(at.x, y).update(V_LINE, style);
        }
    }

    pub fn merge_style(&mut self, within: &Rect, style: &Style) {
        for row in within.origin.y..(within.origin.y + within.dimensions.height) {
            for col in within.origin.x..(within.origin.x + within.dimensions.width) {
                let mut cell = self.mut_cell(col, row);
                if let Some(existing) = &mut cell.style {
                    existing.update(style)
                } else {
                    cell.style = Some(style.clone())
                }
            }
        }
    }

    pub fn draw_fill(&mut self, within: &Rect, style: Style, char: Option<char>) {
        let char = char.unwrap_or(' ');
        for row in within.origin.y..(within.origin.y + within.dimensions.height) {
            for col in within.origin.x..(within.origin.x + within.dimensions.width) {
                self.mut_cell(col, row).update(char, &Some(style));
            }
        }
    }

    pub fn draw_box(&mut self, rect: &Rect, rounded: bool, style: &Option<Style>) {
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
                self.mut_cell(x, rect.origin.y).update(top, style);
                self.mut_cell(x, y_inset).update(bottom, style);
                for y in (rect.origin.y + 1)..y_inset {
                    self.mut_cell(x, y).update(V_LINE, style);
                }
            } else if x == last_inset {
                let (top, bottom) = if rounded {
                    (ARC_DOWN_LEFT, ARC_UP_LEFT)
                } else {
                    (DOWN_LEFT, UP_LEFT)
                };
                self.mut_cell(x, rect.origin.y).update(top, style);
                self.draw_cell(Point { x: x, y: y_inset }, bottom, style.clone());
                self.mut_cell(x, y_inset).update(bottom, style);
                for y in (rect.origin.y + 1)..y_inset {
                    self.mut_cell(x, y).update(V_LINE, style);
                }
            } else {
                self.mut_cell(x, rect.origin.y).update(H_LINE, style);
                self.mut_cell(x, y_inset).update(H_LINE, style);
            }
        }
    }

    pub fn read_cell(&self, point: &Point) -> &Cell {
        &self.cells[point.y][point.x]
    }
}
