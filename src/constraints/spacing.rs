use crate::values::{Arrangement, Constraint, Constraints, Rect};

pub enum Spacing {
    Even(Vec<usize>),
    Fixed(usize, Vec<usize>),
    None,
}

impl Spacing {
    pub fn from_arrangement(arrangement: &Arrangement) -> Self {
        match arrangement {
            Arrangement::Packed {
                vertical: _,
                horizontal: _,
            } => Self::None,
            Arrangement::FixedSpacing {
                vertical: _,
                horizontal: _,
                spacing,
            } => Self::Fixed(*spacing, Vec::new()),
            Arrangement::EvenSpacing {
                vertical: _,
                horizontal: _,
            } => Self::Even(Vec::new()),
        }
    }

    pub fn introduce(&mut self, items: &mut Vec<Constraints>) {
        match self {
            Spacing::Even(indexes) => {
                if items.len() > 1 {
                    Self::insert_spaces(
                        Constraints::new(Constraint::Fill, Constraint::Fill),
                        indexes,
                        items,
                    );
                }
            }
            Spacing::Fixed(spacing, indexes) => {
                if items.len() > 1 {
                    Self::insert_spaces(
                        Constraints::new(Constraint::Fixed(*spacing), Constraint::Fill),
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
mod tests {
    use super::*;
    use crate::values::Alignment;

    #[test]
    fn even_spacing() {
        let mut items = vec![
            Constraints::new(Constraint::Fixed(10), Constraint::Fill),
            Constraints::new(Constraint::Fixed(10), Constraint::Fill),
            Constraints::new(Constraint::Fixed(10), Constraint::Fill),
            Constraints::new(Constraint::Fixed(10), Constraint::Fill),
        ];

        let expected = vec![
            Constraints::new(Constraint::Fixed(10), Constraint::Fill),
            Constraints::new(Constraint::Fill, Constraint::Fill),
            Constraints::new(Constraint::Fixed(10), Constraint::Fill),
            Constraints::new(Constraint::Fill, Constraint::Fill),
            Constraints::new(Constraint::Fixed(10), Constraint::Fill),
            Constraints::new(Constraint::Fill, Constraint::Fill),
            Constraints::new(Constraint::Fixed(10), Constraint::Fill),
        ];

        let mut spacing = Spacing::from_arrangement(&Arrangement::EvenSpacing {
            vertical: Alignment::Beginning,
            horizontal: Alignment::Beginning,
        });
        spacing.introduce(&mut items);
        assert_eq!(7, items.len());
        assert_eq!(vec![5, 3, 1], spacing.indexes());
        assert_eq!(expected, items);
    }
}
