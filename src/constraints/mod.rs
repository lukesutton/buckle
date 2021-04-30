mod spacing;
use crate::values::*;
use spacing::Spacing;

// Accept a list of constraints and a rect
// Figure out how to fit all the constrained elements in the
pub fn solve_auto(
    items: &Vec<Constraints>,
    orientation: &Orientation,
    arrangement: &Arrangement,
    bounds: &Rect,
) -> Vec<Rect> {
    let rotate = &Orientation::Vertical == orientation;
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
        match constraints.horizontal {
            Constraint::Fixed(amt) => {
                result.dimensions.width = amt.clamp(0, remaining_bounds.dimensions.width);
                remaining_bounds.dimensions.width -= result.dimensions.width;
            }
            Constraint::Fill => fills.push(i),
        }

        // Determine height
        result.dimensions.height = match constraints.vertical {
            Constraint::Fill => remaining_bounds.dimensions.height,
            Constraint::Fixed(amt) => amt.clamp(0, remaining_bounds.dimensions.height),
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
            Alignment::Beginning => (),
            Alignment::Center => {
                result.origin.y = (offset_bounds.dimensions.height - result.dimensions.height) / 2
            }
            Alignment::End => {
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
            Alignment::Beginning => (),
            Alignment::Center => {
                let offset = (bounds.dimensions.width - total_width) / 2;
                for result in results.iter_mut() {
                    result.origin.x += offset;
                }
            }
            Alignment::End => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_fill_width_and_height() {
        let items = vec![Constraints::new(Constraint::Fill, Constraint::Fill)];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve_auto(
            &items,
            &Orientation::Horizontal,
            &Arrangement::default(),
            &bounds,
        );
        let solution = &results[0];
        assert_eq!(1, results.len());
        assert_eq!(&bounds, solution);
    }

    #[test]
    fn two_fill_width_and_height() {
        let items = vec![
            Constraints::new(Constraint::Fill, Constraint::Fill),
            Constraints::new(Constraint::Fill, Constraint::Fill),
        ];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve_auto(
            &items,
            &Orientation::Horizontal,
            &Arrangement::default(),
            &bounds,
        );
        assert_eq!(2, results.len());
        assert_eq!(results[0], Rect::new_from_raw(0, 0, 50, 100));
        assert_eq!(results[1], Rect::new_from_raw(50, 0, 50, 100));
    }

    #[test]
    fn two_fixed_width_fill_height() {
        let items = vec![
            Constraints::new(Constraint::Fixed(30), Constraint::Fill),
            Constraints::new(Constraint::Fixed(40), Constraint::Fill),
        ];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve_auto(
            &items,
            &Orientation::Horizontal,
            &Arrangement::default(),
            &bounds,
        );
        assert_eq!(2, results.len());
        assert_eq!(results[0], Rect::new_from_raw(0, 0, 30, 100));
        assert_eq!(results[1], Rect::new_from_raw(30, 0, 40, 100));
    }

    #[test]
    fn fixed_truncated_width() {
        let items = vec![
            Constraints::new(Constraint::Fixed(40), Constraint::Fill),
            Constraints::new(Constraint::Fixed(40), Constraint::Fill),
            Constraints::new(Constraint::Fixed(40), Constraint::Fill),
        ];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve_auto(
            &items,
            &Orientation::Horizontal,
            &Arrangement::default(),
            &bounds,
        );
        assert_eq!(3, results.len());
        assert_eq!(results[0], Rect::new_from_raw(0, 0, 40, 100));
        assert_eq!(results[1], Rect::new_from_raw(40, 0, 40, 100));
        assert_eq!(results[2], Rect::new_from_raw(80, 0, 20, 100));
    }

    #[test]
    fn fixed_and_filled_width() {
        let items = vec![
            Constraints::new(Constraint::Fill, Constraint::Fill),
            Constraints::new(Constraint::Fixed(20), Constraint::Fill),
        ];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve_auto(
            &items,
            &Orientation::Horizontal,
            &Arrangement::default(),
            &bounds,
        );
        assert_eq!(2, results.len());
        assert_eq!(results[0], Rect::new_from_raw(0, 0, 80, 100));
        assert_eq!(results[1], Rect::new_from_raw(80, 0, 20, 100));
    }

    #[test]
    fn fixed_center() {
        let items = vec![
            Constraints::new(Constraint::Fill, Constraint::Fill),
            Constraints::new(Constraint::Fixed(20), Constraint::Fill),
            Constraints::new(Constraint::Fill, Constraint::Fill),
        ];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let results = solve_auto(
            &items,
            &Orientation::Horizontal,
            &Arrangement::default(),
            &bounds,
        );
        assert_eq!(3, results.len());
        assert_eq!(results[0], Rect::new_from_raw(0, 0, 40, 100));
        assert_eq!(results[1], Rect::new_from_raw(40, 0, 20, 100));
        assert_eq!(results[2], Rect::new_from_raw(60, 0, 40, 100));
    }

    #[test]
    fn alignment_end() {
        let items = vec![Constraints::new(Constraint::Fixed(50), Constraint::Fill)];
        let bounds = Rect::new_from_raw(0, 0, 100, 50);
        let arrangement = Arrangement::Packed {
            vertical: Alignment::Beginning,
            horizontal: Alignment::End,
        };
        let results = solve_auto(&items, &Orientation::Horizontal, &arrangement, &bounds);
        assert_eq!(1, results.len());
        assert_eq!(results[0], Rect::new_from_raw(50, 0, 50, 50));
    }

    #[test]
    fn alignment_center() {
        let items = vec![Constraints::new(
            Constraint::Fixed(50),
            Constraint::Fixed(50),
        )];
        let bounds = Rect::new_from_raw(0, 0, 100, 100);
        let arrangement = Arrangement::Packed {
            vertical: Alignment::Beginning,
            horizontal: Alignment::Center,
        };
        let results = solve_auto(&items, &Orientation::Horizontal, &arrangement, &bounds);
        assert_eq!(1, results.len());
        assert_eq!(results[0], Rect::new_from_raw(25, 0, 50, 50));
    }

    mod vertical {
        use super::*;

        #[test]
        fn filled() {
            let items = vec![Constraints::new(Constraint::Fill, Constraint::Fill)];
            let bounds = Rect::new_from_raw(0, 0, 50, 100);
            let results = solve_auto(
                &items,
                &Orientation::Vertical,
                &Arrangement::default(),
                &bounds,
            );
            assert_eq!(1, results.len());
            assert_eq!(results[0], Rect::new_from_raw(0, 0, 50, 100));
        }

        #[test]
        fn two_fills_fixed_width() {
            let items = vec![
                Constraints::new(Constraint::Fixed(50), Constraint::Fill),
                Constraints::new(Constraint::Fixed(60), Constraint::Fill),
            ];
            let bounds = Rect::new_from_raw(0, 0, 100, 200);
            let results = solve_auto(
                &items,
                &Orientation::Vertical,
                &Arrangement::default(),
                &bounds,
            );
            assert_eq!(2, results.len());
            assert_eq!(results[0], Rect::new_from_raw(0, 0, 50, 100));
            assert_eq!(results[1], Rect::new_from_raw(0, 100, 60, 100));
        }

        #[test]
        fn fixed_and_filled() {
            let items = vec![
                Constraints::new(Constraint::Fixed(50), Constraint::Fill),
                Constraints::new(Constraint::Fill, Constraint::Fixed(120)),
            ];
            let bounds = Rect::new_from_raw(0, 0, 100, 200);
            let results = solve_auto(
                &items,
                &Orientation::Vertical,
                &Arrangement::default(),
                &bounds,
            );
            assert_eq!(2, results.len());
            assert_eq!(results[0], Rect::new_from_raw(0, 0, 50, 80));
            assert_eq!(results[1], Rect::new_from_raw(0, 80, 100, 120));
        }
    }
}
