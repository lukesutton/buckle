use buckle::*;

fn main() {
    let paragraph = "A reasonably long bit of multi-line text,
which is used to demonstrate the rendering of text across 
muliple lines. It does also display the behaviour of this 
widget, which is to truncate the text, not wrap it.";

    let mut terminal = Terminal::new();

    let h_layout = Auto::horizontal()
        .layout(Layout::fixed(Align::Start, Align::Start, 1))
        .height(ContainerSizing::Fixed(35))
        .borders(LineStyle::default())
        .fill(FillStyle::new(
            None,
            Style::new().background(Color::DarkBlue),
        ))
        .add(MultilineText::new(paragraph.to_string()).width(ContainerSizing::Fixed(80)))
        .split()
        .add(
            Label::new("Fill".to_string())
                .width(ContainerSizing::Fill)
                .height(ContainerSizing::Fill)
                .style(
                    Style::new()
                        .bold()
                        .foreground(Color::Black)
                        .background(Color::Green),
                ),
        )
        .split()
        .add(Label::new("Hug".to_string()).height(ContainerSizing::Fill))
        .split()
        .add(
            Label::new("Fixed".to_string())
                .width(ContainerSizing::Fixed(20))
                .height(ContainerSizing::Fill),
        );

    let v_layout = Auto::vertical()
        .borders(LineStyle::default())
        .add(h_layout)
        .split()
        .add(
            Label::new("Ho there".to_string())
                .width(ContainerSizing::Fill)
                .height(ContainerSizing::Fill),
        );

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
