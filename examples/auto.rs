use buckle::*;

fn main() {
    let paragraph = "A reasonably long bit of multi-line text,
which is used to demonstrate the rendering of text across 
muliple lines. It does also display the behaviour of this 
widget, which is to truncate the text, not wrap it.";

    let mut terminal = Terminal::new();

    let h_layout = Auto::new(
        Dir::Horizontal,
        Layout::fixed(Align::Start, Align::Start, 1),
    )
    .add(
        ContainerSizing::Fixed(80),
        ContainerSizing::Fill,
        MultilineText::new(paragraph.to_string(), None),
    )
    .rule(None)
    .add(
        ContainerSizing::Fill,
        ContainerSizing::Fill,
        Text::new(
            "Fill".to_string(),
            Some(
                Style::new()
                    .bold()
                    .foreground(Color::Black)
                    .background(Color::Green),
            ),
        ),
    )
    .rule(None)
    .add(
        ContainerSizing::Hug,
        ContainerSizing::Fill,
        Text::new("Hug".to_string(), None),
    )
    .rule(None)
    .add(
        ContainerSizing::Fixed(20),
        ContainerSizing::Fill,
        Text::new("Fixed".to_string(), None),
    );

    let v_layout = Auto::new(Dir::Vertical, Layout::default())
        .add(ContainerSizing::Fill, ContainerSizing::Fixed(20), h_layout)
        .rule(None)
        .add(
            ContainerSizing::Fill,
            ContainerSizing::Fill,
            Text::new("Ho there".to_string(), None),
        );

    let mut buffer = terminal.prepare_buffer();
    v_layout.render(
        &Rect {
            origin: Point::zero(),
            dimensions: Dimensions::new(buffer.width, buffer.height),
        },
        &mut buffer,
    );

    terminal.update(buffer);

    loop {}
}
