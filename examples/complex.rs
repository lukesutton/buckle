use buckle::*;

fn main() {
    let mut terminal = Terminal::new();
    let mut buffer = terminal.prepare_buffer();
    layout().render(
        &Rect {
            origin: Point::zero(),
            dimensions: buffer.dimensions.clone(),
        },
        &mut buffer,
    );

    terminal.update(buffer);

    loop {}
}

fn layout() -> PinBoard {
    PinBoard::new(Sizing::Fill, Sizing::Fill).add(
        PinOrigin::TopLeft(Point::zero()),
        Auto::horizontal()
            .add(
                Auto::vertical()
                    // .layout(Layout::fixed(Align::Start, Align::Start, 1))
                    .width(ContainerSizing::Fixed(22))
                    .add(BackgroundColor::new(
                        Color::Green,
                        ForegroundColor::new(
                            Color::Black,
                            Padding::all(
                                1,
                                Auto::vertical()
                                    .height(ContainerSizing::Hug)
                                    .add(Label::new(" ◀ ▶ PATTERNS").width(ContainerSizing::Fill))
                                    .add(
                                        Label::new(" ▲ ▼ INDEX: 001").width(ContainerSizing::Fill),
                                    ),
                            ),
                        ),
                    ))
                    .add(Padding::horizontal(1, BigNumbers::new(&144.10)))
                    .add(Label::new(" POS: 00:00:00"))
                    .add(Label::new(" SEQ: 00:00:00"))
                    .add(Label::new(" STP: 00:00:00")),
            )
            .add(
                Auto::vertical()
                    .borders(LineStyle::new(Corners::Regular, Stroke::Solid, None))
                    .add(
                        Auto::horizontal()
                            .height(ContainerSizing::Hug)
                            .layout(Layout::fixed(Align::Start, Align::Start, 1))
                            .add(Label::new("  # "))
                            .add(Label::new("COM"))
                            .add(Label::new("TAR"))
                            .add(Label::new("FRQ"))
                            .rule(None)
                            .add_each(1..=16, |i| Label::new(&format!("{:->3}", i))),
                    )
                    .split()
                    .add_each(1..=8, |i| {
                        Auto::horizontal()
                            .height(ContainerSizing::Hug)
                            .layout(Layout::fixed(Align::Start, Align::Start, 1))
                            .add(Label::new(&format!(" {:->3}", i)))
                            .add(Label::new("···"))
                            .add(Label::new("···"))
                            .add(Label::new("···"))
                            .rule(None)
                            .add_each(1..=16, |i| Label::new("···"))
                    }),
            ),
    )
}

const ZERO: &'static str = "┌─┐
│/│
└─┘";
const ONE: &'static str = "─┐ 
 │ 
─┴─";
const TWO: &'static str = "┌─┐
┌─┘
└─┘";
const THREE: &'static str = "┌─┐
 ─┤
└─┘";
const FOUR: &'static str = "┌ ┐ 
└─┤ 
  ┘";
const FIVE: &'static str = "┌─┐
└─┐
└─┘";
const SIX: &'static str = "┌─┐
├─┐
└─┘";
const SEVEN: &'static str = "┌─┐
  │
  ┘";
const EIGHT: &'static str = "┌─┐
├─┤
└─┘";
const NINE: &'static str = "┌─┐
└─┤
  ┘";
const DECIMAL: &'static str = " 
 
°";

struct BigNumbers {
    layout: Auto,
}

impl BigNumbers {
    fn new(value: &f32) -> Self {
        let mut layout = Auto::horizontal()
            .width(ContainerSizing::Fill)
            .height(ContainerSizing::Fixed(3));
        let formatted = format!("{:.2}", value);
        for i in formatted.chars() {
            layout = match i {
                '0' => layout.add(MultilineText::new(ZERO)),
                '1' => layout.add(MultilineText::new(ONE)),
                '2' => layout.add(MultilineText::new(TWO)),
                '3' => layout.add(MultilineText::new(THREE)),
                '4' => layout.add(MultilineText::new(FOUR)),
                '5' => layout.add(MultilineText::new(FIVE)),
                '6' => layout.add(MultilineText::new(SIX)),
                '7' => layout.add(MultilineText::new(SEVEN)),
                '8' => layout.add(MultilineText::new(EIGHT)),
                '9' => layout.add(MultilineText::new(NINE)),
                '.' => layout.add(MultilineText::new(DECIMAL)),
                _ => layout.add(MultilineText::new(ZERO)),
            };
        }
        Self { layout }
    }
}

impl View for BigNumbers {
    fn sizing(&self, bounds: &Dimensions) -> Constraints {
        self.layout.sizing(&bounds)
    }

    fn render(&self, within: &Rect, buffer: &mut Buffer) {
        self.layout.render(&within, buffer);
    }
}
