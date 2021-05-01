use crate::values::*;

// Accept a list of constraints and a rect
// Figure out how to fit all the constrained elements in the
pub fn solve(
    items: &Vec<Constraints>,
    dir: &Dir,
    arrangement: &Layout,
    bounds: &Rect,
) -> Vec<Rect> {
    let rotate = &Dir::Vertical == dir;
    let mut arrangement = arrangement.clone();
    let mut bounds = bounds.clone();
    let mut items = items.clone();
    let mut spacing = Spacing::from_arrangement(&arrangement);
    spacing.introduce(&mut items);

    if rotate {
        bounds.rotate();
        arrangement.rotate();
        for item in items.iter_mut() {
            item.rotate();
        }
    }

    let mut remaining_bounds = bounds.clone();
    let mut results: Vec<Rect> = Vec::new();
    let mut fills = Vec::new();

    for (i, constraints) in items.iter().enumerate() {
        let mut result = remaining_bounds.clone();

        // Determine width
        match constraints.width {
            Sizing::Fixed(amt) => {
                result.dimensions.width = amt.clamp(0, remaining_bounds.dimensions.width);
                remaining_bounds.dimensions.width -= result.dimensions.width;
            }
            Sizing::Fill => fills.push(i),
        }

        // Determine height
        result.dimensions.height = match constraints.height {
            Sizing::Fill => remaining_bounds.dimensions.height,
            Sizing::Fixed(amt) => amt.clamp(0, remaining_bounds.dimensions.height),
        };

        results.push(result);
    }

    // Calculate the widths for the fills
    let fill_width = remaining_bounds
        .dimensions
        .width
        .checked_div(fills.len())
        .unwrap_or(0);

    // Updates widths for fills, and offsets
    let mut offset_bounds = bounds.clone();
    for (i, result) in results.iter_mut().enumerate() {
        if fills.contains(&i) {
            result.dimensions.width = fill_width;
        }
        result.origin.x = offset_bounds.origin.x;
        match arrangement.vertical() {
            Align::Start => (),
            Align::Center => {
                result.origin.y = (offset_bounds.dimensions.height - result.dimensions.height) / 2
            }
            Align::End => {
                result.origin.y = offset_bounds.dimensions.height - result.dimensions.height
            }
        }
        offset_bounds.origin.x += result.dimensions.width;
    }

    // Finally, if we have remaining space in the bounds, let's do horizontal
    // positioning
    let total_width = results.iter().fold(0, |x, y| x + y.dimensions.width);
    if total_width < bounds.dimensions.width {
        match arrangement.horizontal() {
            Align::Start => (),
            Align::Center => {
                let offset = (bounds.dimensions.width - total_width) / 2;
                for result in results.iter_mut() {
                    result.origin.x += offset;
                }
            }
            Align::End => {
                let offset = bounds.dimensions.width - total_width;
                for result in results.iter_mut() {
                    result.origin.x += offset;
                }
            }
        }
    }

    // Remove the spacer elements
    spacing.cleanup(&mut results);

    // Finally, rotate all the elements if needed
    if rotate {
        for result in results.iter_mut() {
            result.rotate();
        }
    }

    results
}

pub enum Spacing {
    Even(Vec<usize>),
    Fixed(usize, Vec<usize>),
    None,
}

impl Spacing {
    pub fn from_arrangement(arrangement: &Layout) -> Self {
        match arrangement {
            Layout::Packed { v: _, h: _ } => Self::None,
            Layout::Spaced {
                v: _,
                h: _,
                spacing,
            } => Self::Fixed(*spacing, Vec::new()),
            Layout::Spread { v: _, h: _ } => Self::Even(Vec::new()),
        }
    }

    pub fn introduce(&mut self, items: &mut Vec<Constraints>) {
        match self {
            Spacing::Even(indexes) => {
                if items.len() > 1 {
                    Self::insert_spaces(
                        Constraints::new(Sizing::Fill, Sizing::Fill),
                        indexes,
                        items,
                    );
                }
            }
            Spacing::Fixed(spacing, indexes) => {
                if items.len() > 1 {
                    Self::insert_spaces(
                        Constraints::new(Sizing::Fixed(*spacing), Sizing::Fill),
                        indexes,
                        items,
                    );
                }
            }
            Spacing::None => (),
        }
    }

