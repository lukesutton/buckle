use buckle::*;

fn main() {
    let mut terminal = Terminal::new();

    let layout = PinBoard::new(Sizing::Fill, Sizing::Fill)
        .borders(LineStyle::default())
        .fill(FillStyle::new(
            Some('.'),
            Style::new()
                .background(Color::DarkGrey)
                .foreground(Color::Grey),
        ))
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
            origin: Point::new(10, 5),
            dimensions: Dimensions {
                width: buffer.dimensions.width - 20,
                height: buffer.dimensions.height - 10,
            },
        },
        &mut buffer,
    );

    terminal.update(buffer);

    loop {}
}
