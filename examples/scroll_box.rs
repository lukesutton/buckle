use buckle::*;

fn main() {
    let mut terminal = Terminal::new();

    let layout = Auto::horizontal()
        .add(Label::new("Start".to_string()).width(ContainerSizing::Fixed(30)))
        .rule(None)
        .add(
            ScrollBox::new(Dir::Vertical, 0)
                .add(
                    Label::new("1".to_string())
                        .width(ContainerSizing::Fill)
                        .height(ContainerSizing::Fixed(10))
                        .style(Style::new().background(Color::Red)),
                )
                .add(
                    Label::new("2".to_string())
                        .width(ContainerSizing::Fill)
                        .height(ContainerSizing::Fixed(10))
                        .style(Style::new().background(Color::Blue)),
                )
                .add(
                    Label::new("3".to_string())
                        .width(ContainerSizing::Fill)
                        .height(ContainerSizing::Fixed(10))
                        .style(Style::new().background(Color::Green)),
                )
                .add(
                    Label::new("4".to_string())
                        .width(ContainerSizing::Fill)
                        .height(ContainerSizing::Fixed(10))
                        .style(Style::new().background(Color::Magenta)),
                )
                .add(
                    Label::new("5".to_string())
                        .width(ContainerSizing::Fill)
                        .height(ContainerSizing::Fixed(10))
                        .style(Style::new().background(Color::Red)),
                )
                .add(
                    Label::new("6".to_string())
                        .width(ContainerSizing::Fill)
                        .height(ContainerSizing::Fixed(10))
                        .style(Style::new().background(Color::Blue)),
                )
                .add(
                    Label::new("7".to_string())
                        .width(ContainerSizing::Fill)
                        .height(ContainerSizing::Fixed(10))
                        .style(Style::new().background(Color::Green)),
                )
                .add(
                    Label::new("8".to_string())
                        .width(ContainerSizing::Fill)
                        .height(ContainerSizing::Fixed(10))
                        .style(Style::new().background(Color::Magenta)),
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
