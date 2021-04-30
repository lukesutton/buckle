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
pub enum Constraint {
    Fill,
    Fixed(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Beginning,
    Center,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arrangement {
    Packed {
        vertical: Alignment,
        horizontal: Alignment,
    },
    FixedSpacing {
        vertical: Alignment,
        horizontal: Alignment,
        spacing: usize,
    },
    EvenSpacing {
        vertical: Alignment,
        horizontal: Alignment,
    },
}

impl Default for Arrangement {
    fn default() -> Self {
        Self::Packed {
            vertical: Alignment::Beginning,
            horizontal: Alignment::Beginning,
        }
    }
}

impl Arrangement {
    pub fn packed(vertical: Alignment, horizontal: Alignment) -> Self {
        Self::Packed {
            vertical,
            horizontal,
        }
    }

    pub fn fixed(vertical: Alignment, horizontal: Alignment, spacing: usize) -> Self {
        Self::FixedSpacing {
            vertical,
            horizontal,
            spacing,
        }
    }

    pub fn even(vertical: Alignment, horizontal: Alignment) -> Self {
        Self::EvenSpacing {
            vertical,
            horizontal,
        }
    }

    pub fn vertical(&self) -> &Alignment {
        match self {
            Arrangement::Packed {
                vertical,
                horizontal: _,
            } => vertical,
            Arrangement::FixedSpacing {
                vertical,
                horizontal: _,
                spacing: _,
            } => vertical,
            Arrangement::EvenSpacing {
                vertical,
                horizontal: _,
            } => vertical,
        }
    }

    pub fn horizontal(&self) -> &Alignment {
        match self {
            Arrangement::Packed {
                vertical: _,
                horizontal,
            } => horizontal,
            Arrangement::FixedSpacing {
                vertical: _,
                horizontal,
                spacing: _,
            } => horizontal,
            Arrangement::EvenSpacing {
                vertical: _,
                horizontal,
            } => horizontal,
        }
    }

    pub fn rotate(&mut self) {
        *self = match self {
            Self::Packed {
                vertical,
                horizontal,
            } => Self::Packed {
                vertical: *horizontal,
                horizontal: *vertical,
            },

            Self::FixedSpacing {
                vertical,
                horizontal,
                spacing,
            } => Self::FixedSpacing {
                vertical: *horizontal,
                horizontal: *vertical,
                spacing: *spacing,
            },
            Self::EvenSpacing {
                vertical,
                horizontal,
            } => Self::EvenSpacing {
                vertical: *horizontal,
                horizontal: *vertical,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraints {
    pub horizontal: Constraint,
    pub vertical: Constraint,
}

impl Constraints {
    pub fn rotate(&mut self) {
        let h = self.horizontal;
        let v = self.vertical;
        self.horizontal = v;
        self.vertical = h;
    }

    pub fn new(horizontal: Constraint, vertical: Constraint) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}
