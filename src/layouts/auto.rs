use crate::buffer::Buffer;
use crate::layouts::auto_solver::solve;
use crate::styles::{LineStyle, Style};
use crate::values::*;
use crate::views::{HRule, Spacer, VRule, View};

/// A layout which positions it's children automatically based on their size
/// and the constraints provided.
pub struct Auto {
    dir: Dir,
    layout: Layout,
    width: ContainerSizing,
    height: ContainerSizing,
    borders: Option<LineStyle>,
    items: Vec<Box<dyn View>>,
}

impl Auto {
    pub fn new(
        dir: Dir,
        layout: Layout,
        width: ContainerSizing,
        height: ContainerSizing,
        borders: Option<LineStyle>,
    ) -> Self {
        Auto {
            dir,
            layout,
            width,
            height,
            borders,
            items: Vec::new(),
        }
    }

    pub fn add<V: 'static + View>(mut self, item: V) -> Self {
        self.items.push(Box::new(item));
        self
    }

    pub fn rule(mut self, style: Option<Style>) -> Self {
        match self.dir {
            Dir::Horizontal => self.items.push(Box::new(VRule::new(style))),
            Dir::Vertical => self.items.push(Box::new(HRule::new(style))),
        }

        self
    }

    pub fn spacer(mut self) -> Self {
        self.items.push(Box::new(Spacer {}));
        self
    }
}

impl View for Auto {
    fn sizing(&self, bounds: &Dimensions) -> Constraints {
        let mut height = 0;
        let mut width = 0;
        for item in &self.items {
            let constraints = item.sizing(&bounds);
            if let Sizing::Fixed(w) = constraints.width {
                if w > width {
                    width = w
                }
            }
            if let Sizing::Fixed(h) = constraints.height {
                if h > height {
                    height = h
                }
            }
        }

        Constraints {
            width: self.width.simplify(width),
            height: self.height.simplify(height),
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        let mut within = within.clone();
        if let Some(borders) = &self.borders {
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
            item.render(&rect, buffer);
        }
    }
}
