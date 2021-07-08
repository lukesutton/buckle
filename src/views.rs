use crate::buffer::Buffer;
use crate::styles::{Stroke, Style};
use crate::values::{Constraints, ContainerSizing, Dimensions, Dir, Point, Rect, Sizing};
use crossterm::style::Color;

pub trait View: 'static {
    fn sizing(&self, bounds: &Dimensions) -> Constraints;
    fn render(&self, within: &Rect, buffer: &mut Buffer);
}

pub struct Spacer;

impl Spacer {
    pub fn new() -> Self {
        Self {}
    }
}

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
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            width: ContainerSizing::Hug,
            height: ContainerSizing::Hug,
        }
    }

    pub fn width(mut self, width: ContainerSizing) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: ContainerSizing) -> Self {
        self.height = height;
        self
    }
}

impl View for Label {
    fn sizing(&self, bounds: &Dimensions) -> Constraints {
        Constraints {
            width: self
                .width
                .simplify(self.text.chars().count().clamp(0, bounds.width)),
            height: self.height.simplify(1),
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        if self.text.chars().count() > within.dimensions.width {
            let mut prefix = self.text.clone();
            prefix.truncate(within.dimensions.width);
            buffer.draw_text(&within.origin, &prefix);
        } else {
            buffer.draw_text(&within.origin, &self.text);
        }
    }
}

pub struct MultilineText {
    pub text: String,
    pub width: ContainerSizing,
    pub height: ContainerSizing,
}

impl MultilineText {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            width: ContainerSizing::Hug,
            height: ContainerSizing::Hug,
        }
    }

    pub fn width(mut self, width: ContainerSizing) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: ContainerSizing) -> Self {
        self.height = height;
        self
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
        buffer.draw_multiline_text(&within, &self.text);
    }
}

pub struct Rule {
    dir: Dir,
}

impl Rule {
    pub fn new(dir: Dir) -> Self {
        Self { dir }
    }
}

impl View for Rule {
    fn sizing(&self, _: &Dimensions) -> Constraints {
        match self.dir {
            Dir::Horizontal => Constraints {
                height: Sizing::Fixed(1),
                width: Sizing::Fill,
            },
            Dir::Vertical => Constraints {
                height: Sizing::Fill,
                width: Sizing::Fixed(1),
            },
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        match self.dir {
            Dir::Horizontal => buffer.draw_h_rule(&within.origin, within.dimensions.width),
            Dir::Vertical => buffer.draw_v_rule(&within.origin, within.dimensions.height),
        }
    }
}

pub struct Padding {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    item: Box<dyn View>,
}

impl Padding {
    pub fn new<V: View>(left: usize, right: usize, top: usize, bottom: usize, item: V) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            item: Box::new(item),
        }
    }

    pub fn all<V: View>(value: usize, item: V) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
            item: Box::new(item),
        }
    }

    pub fn vertical<V: View>(value: usize, item: V) -> Self {
        Self {
            top: value,
            bottom: value,
            left: 0,
            right: 0,
            item: Box::new(item),
        }
    }

    pub fn horizontal<V: View>(value: usize, item: V) -> Self {
        Self {
            top: 0,
            bottom: 0,
            left: value,
            right: value,
            item: Box::new(item),
        }
    }

    pub fn both<V: View>(h: usize, v: usize, item: V) -> Self {
        Self {
            top: v,
            bottom: v,
            left: h,
            right: h,
            item: Box::new(item),
        }
    }
}

