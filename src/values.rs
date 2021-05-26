#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn rotate(&mut self) {
        let x = self.x;
        let y = self.y;
        self.x = y;
        self.y = x;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Dimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn zero() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    pub fn rotate(&mut self) {
        let width = self.width;
        let height = self.height;
        self.width = height;
        self.height = width;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rect {
    pub origin: Point,
    pub dimensions: Dimensions,
}

impl Rect {
    pub fn new(origin: Point, dimensions: Dimensions) -> Self {
        Self { origin, dimensions }
    }

    pub fn new_from_raw(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            origin: Point::new(x, y),
            dimensions: Dimensions::new(width, height),
        }
    }

    pub fn rotate(&mut self) {
        self.origin.rotate();
        self.dimensions.rotate();
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sizing {
    Fill,
    Fixed(usize),
}

#[derive(Debug)]
pub enum ContainerSizing {
    Hug,
    Fill,
    Fixed(usize),
}

impl ContainerSizing {
    pub fn simplify(&self, hug_value: usize) -> Sizing {
        match self {
            ContainerSizing::Hug => Sizing::Fixed(hug_value),
            ContainerSizing::Fill => Sizing::Fill,
            ContainerSizing::Fixed(size) => Sizing::Fixed(*size),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dir {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layout {
    Packed { v: Align, h: Align },
    Spaced { v: Align, h: Align, spacing: usize },
    Spread { v: Align, h: Align },
}

impl Default for Layout {
    fn default() -> Self {
        Self::Packed {
            v: Align::Start,
            h: Align::Start,
        }
    }
}

impl Layout {
    pub fn packed(vertical: Align, horizontal: Align) -> Self {
        Self::Packed {
            v: vertical,
            h: horizontal,
        }
    }

    pub fn fixed(vertical: Align, horizontal: Align, spacing: usize) -> Self {
        Self::Spaced {
            v: vertical,
            h: horizontal,
            spacing,
        }
    }

    pub fn even(vertical: Align, horizontal: Align) -> Self {
        Self::Spread {
            v: vertical,
            h: horizontal,
        }
    }

    pub fn vertical(&self) -> &Align {
        match self {
            Layout::Packed { v: vertical, h: _ } => vertical,
            Layout::Spaced {
                v: vertical,
                h: _,
                spacing: _,
            } => vertical,
            Layout::Spread { v: vertical, h: _ } => vertical,
        }
    }

    pub fn horizontal(&self) -> &Align {
        match self {
            Layout::Packed {
                v: _,
                h: horizontal,
            } => horizontal,
            Layout::Spaced {
                v: _,
                h: horizontal,
                spacing: _,
            } => horizontal,
            Layout::Spread {
                v: _,
                h: horizontal,
            } => horizontal,
        }
    }

    pub fn rotate(&mut self) {
        *self = match self {
            Self::Packed {
                v: vertical,
                h: horizontal,
            } => Self::Packed {
                v: *horizontal,
                h: *vertical,
            },

            Self::Spaced {
                v: vertical,
                h: horizontal,
                spacing,
            } => Self::Spaced {
                v: *horizontal,
                h: *vertical,
                spacing: *spacing,
            },
            Self::Spread {
                v: vertical,
                h: horizontal,
            } => Self::Spread {
                v: *horizontal,
                h: *vertical,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraints {
    pub width: Sizing,
    pub height: Sizing,
}

impl Constraints {
    pub fn rotate(&mut self) {
        let width = self.width;
        let height = self.height;
        self.width = height;
        self.height = width;
    }

    pub fn new(width: Sizing, height: Sizing) -> Self {
        Self { width, height }
    }
}
