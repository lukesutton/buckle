use buckle::*;

fn main() {
    let layout = ScrollBox::vertical(4)
        .add(Label::new("One"))
        .add(Label::new("Two"))
        .add(Label::new("Three"))
        .add(Label::new("Four"))
        .add(Label::new("Five"))
        .add(Label::new("Six"))
        .add(Label::new("Seven"))
        .add(Label::new("Eight"))
        .add(Label::new("Nine"))
        .add(Label::new("Ten"))
        .add(
            Auto::vertical()
                .borders(LineStyle::new(Corners::Regular, Stroke::Solid, None))
                .height(ContainerSizing::Fixed(300)),
        );

    let mut terminal = Terminal::new();
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
