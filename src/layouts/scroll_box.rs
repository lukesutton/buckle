use crate::buffer::Buffer;
use crate::layouts::Auto;
use crate::values::{Constraints, ContainerSizing, Dimensions, Dir, Point, Rect, Sizing};
use crate::views::View;

pub struct ScrollBox {
    position: usize,
    dir: Dir,
    width: Sizing,
    height: Sizing,
    contents: Auto,
}

const MAX_SIZE: usize = 1000;

impl ScrollBox {
    pub fn vertical(position: usize) -> Self {
        Self {
            position,
            dir: Dir::Vertical,
            width: Sizing::Fill,
            height: Sizing::Fill,
            contents: Auto::vertical().width(ContainerSizing::Hug),
        }
    }

    pub fn horizontal(position: usize) -> Self {
        Self {
            position,
            dir: Dir::Horizontal,
            width: Sizing::Fill,
            height: Sizing::Fill,
            contents: Auto::horizontal().height(ContainerSizing::Hug),
        }
    }

    pub fn add<V: 'static + View>(mut self, item: V) -> Self {
        self.contents = self.contents.add(item);
        self
    }

    pub fn width(mut self, width: Sizing) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Sizing) -> Self {
        self.height = height;
        self
    }
}

impl View for ScrollBox {
    fn sizing(&self, bounds: &Dimensions) -> Constraints {
        Constraints::new(
            self.width.constrain_by(bounds.width),
            self.height.constrain_by(bounds.height),
        )
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        match self.dir {
            Dir::Horizontal => {
                let content_dimensions = Dimensions::new(MAX_SIZE, within.dimensions.height);
                let mut content_buffer = Buffer::new(content_dimensions.clone());
                self.contents.render(
                    &Rect::new(Point::zero(), content_dimensions),
                    &mut content_buffer,
                );

                content_buffer.shrink(
                    Point::new(self.position, 0),
                    Point::new(
                        within.dimensions.width + self.position,
                        within.dimensions.height,
                    ),
                );

                buffer.merge(&within.origin, &content_buffer);
            }
            Dir::Vertical => {
                let content_dimensions = Dimensions::new(within.dimensions.width, MAX_SIZE);
                let mut content_buffer = Buffer::new(content_dimensions.clone());
                self.contents.render(
                    &Rect::new(Point::zero(), content_dimensions),
                    &mut content_buffer,
                );

                content_buffer.shrink(
                    Point::new(0, self.position),
                    Point::new(
                        within.dimensions.width,
                        within.dimensions.height + self.position,
                    ),
                );

                buffer.merge(&within.origin, &content_buffer);
            }
        }
    }
}