    fn insert_spaces(
        constraints: Constraints,
        indexes: &mut Vec<usize>,
        items: &mut Vec<Constraints>,
    ) {
        let mut current = 1;
        for _ in 0..(items.len() - 1) {
            items.insert(current, constraints.clone());
            indexes.push(current);
            current += 2;
        }
        // Put indexes in reverse order, since that's the order
        // they should be removed in.
        indexes.reverse();
    }

    pub fn cleanup(&self, results: &mut Vec<Rect>) {
        match self {
            Spacing::Even(indexes) | Spacing::Fixed(_, indexes) => {
                for i in indexes {
                    let _ = results.remove(*i);
                }
            }
            Spacing::None => (),
        }
    }

    #[cfg(test)]
    pub fn indexes(&self) -> Vec<usize> {
        match self {
            Spacing::Even(indexes) => indexes.clone(),
            Spacing::Fixed(_, indexes) => indexes.clone(),
            Spacing::None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod auto_layout_tests {
    use super::*;

    #[test]
    fn one_fill_width_and_height() {
        let items = vec![Constraints::new(Sizing::Fill, Sizing::Fill)];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve(&items, &Dir::Horizontal, &Layout::default(), &bounds);
        let solution = &results[0];
        assert_eq!(1, results.len());
        assert_eq!(&bounds, solution);
    }

    #[test]
    fn two_fill_width_and_height() {
        let items = vec![
            Constraints::new(Sizing::Fill, Sizing::Fill),
            Constraints::new(Sizing::Fill, Sizing::Fill),
        ];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve(&items, &Dir::Horizontal, &Layout::default(), &bounds);
        assert_eq!(2, results.len());
        assert_eq!(results[0], Rect::new_from_raw(0, 0, 50, 100));
        assert_eq!(results[1], Rect::new_from_raw(50, 0, 50, 100));
    }

    #[test]
    fn two_fixed_width_fill_height() {
        let items = vec![
            Constraints::new(Sizing::Fixed(30), Sizing::Fill),
            Constraints::new(Sizing::Fixed(40), Sizing::Fill),
        ];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve(&items, &Dir::Horizontal, &Layout::default(), &bounds);
        assert_eq!(2, results.len());
        assert_eq!(results[0], Rect::new_from_raw(0, 0, 30, 100));
        assert_eq!(results[1], Rect::new_from_raw(30, 0, 40, 100));
    }

    #[test]
    fn fixed_truncated_width() {
        let items = vec![
            Constraints::new(Sizing::Fixed(40), Sizing::Fill),
            Constraints::new(Sizing::Fixed(40), Sizing::Fill),
            Constraints::new(Sizing::Fixed(40), Sizing::Fill),
        ];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve(&items, &Dir::Horizontal, &Layout::default(), &bounds);
        assert_eq!(3, results.len());
        assert_eq!(results[0], Rect::new_from_raw(0, 0, 40, 100));
        assert_eq!(results[1], Rect::new_from_raw(40, 0, 40, 100));
        assert_eq!(results[2], Rect::new_from_raw(80, 0, 20, 100));
    }

    #[test]
    fn fixed_and_filled_width() {
        let items = vec![
            Constraints::new(Sizing::Fill, Sizing::Fill),
            Constraints::new(Sizing::Fixed(20), Sizing::Fill),
        ];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve(&items, &Dir::Horizontal, &Layout::default(), &bounds);
        assert_eq!(2, results.len());
        assert_eq!(results[0], Rect::new_from_raw(0, 0, 80, 100));
        assert_eq!(results[1], Rect::new_from_raw(80, 0, 20, 100));
    }

    #[test]
    fn fixed_center() {
        let items = vec![
            Constraints::new(Sizing::Fill, Sizing::Fill),
            Constraints::new(Sizing::Fixed(20), Sizing::Fill),
            Constraints::new(Sizing::Fill, Sizing::Fill),
        ];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve(&items, &Dir::Horizontal, &Layout::default(), &bounds);
        assert_eq!(3, results.len());
        assert_eq!(results[0], Rect::new_from_raw(0, 0, 40, 100));
        assert_eq!(results[1], Rect::new_from_raw(40, 0, 20, 100));
        assert_eq!(results[2], Rect::new_from_raw(60, 0, 40, 100));
    }

    #[test]
    fn alignment_end() {
        let items = vec![Constraints::new(Sizing::Fixed(50), Sizing::Fill)];
        let bounds = Rect::new_from_raw(0, 0, 100, 50);
        let arrangement = Layout::Packed {
            v: Align::Start,
            h: Align::End,
        };
        let results = solve(&items, &Dir::Horizontal, &arrangement, &bounds);
        assert_eq!(1, results.len());
        assert_eq!(results[0], Rect::new_from_raw(50, 0, 50, 50));
    }

    #[test]
    fn alignment_center() {
        let items = vec![Constraints::new(Sizing::Fixed(50), Sizing::Fixed(50))];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let arrangement = Layout::Packed {
            v: Align::Start,
            h: Align::Center,
        };
        let results = solve(&items, &Dir::Horizontal, &arrangement, &bounds);
        assert_eq!(1, results.len());
        assert_eq!(results[0], Rect::new_from_raw(25, 0, 50, 50));
    }

    mod vertical {
        use super::*;

        #[test]
        fn filled() {
            let items = vec![Constraints::new(Sizing::Fill, Sizing::Fill)];
            let bounds = Rect::new_from_raw(0, 0, 50, 100);
            let results = solve(&items, &Dir::Vertical, &Layout::default(), &bounds);
            assert_eq!(1, results.len());
            assert_eq!(results[0], Rect::new_from_raw(0, 0, 50, 100));
        }

        #[test]
        fn two_fills_fixed_width() {
            let items = vec![
                Constraints::new(Sizing::Fixed(50), Sizing::Fill),
                Constraints::new(Sizing::Fixed(60), Sizing::Fill),
            ];
            let bounds = Rect::new_from_raw(0, 0, 100, 200);
            let results = solve(&items, &Dir::Vertical, &Layout::default(), &bounds);
            assert_eq!(2, results.len());
            assert_eq!(results[0], Rect::new_from_raw(0, 0, 50, 100));
            assert_eq!(results[1], Rect::new_from_raw(0, 100, 60, 100));
        }

        #[test]
        fn fixed_and_filled() {
            let items = vec![
                Constraints::new(Sizing::Fixed(50), Sizing::Fill),
                Constraints::new(Sizing::Fill, Sizing::Fixed(120)),
            ];
            let bounds = Rect::new_from_raw(0, 0, 100, 200);
            let results = solve(&items, &Dir::Vertical, &Layout::default(), &bounds);
            assert_eq!(2, results.len());
            assert_eq!(results[0], Rect::new_from_raw(0, 0, 50, 80));
            assert_eq!(results[1], Rect::new_from_raw(0, 80, 100, 120));
        }
    }
}

#[cfg(test)]
mod spacing_tests {
    use super::*;
    use crate::values::Align;

    #[test]
    fn even_spacing() {
        let mut items = vec![
            Constraints::new(Sizing::Fixed(10), Sizing::Fill),
            Constraints::new(Sizing::Fixed(10), Sizing::Fill),
            Constraints::new(Sizing::Fixed(10), Sizing::Fill),
            Constraints::new(Sizing::Fixed(10), Sizing::Fill),
        ];

        let expected = vec![
            Constraints::new(Sizing::Fixed(10), Sizing::Fill),
            Constraints::new(Sizing::Fill, Sizing::Fill),
            Constraints::new(Sizing::Fixed(10), Sizing::Fill),
            Constraints::new(Sizing::Fill, Sizing::Fill),
            Constraints::new(Sizing::Fixed(10), Sizing::Fill),
            Constraints::new(Sizing::Fill, Sizing::Fill),
            Constraints::new(Sizing::Fixed(10), Sizing::Fill),
        ];

        let mut spacing = Spacing::from_arrangement(&Layout::Spread {
            v: Align::Start,
            h: Align::Start,
        });
        spacing.introduce(&mut items);
        assert_eq!(7, items.len());
        assert_eq!(vec![5, 3, 1], spacing.indexes());
        assert_eq!(expected, items);
    }
}
