use crate::direction::Direction;

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
        let mut options = 0;
        if left {
            options += 1;
        }
        if right {
            options += 1;
        }
        if up {
            options += 1;
        }
        if down {
            options += 1;
        }
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

    #[must_use]
    pub const fn single(&self) -> Option<Direction> {
        if self.options == 1 {
            self.any()
        } else {
            None
        }
    }

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

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.options == 0
    }
}

impl core::fmt::Display for DirectionsPossible {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_empty() {
            f.write_str("None")
        } else {
            f.write_str(if self.contains(Direction::Left) {
                "L"
            } else {
                " "
            })?;
            f.write_str(if self.contains(Direction::Right) {
                "R"
            } else {
                " "
            })?;
            f.write_str(if self.contains(Direction::Up) {
                "U"
            } else {
                " "
            })?;
            f.write_str(if self.contains(Direction::Down) {
                "D"
            } else {
                " "
            })
        }
    }
}
