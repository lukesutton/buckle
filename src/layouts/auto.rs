use crate::buffer::{Buffer, DOWN_HORIZONTAL, UP_HORIZONTAL, VERTICAL_LEFT, VERTICAL_RIGHT};
use crate::layouts::auto_solver::solve;
use crate::styles::{FillStyle, LineStyle, Style};
use crate::values::*;
use crate::views::{HRule, Spacer, VRule, View};

/// A layout which positions it's children automatically based on their size
/// and the constraints provided.

/*
Invariants that should hold:

- Child elements that fall entirely within the layouts bounds will be entirely rendered
- Elements that start within the layout, but end without, will be truncated
- Elements which start outside of the layout, will be clipped
*/
pub struct Auto {
    dir: Dir,
    layout: Layout,
    width: ContainerSizing,
    height: ContainerSizing,
    border_style: Option<LineStyle>,
    fill_style: Option<FillStyle>,
    items: Vec<Box<dyn View>>,
    splits: Vec<usize>,
}

impl Auto {
    pub fn new(dir: Dir, layout: Layout, width: ContainerSizing, height: ContainerSizing) -> Self {
        Auto {
            dir,
            layout,
            width,
            height,
            border_style: None,
            fill_style: None,
            items: Vec::new(),
            splits: Vec::new(),
        }
    }

    pub fn vertical() -> Self {
        Self::new(
            Dir::Vertical,
            Layout::default(),
            ContainerSizing::Fill,
            ContainerSizing::Fill,
        )
    }

    pub fn horizontal() -> Self {
        Self::new(
            Dir::Horizontal,
            Layout::default(),
            ContainerSizing::Fill,
            ContainerSizing::Fill,
        )
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn width(mut self, width: ContainerSizing) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: ContainerSizing) -> Self {
        self.height = height;
        self
    }

    pub fn borders(mut self, borders: LineStyle) -> Self {
        self.border_style = Some(borders);
        self
    }

    pub fn fill(mut self, fill: FillStyle) -> Self {
        self.fill_style = Some(fill);
        self
    }

    pub fn add<V: 'static + View>(mut self, item: V) -> Self {
        self.items.push(Box::new(item));
        self
    }

    pub fn add_each<I, F, V>(mut self, items: I, render: F) -> Self
    where
        I: IntoIterator,
        V: 'static + View,
        F: Fn(&I::Item) -> V,
    {
        for item in items {
            self.items.push(Box::new(render(&item)));
        }
        self
    }

    pub fn maybe_add<V: 'static + View>(mut self, check: bool, item: V) -> Self {
        if check {
            self.items.push(Box::new(item));
        }
        self
    }

    pub fn rule(mut self, style: Option<Style>) -> Self {
        match self.dir {
            Dir::Horizontal => self.items.push(Box::new(VRule::new(style))),
            Dir::Vertical => self.items.push(Box::new(HRule::new(style))),
        }

        self
    }

    pub fn split(mut self) -> Self {
        let split = match self.dir {
            Dir::Horizontal => Split {
                width: Sizing::Fixed(1),
                height: Sizing::Fill,
            },
            Dir::Vertical => Split {
                width: Sizing::Fill,
                height: Sizing::Fixed(1),
            },
        };
        self.splits.push(self.items.len());
        self.items.push(Box::new(split));

        self
    }

    pub fn spacer(mut self) -> Self {
        self.items.push(Box::new(Spacer {}));
        self
    }
}

impl View for Auto {
    fn sizing(&self, bounds: &Dimensions) -> Constraints {
        // TODO:
        // Look at optimising this by avoiding sizing calculations when the
        // layout has a fixed or fill sizing in a particular axis.

        let mut height = 0;
        let mut width = 0;

        match self.dir {
            Dir::Horizontal => {
                for item in &self.items {
                    let constraints = item.sizing(&bounds);

                    if let Sizing::Fixed(n) = constraints.width {
                        width = (width + n).clamp(0, bounds.width)
                    }

                    if let Sizing::Fixed(n) = constraints.height {
                        if n > height {
                            height = n.clamp(0, bounds.height)
                        }
                    }
                }
            }
            Dir::Vertical => {
                for item in &self.items {
                    let constraints = item.sizing(&bounds);

                    if let Sizing::Fixed(n) = constraints.height {
                        height = (height + n).clamp(0, bounds.height);
                    }

                    if let Sizing::Fixed(n) = constraints.width {
                        if n > width {
                            width = n.clamp(0, bounds.width)
                        }
                    }
                }
            }
        }

        Constraints {
            width: self.width.simplify(width),
            height: self.height.simplify(height),
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        if let Some(fill) = &self.fill_style {
            buffer.draw_fill(&within, fill.style, fill.repeating);
        }
        let mut within = within.clone();
        if let Some(borders) = &self.border_style {
            buffer.draw_box(&within, false, &borders.style);
            within.origin.x += 1;
            within.origin.y += 1;
            within.dimensions.width -= 2;
            within.dimensions.height -= 2;
        }

        let items: Vec<Constraints> = self
            .items
            .iter()
            .map(|i| i.sizing(&within.dimensions))
            .collect();
        let layout = solve(&items, &self.dir, &self.layout, &within);
        for (rect, item) in layout.iter().zip(&self.items) {
            if rect.origin.x < within.origin.x + within.dimensions.width
                && rect.origin.y < within.origin.y + within.dimensions.height
            {
                item.render(&rect, buffer);
            }
        }

        if let Some(borders) = &self.border_style {
            for i in &self.splits {
                let rect = &layout[*i];
                match self.dir {
                    Dir::Horizontal => {
                        buffer.draw_v_rule(&rect.origin, rect.dimensions.height, &borders.style);
                        buffer.draw_char(
                            rect.origin.x,
                            rect.origin.y - 1,
                            DOWN_HORIZONTAL,
                            &borders.style,
                        );
                        buffer.draw_char(
                            rect.origin.x,
                            rect.origin.y + rect.dimensions.height,
                            UP_HORIZONTAL,
                            &borders.style,
                        );
                    }
                    Dir::Vertical => {
                        buffer.draw_h_rule(&rect.origin, rect.dimensions.width, &borders.style);
                        buffer.draw_char(
                            rect.origin.x - 1,
                            rect.origin.y,
                            VERTICAL_RIGHT,
                            &borders.style,
                        );
                        buffer.draw_char(
                            rect.origin.x + rect.dimensions.width,
                            rect.origin.y,
                            VERTICAL_LEFT,
                            &borders.style,
                        );
                    }
                }
            }
        }
    }
}

struct Split {
    width: Sizing,
    height: Sizing,
}

impl View for Split {
    fn sizing(&self, _: &Dimensions) -> Constraints {
        Constraints {
            width: self.width,
            height: self.height,
        }
    }

    // This is a no-op, since the actual rendering of splits is handled by
    // it's view. It is only implemented as a view so it can participate in
    // positioning.
    fn render(&self, _: &Rect, _: &mut Buffer) {}
}
