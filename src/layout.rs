use crate::buffer::Buffer;
use crate::solver::solve_auto;
use crate::styles::Style;
use crate::values::*;

/// Any element which is rendered to screen.
pub trait View {
    fn sizing(&self, bounds: &Dimensions) -> Dimensions;
    fn render(&self, within: &Rect, buffer: &mut Buffer);
}

/// A layout which positions it's children automatically based on their size
/// and the constraints provided.
pub struct Auto {
    dir: Dir,
    arrangement: Layout,
    items: Vec<AutoItem>,
}

impl Auto {
    pub fn new(dir: Dir, arrangement: Layout) -> Self {
        Auto {
            dir,
            arrangement,
            items: Vec::new(),
        }
    }

    pub fn add<V: 'static + View>(
        mut self,
        width: ContainerSizing,
        height: ContainerSizing,
        item: V,
    ) -> Self {
        self.items.push(AutoItem {
            width,
            height,
            item: Box::new(item),
        });
        self
    }

    pub fn rule(mut self, style: Option<Style>) -> Self {
        match self.dir {
            Dir::Horizontal => self.items.push(AutoItem {
                width: ContainerSizing::Fixed(1),
                height: ContainerSizing::Fill,
                item: Box::new(VRule::new(style)),
            }),
            Dir::Vertical => self.items.push(AutoItem {
                width: ContainerSizing::Fill,
                height: ContainerSizing::Fixed(1),
                item: Box::new(HRule::new(style)),
            }),
        }

        self
    }

    pub fn spacer(mut self) -> Self {
        self.items.push(AutoItem {
            width: ContainerSizing::Fill,
            height: ContainerSizing::Fill,
            item: Box::new(Spacer),
        });
        self
    }
}

impl View for Auto {
    fn sizing(&self, bounds: &Dimensions) -> Dimensions {
        let mut width = 0;
        let mut height = 0;

        for item in &self.items {
            let constraints = item.constraints(&bounds);

            let proposed_width = match constraints.h {
                Sizing::Fill => bounds.width,
                Sizing::Fixed(size) => size,
            };

            if proposed_width > width {
                width = proposed_width
            }

            let proposed_height = match constraints.v {
                Sizing::Fill => bounds.height,
                Sizing::Fixed(size) => size,
            };

            if proposed_height > height {
                height = proposed_height
            }
        }

        Dimensions { width, height }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        let items: Vec<Constraints> = self
            .items
            .iter()
            .map(|i| i.constraints(&within.dimensions))
            .collect();
        let layout = solve_auto(&items, &self.dir, &self.arrangement, &within);
        for (rect, item) in layout.iter().zip(&self.items) {
            item.render(&rect, buffer);
        }
    }
}

pub struct AutoItem {
    width: ContainerSizing,
    height: ContainerSizing,
    item: Box<dyn View>,
}

impl AutoItem {
    fn constraints(&self, bounds: &Dimensions) -> Constraints {
        let item_size = self.item.sizing(&bounds);

        let vertical = match self.height {
            ContainerSizing::Hug => Sizing::Fixed(item_size.height),
            ContainerSizing::Fill => Sizing::Fill,
            ContainerSizing::Fixed(size) => Sizing::Fixed(size),
        };

        let horizontal = match self.width {
            ContainerSizing::Hug => Sizing::Fixed(item_size.width),
            ContainerSizing::Fill => Sizing::Fill,
            ContainerSizing::Fixed(size) => Sizing::Fixed(size),
        };

        Constraints {
            v: vertical,
            h: horizontal,
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        self.item.render(within, buffer)
    }
}

pub struct Spacer;

impl View for Spacer {
    fn sizing(&self, _: &Dimensions) -> Dimensions {
        Dimensions {
            width: 1,
            height: 1,
        }
    }

    fn render(&self, _: &Rect, _: &mut Buffer) {}
}

// Styled text.
pub struct Text {
    pub value: String,
    pub style: Option<Style>,
}

impl Text {
    pub fn new(value: String, style: Option<Style>) -> Self {
        Self { value, style }
    }
}

impl View for Text {
    fn sizing(&self, bounds: &Dimensions) -> Dimensions {
        let len = self.value.len();
        Dimensions {
            width: if len > bounds.width {
                bounds.width
            } else {
                len
            },
            height: 1,
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        if self.value.len() > within.dimensions.width {
            let (prefix, _) = self.value.split_at(within.dimensions.width);
            buffer.draw_text(&within.origin, prefix, &self.style);
        } else {
            buffer.draw_text(&within.origin, &self.value, &self.style);
        }
    }
}

pub struct MultilineText {
    pub value: String,
    pub style: Option<Style>,
}

impl MultilineText {
    pub fn new(value: String, style: Option<Style>) -> Self {
        Self { value, style }
    }
}

impl View for MultilineText {
    fn sizing(&self, bounds: &Dimensions) -> Dimensions {
        Dimensions {
            width: self
                .value
                .lines()
                .max()
                .map(|l| l.chars().count())
                .unwrap_or(0)
                .clamp(0, bounds.width),
            height: self.value.lines().count().clamp(0, bounds.height),
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        buffer.draw_multiline_text(&within, &self.value, &self.style);
    }
}

// A vertical line that occupies the full height of it's container.
pub struct VRule {
    pub style: Option<Style>,
}

impl VRule {
    pub fn new(style: Option<Style>) -> Self {
        Self { style }
    }
}

impl View for VRule {
    fn sizing(&self, bounds: &Dimensions) -> Dimensions {
        Dimensions {
            width: 1,
            height: bounds.height,
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
    fn sizing(&self, bounds: &Dimensions) -> Dimensions {
        Dimensions {
            width: bounds.width,
            height: 1,
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        buffer.draw_h_rule(&within.origin, within.dimensions.width, &self.style)
    }
}
