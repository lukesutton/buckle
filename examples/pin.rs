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
        .add(PinOrigin::TopLeft(Point::zero()), Label::new("Top Left"))
        .add(
            PinOrigin::TopLeft(Point::new(10, 4)),
            Label::new("Top Left Offset"),
        )
        .add(PinOrigin::TopRight(Point::zero()), Label::new("Top Right"))
        .add(
            PinOrigin::TopRight(Point::new(10, 4)),
            Label::new("Top Right Offset"),
        )
        .add(
            PinOrigin::BottomLeft(Point::zero()),
            Label::new("Bottom Left"),
        )
        .add(
            PinOrigin::BottomLeft(Point::new(10, 4)),
            Label::new("Bottom Left Offset"),
        )
        .add(
            PinOrigin::BottomRight(Point::zero()),
            Label::new("Bottom Right"),
        )
        .add(
            PinOrigin::BottomRight(Point::new(10, 4)),
            Label::new("Bottom Right Offset"),
        )
        .add(PinOrigin::Center, Label::new("Center"));

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
