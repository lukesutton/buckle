use buckle::*;

fn main() {
    let paragraph = "A reasonably long bit of multi-line text,
which is used to demonstrate the rendering of text across 
muliple lines. It does also display the behaviour of this 
widget, which is to truncate the text, not wrap it.";

    let mut terminal = Terminal::new();

    let h_layout = Auto::new(
        Orientation::Horizontal,
        Arrangement::fixed(Alignment::Beginning, Alignment::Beginning, 1),
    )
    .add(
        AutoItemConstraint::Fixed(80),
        AutoItemConstraint::Fill,
        MultilineText::new(paragraph.to_string(), None),
    )
    .rule(None)
    .add(
        AutoItemConstraint::Fill,
        AutoItemConstraint::Fill,
        Text::new(
            "Fill".to_string(),
            Some(
                ContentStyle::new()
                    .attribute(Attribute::Bold)
                    .foreground(Color::Black)
                    .background(Color::Green),
            ),
        ),
    )
    .rule(None)
    .add(
        AutoItemConstraint::Hug,
        AutoItemConstraint::Fill,
        Text::new("Hug".to_string(), None),
    )
    .rule(None)
    .add(
        AutoItemConstraint::Fixed(20),
        AutoItemConstraint::Fill,
        Text::new("Fixed".to_string(), None),
    );

    let v_layout = Auto::new(Orientation::Vertical, Arrangement::default())
        .add(
            AutoItemConstraint::Fill,
            AutoItemConstraint::Fixed(20),
            h_layout,
        )
        .rule(None)
        .add(
            AutoItemConstraint::Fill,
            AutoItemConstraint::Fill,
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
