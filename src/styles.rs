pub use crossterm::style::Color;
use crossterm::style::{Attribute, ContentStyle};

pub struct FillStyle {
    pub repeating: Option<char>,
    pub style: Style,
}

impl FillStyle {
    pub fn new(repeating: Option<char>, style: Style) -> Self {
        Self { repeating, style }
    }
}

pub enum BorderStyle {
    Solid,
    SolidRounded,
    Dashed,
    Dotted,
    Double,
}

pub struct LineStyle {
    pub corners: Corners,
    pub stroke: Stroke,
    pub style: Option<Style>,
}

impl LineStyle {
    pub fn new(corners: Corners, stroke: Stroke, style: Option<Style>) -> Self {
        Self {
            corners,
            stroke,
            style,
        }
    }
}

impl Default for LineStyle {
    fn default() -> Self {
        Self {
            corners: Corners::Regular,
            stroke: Stroke::Solid,
            style: None,
        }
    }
}

pub enum Stroke {
    Solid,
    Dashed,
    Dotted,
    Double,
}

pub enum Corners {
    Regular,
    Rounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub(crate) style: ContentStyle,
}

impl Style {
    pub fn new() -> Self {
        Self {
            style: ContentStyle::new(),
        }
    }

    /// Merges the other style into this one, preserving attributes and only
    // adding or updating them.
    pub fn update(&mut self, other: &Style) {
        if let Some(color) = other.style.background_color {
            self.style.background(color);
        }

        if let Some(color) = other.style.foreground_color {
            self.style.foreground(color);
        }

        if !other.style.attributes.is_empty() {
            self.style.attributes.extend(other.style.attributes)
        }
    }

    pub fn background(mut self, color: Color) -> Self {
        self.style.background_color = Some(color);
        self
    }

    pub fn foreground(mut self, color: Color) -> Self {
        self.style.foreground_color = Some(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.style.attributes.set(Attribute::Bold);
        self
    }

    pub fn italic(mut self) -> Self {
        self.style.attributes.set(Attribute::Italic);
        self
    }

    pub fn underlined(mut self) -> Self {
        self.style.attributes.set(Attribute::Underlined);
        self
    }

    pub fn crossed_out(mut self) -> Self {
        self.style.attributes.set(Attribute::CrossedOut);
        self
    }

    pub fn reverse(mut self) -> Self {
        self.style.attributes.set(Attribute::Reverse);
        self
    }
}
