use crate::buffer::Buffer;
use crate::values::{Constraints, Dimensions, Point, Rect, Sizing};
use crate::views::View;

pub struct PinBoard {
    width: Sizing,
    height: Sizing,
    pins: Vec<Pin>,
}

impl PinBoard {
    pub fn new(width: Sizing, height: Sizing) -> Self {
        Self {
            width,
            height,
            pins: Vec::new(),
        }
    }

    pub fn add<V: View>(mut self, origin: PinOrigin, item: V) -> Self {
        self.pins.push(Pin::new(origin, item));
        self
    }

    pub fn maybe_add<V: View>(mut self, check: bool, origin: PinOrigin, item: V) -> Self {
        if check {
            self.pins.push(Pin::new(origin, item));
        }
        self
    }
}

impl View for PinBoard {
    fn sizing(&self, _: &Dimensions) -> Constraints {
        Constraints {
            width: self.width,
            height: self.height,
        }
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        for pin in &self.pins {
            match &pin.origin {
                PinOrigin::TopLeft(point)
                | PinOrigin::TopRight(point)
                | PinOrigin::BottomLeft(point)
                | PinOrigin::BottomRight(point) => {
                    let mut dimensions = Dimensions::new(
                        within.dimensions.width - point.x,
                        within.dimensions.height - point.y,
                    );
                    let constraints = pin.item.sizing(&dimensions);
                    if let Sizing::Fixed(size) = constraints.width {
                        dimensions.width = size.clamp(0, dimensions.width)
                    }
                    if let Sizing::Fixed(size) = constraints.height {
                        dimensions.height = size.clamp(0, dimensions.height)
                    }

                    let offset = match &pin.origin {
                        PinOrigin::TopLeft(_) => {
                            Point::new(point.x + within.origin.x, point.y + within.origin.y)
                        }
                        PinOrigin::TopRight(_) => Point::new(
                            (within.dimensions.width - dimensions.width - point.x)
                                + within.origin.x,
                            within.origin.y + point.y,
                        ),
                        PinOrigin::BottomLeft(_) => Point::new(
                            point.x + within.origin.x,
                            (within.dimensions.height - dimensions.height - point.y)
                                + within.origin.y,
                        ),
                        PinOrigin::BottomRight(_) => Point::new(
                            (within.dimensions.width - dimensions.width - point.x)
                                + within.origin.x,
                            (within.dimensions.height - dimensions.height - point.y)
                                + within.origin.y,
                        ),
                        _ => within.origin.clone(),
                    };

                    let rect = Rect::new(offset, dimensions);
                    pin.item.render(&rect, buffer);
                }
                PinOrigin::Center => {
                    let constraints = pin.item.sizing(&within.dimensions);
                    let dimensions = Dimensions::new(
                        match constraints.width {
                            Sizing::Fill => within.dimensions.width,
                            Sizing::Fixed(size) => size.clamp(0, within.dimensions.width),
                        },
                        match constraints.height {
                            Sizing::Fill => within.dimensions.height,
                            Sizing::Fixed(size) => size.clamp(0, within.dimensions.height),
                        },
                    );
                    let point = Point::new(
                        (within.dimensions.width - dimensions.width) / 2 + within.origin.x,
                        (within.dimensions.height - dimensions.height) / 2 + within.origin.y,
                    );
                    let rect = Rect::new(point, dimensions);
                    pin.item.render(&rect, buffer);
                }
            }
        }
    }
}

pub enum PinOrigin {
    TopLeft(Point),
    TopRight(Point),
    BottomLeft(Point),
    BottomRight(Point),
    Center,
}

pub struct Pin {
    origin: PinOrigin,
    item: Box<dyn View>,
}

impl Pin {
    pub fn new<V: View>(origin: PinOrigin, item: V) -> Self {
        Self {
            origin,
            item: Box::new(item),
        }
    }
}
