use buckle::*;

fn main() {
    let paragraph = "A reasonably long bit of multi-line text,
which is used to demonstrate the rendering of text across 
muliple lines. It does also display the behaviour of this 
widget, which is to truncate the text, not wrap it.";

    let mut terminal = Terminal::new();

    let h_layout = Border::new(
        Stroke::SolidRounded,
        Styled::new(
            Auto::horizontal()
                .layout(Layout::fixed(Align::Start, Align::Start, 1))
                .height(ContainerSizing::Fixed(35))
                .add(MultilineText::new(paragraph).width(ContainerSizing::Fixed(80)))
                .add(
                    Label::new("Fill")
                        .width(ContainerSizing::Fill)
                        .height(ContainerSizing::Fill),
                )
                .add(Label::new("Hug").height(ContainerSizing::Fill))
                .add(
                    Label::new("Fixed")
                        .width(ContainerSizing::Fixed(20))
                        .height(ContainerSizing::Fill),
                ),
        )
        .background(Color::DarkBlue),
    );

    let v_layout = Border::new(
        Stroke::SolidRounded,
        Auto::vertical().add(h_layout).add(
            Auto::vertical()
                .height(ContainerSizing::Fixed(4))
                .add(Label::new("Before"))
                .add(Rule::new(Dir::Horizontal))
                .add(Label::new("After"))
                .add(Label::new("Another After"))
                .add(Label::new("More After"))
                .add(Label::new("Final After")),
        ),
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
