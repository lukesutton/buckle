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
        Orientation::Horizontal,
        Arrangement::packed(Alignment::Beginning, Alignment::Beginning),
    )
    .add(
        ContainerSizing::Fixed(3),
        ContainerSizing::Fixed(3),
        MultilineText::new(ONE.to_string(), None),
    )
    .add(
        ContainerSizing::Fixed(3),
        ContainerSizing::Fixed(3),
        MultilineText::new(TWO.to_string(), None),
    );

    let mut buffer = terminal.prepare_buffer();
    layout.render(
        &Rect {
            origin: Point::zero(),
            dimensions: Dimensions::new(buffer.width, buffer.height),
        },
        &mut buffer,
    );

    terminal.update(buffer);

    loop {}
}
