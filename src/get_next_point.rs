#![allow(clippy::option_if_let_else)]

use crate::direction::Direction;
use crate::directions_possible::DirectionsPossible;
use crate::point::Point;

mod previous;

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
    let previous = previous::generate(snake);

    let direction = if let Some(direction) = possible_desired.single() {
        Some(direction)
    } else if let Some(direction) = previous::repeat(&previous, &possible) {
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
