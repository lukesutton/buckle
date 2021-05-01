use buckle::*;

fn main() {
    let mut terminal = Terminal::new();

    let layout = PinBoard::new(Sizing::Fill, Sizing::Fill)
        .add(
            PinOrigin::TopLeft(Point::zero()),
            Label::new(
                "Top Left".to_string(),
                ContainerSizing::Hug,
                ContainerSizing::Hug,
                None,
            ),
        )
        .add(
            PinOrigin::TopLeft(Point::new(10, 4)),
            Label::new(
                "Top Left Offset".to_string(),
                ContainerSizing::Hug,
                ContainerSizing::Hug,
                None,
            ),
        )
        .add(
            PinOrigin::TopRight(Point::zero()),
            Label::new(
                "Top Right".to_string(),
                ContainerSizing::Hug,
                ContainerSizing::Hug,
                None,
            ),
        )
        .add(
            PinOrigin::TopRight(Point::new(10, 4)),
            Label::new(
                "Top Right Offset".to_string(),
                ContainerSizing::Hug,
                ContainerSizing::Hug,
                None,
            ),
        )
        .add(
            PinOrigin::BottomLeft(Point::zero()),
            Label::new(
                "Bottom Left".to_string(),
                ContainerSizing::Hug,
                ContainerSizing::Hug,
                None,
            ),
        )
        .add(
            PinOrigin::BottomLeft(Point::new(10, 4)),
            Label::new(
                "Bottom Left Offset".to_string(),
                ContainerSizing::Hug,
                ContainerSizing::Hug,
                None,
            ),
        )
        .add(
            PinOrigin::BottomRight(Point::zero()),
            Label::new(
                "Bottom Right".to_string(),
                ContainerSizing::Hug,
                ContainerSizing::Hug,
                None,
            ),
        )
        .add(
            PinOrigin::BottomRight(Point::new(10, 4)),
            Label::new(
                "Bottom Right Offset".to_string(),
                ContainerSizing::Hug,
                ContainerSizing::Hug,
                None,
            ),
        )
        .add(
            PinOrigin::Center,
            Label::new(
                "Center".to_string(),
                ContainerSizing::Hug,
                ContainerSizing::Hug,
                None,
            ),
        );

    let mut buffer = terminal.prepare_buffer();
    layout.render(
        &Rect {
            origin: Point::zero(),
            dimensions: buffer.dimensions.clone(),
        },
        &mut buffer,
    );

    terminal.update(buffer);

    loop {}
}
