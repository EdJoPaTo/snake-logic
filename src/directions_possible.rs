use crate::direction::Direction;

/// State which directions are possible or not.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy)]
pub struct DirectionsPossible {
    options: usize,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

#[allow(clippy::fn_params_excessive_bools)]
impl DirectionsPossible {
    #[must_use]
    pub const fn from_bools(left: bool, right: bool, up: bool, down: bool) -> Self {
        let options = left as usize + right as usize + up as usize + down as usize;
        Self {
            options,
            left,
            right,
            up,
            down,
        }
    }

    #[must_use]
    pub const fn intersection(first: &Self, second: &Self) -> Self {
        Self::from_bools(
            first.left && second.left,
            first.right && second.right,
            first.up && second.up,
            first.down && second.down,
        )
    }

    /// When only one [`Direction`](crate::Direction) is possible, it is returned.
    /// When no or more than one Direction is possible, `None` is returned.
    #[must_use]
    pub const fn single(&self) -> Option<Direction> {
        if self.options == 1 {
            self.any()
        } else {
            None
        }
    }

    /// Returns any of the possible [`Direction`s](crate::Direction).
    #[must_use]
    pub const fn any(&self) -> Option<Direction> {
        if self.left {
            Some(Direction::Left)
        } else if self.right {
            Some(Direction::Right)
        } else if self.up {
            Some(Direction::Up)
        } else if self.down {
            Some(Direction::Down)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn contains(&self, direction: Direction) -> bool {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
            Direction::Up => self.up,
            Direction::Down => self.down,
        }
    }

    /// Returns `true` when there is no possible [`Direction`](crate::Direction).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.options == 0
    }
}

impl core::fmt::Display for DirectionsPossible {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_empty() {
            fmt.write_str("None")
        } else {
            fmt.write_str(if self.contains(Direction::Left) {
                "L"
            } else {
                " "
            })?;
            fmt.write_str(if self.contains(Direction::Right) {
                "R"
            } else {
                " "
            })?;
            fmt.write_str(if self.contains(Direction::Up) {
                "U"
            } else {
                " "
            })?;
            fmt.write_str(if self.contains(Direction::Down) {
                "D"
            } else {
                " "
            })
        }
    }
}