impl View for Padding {
    fn sizing(&self, bounds: &Dimensions) -> Constraints {
        let remaining = Dimensions::new(
            bounds.width - self.left - self.right,
            bounds.height - self.top - self.bottom,
        );
        let constraints = self.item.sizing(&remaining);
        Constraints::new(
            match constraints.width {
                Sizing::Fill => Sizing::Fill,
                Sizing::Fixed(n) => Sizing::Fixed(n + self.left + self.right),
            },
            match constraints.height {
                Sizing::Fill => Sizing::Fill,
                Sizing::Fixed(n) => Sizing::Fixed(n + self.top + self.bottom),
            },
        )
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        // Offset the rect and reduce it's size
        let offset = Rect::new(
            Point::new(within.origin.x + self.left, within.origin.y + self.top),
            Dimensions::new(
                within.dimensions.width - self.left - self.right,
                within.dimensions.height - self.top - self.bottom,
            ),
        );

        self.item.render(&offset, buffer);
    }
}

pub struct Styled {
    style: Style,
    item: Box<dyn View>,
}

impl Styled {
    pub fn new<V: View>(item: V) -> Self {
        Self {
            style: Style::new(),
            item: Box::new(item),
        }
    }

    pub fn background(mut self, color: Color) -> Self {
        self.style = self.style.background(color);
        self
    }

    pub fn foreground(mut self, color: Color) -> Self {
        self.style = self.style.foreground(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.style = self.style.bold();
        self
    }

    pub fn italic(mut self) -> Self {
        self.style = self.style.italic();
        self
    }

    pub fn underlined(mut self) -> Self {
        self.style = self.style.underlined();
        self
    }

    pub fn crossed_out(mut self) -> Self {
        self.style = self.style.crossed_out();
        self
    }

    pub fn reverse(mut self) -> Self {
        self.style = self.style.reverse();
        self
    }
}

impl View for Styled {
    fn sizing(&self, bounds: &Dimensions) -> Constraints {
        self.item.sizing(&bounds)
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        buffer.merge_style(within, &self.style);
        self.item.render(within, buffer)
    }
}

pub struct Border {
    stroke: Stroke,
    style: Style,
    item: Box<dyn View>,
}

impl Border {
    pub fn new<V: View>(stroke: Stroke, item: V) -> Self {
        Self {
            stroke,
            style: Style::new(),
            item: Box::new(item),
        }
    }

    pub fn foreground(mut self, color: Color) -> Self {
        self.style = self.style.foreground(color);
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.style = self.style.background(color);
        self
    }
}

impl View for Border {
    fn sizing(&self, bounds: &Dimensions) -> Constraints {
        let sizing = self.item.sizing(&bounds);
        Constraints::new(
            match sizing.width {
                Sizing::Fill => Sizing::Fill,
                Sizing::Fixed(n) => Sizing::Fixed((n + 2).clamp(0, bounds.width)),
            },
            match sizing.height {
                Sizing::Fill => Sizing::Fill,
                Sizing::Fixed(n) => Sizing::Fixed((n + 2).clamp(0, bounds.height)),
            },
        )
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        let mut within = within.clone();
        buffer.draw_box(&within, &self.stroke, &Some(self.style));
        within.origin.x += 1;
        within.origin.y += 1;
        within.dimensions.width -= 2;
        within.dimensions.height -= 2;
        self.item.render(&within, buffer);
    }
}

/// A view where all the rendering is handled by the provided function.
pub struct Draw {
    width: Sizing,
    height: Sizing,
    renderer: Box<dyn Fn(&Rect, &mut Buffer)>,
}

impl Draw {
    pub fn new<R: 'static + Fn(&Rect, &mut Buffer)>(
        width: Sizing,
        height: Sizing,
        renderer: R,
    ) -> Self {
        Self {
            width,
            height,
            renderer: Box::new(renderer),
        }
    }
}

impl View for Draw {
    fn sizing(&self, bounds: &Dimensions) -> Constraints {
        Constraints::new(
            match self.width {
                Sizing::Fill => Sizing::Fill,
                Sizing::Fixed(n) => Sizing::Fixed(n.clamp(0, bounds.width)),
            },
            match self.height {
                Sizing::Fill => Sizing::Fill,
                Sizing::Fixed(n) => Sizing::Fixed(n.clamp(0, bounds.height)),
            },
        )
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        (self.renderer)(within, buffer);
    }
}
