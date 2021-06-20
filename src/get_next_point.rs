#![allow(clippy::option_if_let_else)]

use crate::direction::{Direction, ALL_DIRECTIONS};
use crate::directions_possible::DirectionsPossible;
use crate::point::Point;

fn generate_previous(snake: &[Point]) -> Vec<Direction> {
    let mut result = Vec::new();
    if snake.len() < 2 {
        return result;
    }

    let mut iter = snake.iter();
    let mut last = iter.next().unwrap();
    for p in iter {
        result.push(if p.x == last.x {
            if p.y > last.y {
                Direction::Up
            } else {
                Direction::Down
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if p.x > last.x {
                Direction::Right
            } else {
                Direction::Left
            }
        });

        last = p;
    }
    result
}

fn repeat_previous<'a>(
    previous: &'a [Direction],
    possible: &DirectionsPossible,
) -> Option<&'a Direction> {
    for p in previous {
        for d in ALL_DIRECTIONS {
            if p == &d && possible.contains(&d) {
                return Some(p);
            }
        }
    }
    None
}

#[must_use]
pub fn get_next_point(snake: &[Point], food: &Point, height: u8, width: u8) -> Option<Point> {
    let head = &snake[0];
    println!("head: {:?} food: {:?}", head, food);

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

    let priority = DirectionsPossible::from_bools(
        head.y == food.y && head.x > food.x, // || prio && start.x > food.x,
        head.y == food.y && head.x < food.x, // || prio && start.x < food.x,
        head.x == food.x && head.y > food.y,
        head.x == food.x && head.y < food.y,
    );

    //prio = false;

    let possible_desired = DirectionsPossible::intersection(&possible, &desired);
    let possible_desired_priority = DirectionsPossible::intersection(&possible_desired, &priority);

    #[cfg(debug_assertions)]
    {
        println!("{} possible", possible);
        println!("{} desired", desired);
        println!("{} priority", priority);
        println!("{} possible_desired", possible_desired);
        println!("{} possible_desired_priority", possible_desired_priority);
    }

    let previous = generate_previous(snake);
    let direction = if let Some(direction) = possible_desired_priority.any() {
        Some(direction)
    } else if let Some(direction) = possible_desired.any() {
        Some(direction)
    } else if let Some(direction) = repeat_previous(&previous, &possible) {
        Some(direction)
    } else {
        possible.any()
    };

    if let Some(direction) = direction {
        println!("decided for {:?}", direction);
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
