use crate::buffer::Buffer;
use crate::layouts::auto_solver::solve;
use crate::values::*;
use crate::views::View;

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
    items: Vec<Box<dyn View>>,
}

impl Auto {
    pub fn new(dir: Dir, layout: Layout, width: ContainerSizing, height: ContainerSizing) -> Self {
        Auto {
            dir,
            layout,
            width,
            height,
            items: Vec::new(),
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

    pub fn add<V: View>(mut self, item: V) -> Self {
        self.items.push(Box::new(item));
        self
    }

    pub fn add_each<I, F, V>(mut self, items: I, render: F) -> Self
    where
        I: IntoIterator,
        V: View,
        F: Fn(&I::Item) -> V,
    {
        for item in items {
            self.items.push(Box::new(render(&item)));
        }
        self
    }

    pub fn maybe_add<V: View>(mut self, check: bool, item: V) -> Self {
        if check {
            self.items.push(Box::new(item));
        }
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
    }
}
