use crate::buffer::Buffer;
use crate::styles::Style;
use crate::values::{Constraints, ContainerSizing, Dimensions, Rect, Sizing};

pub trait View {
    fn sizing(&self, bounds: &Dimensions) -> Constraints;
    fn render(&self, within: &Rect, buffer: &mut Buffer);
}

pub struct Spacer;

impl View for Spacer {
    fn sizing(&self, _: &Dimensions) -> Constraints {
        Constraints {
            width: Sizing::Fill,
            height: Sizing::Fill,
        }
    }

    fn render(&self, _: &Rect, _: &mut Buffer) {}
}

pub struct Label {
    pub text: String,
    pub width: ContainerSizing,
    pub height: ContainerSizing,
    pub style: Option<Style>,
}

impl Label {
    pub fn new(
        text: String,
        width: ContainerSizing,
        height: ContainerSizing,
        style: Option<Style>,
    ) -> Self {
        Self {
            text,
            width,
            height,
            style,
        }
    }
}

impl View for Label {
    fn sizing(&self, bounds: &Dimensions) -> Constraints {
        Constraints {
            width: self.width.simplify(self.text.len().clamp(0, bounds.width)),
            height: self.height.simplify(1),
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        if self.text.len() > within.dimensions.width {
            let (prefix, _) = self.text.split_at(within.dimensions.width);
            buffer.draw_text(&within.origin, prefix, &self.style);
        } else {
            buffer.draw_text(&within.origin, &self.text, &self.style);
        }
    }
}

pub struct MultilineText {
    pub text: String,
    pub width: ContainerSizing,
    pub height: ContainerSizing,
    pub style: Option<Style>,
}

impl MultilineText {
    pub fn new(
        text: String,
        width: ContainerSizing,
        height: ContainerSizing,
        style: Option<Style>,
    ) -> Self {
        Self {
            text,
            width,
            height,
            style,
        }
    }
}

impl View for MultilineText {
    fn sizing(&self, bounds: &Dimensions) -> Constraints {
        Constraints {
            width: self.width.simplify(
                self.text
                    .lines()
                    .max()
                    .map(|l| l.chars().count())
                    .unwrap_or(0)
                    .clamp(0, bounds.width),
            ),
            height: self
                .height
                .simplify(self.text.lines().count().clamp(0, bounds.height)),
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        buffer.draw_multiline_text(&within, &self.text, &self.style);
    }
}

// // A vertical line that occupies the full height of it's container.
pub struct VRule {
    pub style: Option<Style>,
}

impl VRule {
    pub fn new(style: Option<Style>) -> Self {
        Self { style }
    }
}

impl View for VRule {
    fn sizing(&self, _: &Dimensions) -> Constraints {
        Constraints {
            height: Sizing::Fill,
            width: Sizing::Fixed(1),
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        buffer.draw_v_rule(&within.origin, within.dimensions.height, &self.style)
    }
}

pub struct HRule {
    pub style: Option<Style>,
}

impl HRule {
    pub fn new(style: Option<Style>) -> Self {
        Self { style }
    }
}

impl View for HRule {
    fn sizing(&self, _: &Dimensions) -> Constraints {
        Constraints {
            height: Sizing::Fixed(1),
            width: Sizing::Fill,
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        buffer.draw_h_rule(&within.origin, within.dimensions.width, &self.style)
    }
}