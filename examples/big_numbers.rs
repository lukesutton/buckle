use buckle::*;

const ONE: &'static str = "─┐ 
 │ 
─┴─";

const TWO: &'static str = "┌─┐
┌─┘
└─┘";

fn main() {
    let mut terminal = Terminal::new();

    let layout = Auto::new(
        Dir::Horizontal,
        Layout::packed(Align::Start, Align::Start),
        ContainerSizing::Fill,
        ContainerSizing::Fill,
    )
    .add(
        MultilineText::new(ONE.to_string())
            .width(ContainerSizing::Fixed(3))
            .height(ContainerSizing::Fixed(3)),
    )
    .add(
        MultilineText::new(TWO.to_string())
            .width(ContainerSizing::Fixed(3))
            .height(ContainerSizing::Fixed(3)),
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
