use buckle::*;

const ONE: &'static str = " ─┐ 
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
    .add(MultilineText::new(
        ONE.to_string(),
        ContainerSizing::Fixed(3),
        ContainerSizing::Fixed(3),
        None,
    ))
    .add(MultilineText::new(
        TWO.to_string(),
        ContainerSizing::Fixed(3),
        ContainerSizing::Fixed(3),
        None,
    ));

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
