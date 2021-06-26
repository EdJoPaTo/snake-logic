use crate::{Direction, DirectionsPossible, Point, ALL_DIRECTIONS};

/// Contains the previous chosen directions.
/// Contains up to the 4 directions in the order of last usage.
pub fn generate(snake: &[Point]) -> Vec<Direction> {
    let mut result = Vec::with_capacity(4);

    let mut iter = snake.iter();
    let mut headwards = iter.next().expect("Snake has to have a head");
    for behind in iter {
        if result.len() >= 3 {
            // 3 -> one is missing
            for d in &ALL_DIRECTIONS {
                if !result.contains(d) {
                    result.push(d.clone());
                }
            }
            break;
        }

        let direction = if behind.x == headwards.x {
            if behind.y > headwards.y {
                Direction::Up
            } else {
                Direction::Down
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if behind.x > headwards.x {
                Direction::Left
            } else {
                Direction::Right
            }
        };
        if !result.contains(&direction) {
            result.push(direction);
        }
        headwards = behind;
    }
    result
}

pub fn repeat<'a, P>(previous: P, possible: &DirectionsPossible) -> Option<&'a Direction>
where
    P: IntoIterator<Item = &'a Direction>,
{
    for p in previous {
        for d in &ALL_DIRECTIONS {
            if p == d && possible.contains(d) {
                return Some(p);
            }
        }
    }
    None
}
