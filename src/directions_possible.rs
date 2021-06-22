use std::collections::HashSet;

use crate::Direction;

pub struct DirectionsPossible {
    options: HashSet<Direction>,
}

#[allow(clippy::fn_params_excessive_bools)]
impl DirectionsPossible {
    #[must_use]
    pub fn from_bools(left: bool, right: bool, up: bool, down: bool) -> Self {
        let mut options = HashSet::new();
        if left {
            options.insert(Direction::Left);
        }
        if right {
            options.insert(Direction::Right);
        }
        if up {
            options.insert(Direction::Up);
        }
        if down {
            options.insert(Direction::Down);
        }
        Self { options }
    }

    #[must_use]
    pub fn intersection(first: &Self, second: &Self) -> Self {
        let options = first
            .options
            .intersection(&second.options)
            .cloned()
            .collect::<HashSet<_>>();
        Self { options }
    }

    #[must_use]
    pub fn single(&self) -> Option<&Direction> {
        if self.options.len() == 1 {
            self.options.iter().next()
        } else {
            None
        }
    }

    #[must_use]
    pub fn any(&self) -> Option<&Direction> {
        self.options.iter().next()
    }

    #[must_use]
    pub fn contains(&self, direction: &Direction) -> bool {
        self.options.contains(direction)
    }

    #[must_use]
    pub fn has_multiple(&self) -> bool {
        self.options.len() > 1
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }
}

impl core::fmt::Display for DirectionsPossible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.options.is_empty() {
            f.write_str("None")
        } else {
            f.write_str(if self.contains(&Direction::Left) {
                "L"
            } else {
                " "
            })?;
            f.write_str(if self.contains(&Direction::Right) {
                "R"
            } else {
                " "
            })?;
            f.write_str(if self.contains(&Direction::Up) {
                "U"
            } else {
                " "
            })?;
            f.write_str(if self.contains(&Direction::Down) {
                "D"
            } else {
                " "
            })
        }
    }
}
