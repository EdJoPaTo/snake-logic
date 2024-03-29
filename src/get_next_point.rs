use crate::direction::Direction;
use crate::directions_possible::DirectionsPossible;
use crate::point::Point;

/// Calculate a point towards the `food` for the given `snake` on a board with `width` and `height`.
///
/// # Panics
/// Panics when the `snake` has no head (needs len >= 1)
#[must_use]
pub fn get_next_point(width: u8, height: u8, snake: &[Point], food: Point) -> Option<Point> {
    let head = snake.first().expect("Snake has to have a head");

    // Directions wont leave the area. Saturating sub prevents < 0, min() prevents > width/height
    let left = Point::new(head.x.saturating_sub(1), head.y);
    let right = Point::new(head.x.saturating_add(1).min(width - 1), head.y);
    let up = Point::new(head.x, head.y.saturating_sub(1));
    let down = Point::new(head.x, head.y.saturating_add(1).min(height - 1));

    let possible = DirectionsPossible::from_bools(
        !snake.contains(&left),
        !snake.contains(&right),
        !snake.contains(&up),
        !snake.contains(&down),
    );
    let desired = DirectionsPossible::from_bools(
        head.x > food.x,
        head.x < food.x,
        head.y > food.y,
        head.y < food.y,
    );

    let possible_desired = DirectionsPossible::intersection(&possible, &desired);

    let direction = possible_desired
        .single()
        .or_else(|| repeat_previous(snake, possible))
        .or_else(|| possible.any());

    #[cfg(feature = "debug")]
    {
        println!("{possible} possible");
        println!("{desired} desired");
        println!("{possible_desired} possible_desired");
        println!("decided for {direction:?}");
    }

    let point = match direction? {
        Direction::Left => left,
        Direction::Right => right,
        Direction::Up => up,
        Direction::Down => down,
    };
    Some(point)
}

fn repeat_previous(snake: &[Point], possible: DirectionsPossible) -> Option<Direction> {
    let mut iter = snake.iter();
    let mut headwards = iter.next().expect("Snake has to have a head");
    for behind in iter {
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
        if possible.contains(direction) {
            return Some(direction);
        }
        headwards = behind;
    }
    None
}
