use crate::buffer::Buffer;
use crate::layouts::auto::Auto;
use crate::values::{Constraints, ContainerSizing, Dimensions, Dir, Point, Rect, Sizing};
use crate::views::View;

pub struct ScrollBox {
    dir: Dir,
    width: Sizing,
    height: Sizing,
    position: usize,
    layout: Auto,
}

impl ScrollBox {
    pub fn new(dir: Dir, position: usize) -> Self {
        let layout = match dir {
            Dir::Horizontal => Auto::horizontal().height(ContainerSizing::Hug),
            Dir::Vertical => Auto::vertical().width(ContainerSizing::Hug),
        };

        Self {
            dir,
            width: Sizing::Fill,
            height: Sizing::Fill,
            position,
            layout,
        }
    }

    pub fn width(mut self, width: Sizing) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Sizing) -> Self {
        self.height = height;
        self
    }

    pub fn add<V: 'static + View>(mut self, item: V) -> Self {
        self.layout = self.layout.add(item);
        self
    }
}

impl View for ScrollBox {
    fn sizing(&self, _: &Dimensions) -> Constraints {
        Constraints {
            width: self.width,
            height: self.height,
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        // TODO: Clamp position to a value in order to prevent truncation outside of the buffer
        // This may not be necessary, but should at least be investigated
        let mut interior = Dimensions::new(within.dimensions.width - 2, within.dimensions.height);
        interior.height = match self.layout.sizing(&interior).height {
            Sizing::Fill => within.dimensions.height,
            Sizing::Fixed(h) => h,
        };
        // tthis should actually be the same size as the constraints given by the auto layout
        let mut interior_buffer = Buffer::new(interior.clone());
        self.layout.render(
            &Rect::new(Point::zero(), interior.clone()),
            &mut interior_buffer,
        );
        // Truncate interior buffer to offset.
        interior_buffer.truncate(
            Point::new(0, self.position),
            Point::new(interior.width, self.position + interior.height),
        );
        buffer.merge(&within.origin, &interior_buffer);

        // Draw scrollbar

        // Make a dummy buffer to render the auto layout into
        // Inset it as necessary for any borders and the scroll bar
        // Based on the available dimensions and the scroll offset, grab a slice of the mutated buffer
        // Merge it into the main buffer at the correct offsets
    }
}
