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
        ContainerSizing::Fill,
        ContainerSizing::Fixed(35),
    )
    .borders(LineStyle::default())
    .fill(FillStyle::new(
        None,
        Style::new().background(Color::DarkBlue),
    ))
    .add(MultilineText::new(
        paragraph.to_string(),
        ContainerSizing::Fixed(80),
        ContainerSizing::Fill,
        None,
    ))
    .rule(None)
    .add(Label::new(
        "Fill".to_string(),
        ContainerSizing::Fill,
        ContainerSizing::Fill,
        Some(
            Style::new()
                .bold()
                .foreground(Color::Black)
                .background(Color::Green),
        ),
    ))
    .rule(None)
    .add(Label::new(
        "Hug".to_string(),
        ContainerSizing::Hug,
        ContainerSizing::Fill,
        None,
    ))
    .rule(None)
    .add(Label::new(
        "Fixed".to_string(),
        ContainerSizing::Fixed(20),
        ContainerSizing::Fill,
        None,
    ));

    let v_layout = Auto::new(
        Dir::Vertical,
        Layout::default(),
        ContainerSizing::Fill,
        ContainerSizing::Fill,
    )
    .borders(LineStyle::default())
    .add(h_layout)
    .rule(None)
    .add(Label::new(
        "Ho there".to_string(),
        ContainerSizing::Fill,
        ContainerSizing::Fill,
        None,
    ));

    let mut buffer = terminal.prepare_buffer();
    v_layout.render(
        &Rect {
            origin: Point::zero(),
            dimensions: buffer.dimensions.clone(),
        },
        &mut buffer,
    );

    terminal.update(buffer);

    loop {}
}
