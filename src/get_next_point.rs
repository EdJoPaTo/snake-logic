#![allow(clippy::option_if_let_else)]

use crate::direction::{Direction, ALL_DIRECTIONS};
use crate::directions_possible::DirectionsPossible;
use crate::point::Point;

/// Contains the previous chosen directions.
/// Contains up to the 4 directions in the order of last usage.
fn generate_previous(snake: &[Point]) -> Vec<Direction> {
    let mut result = Vec::new();
    if snake.len() < 2 {
        return result;
    }

    let mut iter = snake.iter();
    let mut headwards = iter.next().unwrap();
    for behind in iter {
        if result.len() >= 4 {
            // Already got all 4 directions -> done
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

fn repeat_previous<'a>(
    previous: &'a [Direction],
    possible: &DirectionsPossible,
) -> Option<&'a Direction> {
    for p in previous {
        for d in &ALL_DIRECTIONS {
            if p == d && possible.contains(d) {
                return Some(p);
            }
        }
    }
    None
}

#[must_use]
pub fn get_next_point(width: u8, height: u8, snake: &[Point], food: &Point) -> Option<Point> {
    let head = &snake[0];

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
    let previous = generate_previous(snake);

    let direction = if let Some(direction) = possible_desired.single() {
        Some(direction)
    } else if let Some(direction) = repeat_previous(&previous, &possible) {
        Some(direction)
    } else {
        possible.any()
    };

    #[cfg(debug_assertions)]
    {
        println!("{} possible", possible);
        println!("{} desired", desired);
        println!("{} possible_desired", possible_desired);
        println!("previous {:?}", previous);
        println!("decided for {:?}", direction);
    }

    if let Some(direction) = direction {
        let point = match direction {
            Direction::Left => left,
            Direction::Right => right,
            Direction::Up => up,
            Direction::Down => down,
        };
        Some(point)
    } else {
        None
    }
}
