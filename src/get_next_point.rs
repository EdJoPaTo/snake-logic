use crate::point::Point;

#[must_use]
pub fn get_next_point(snake: &[Point], food: &Point, height: u8, width: u8) -> Option<Point> {
    let head = &snake[0];
    // Directions wont leave the area. Saturating sub prevents < 0, min() prevents > width/height
    let left = Point::new(head.x.saturating_sub(1), head.y);
    let right = Point::new(head.x.saturating_add(1).min(width - 1), head.y);
    let up = Point::new(head.x, head.y.saturating_sub(1));
    let down = Point::new(head.x, head.y.saturating_add(1).min(height - 1));

    #[allow(clippy::if_not_else, clippy::collapsible_else_if)]
    if head.x > food.x {
        if !snake.contains(&left) {
            Some(left)
        } else if !snake.contains(&up) {
            Some(up)
        } else if !snake.contains(&down) {
            Some(down)
        } else if !snake.contains(&right) {
            Some(right)
        } else {
            None
        }
    } else if head.x < food.x {
        if !snake.contains(&right) {
            Some(right)
        } else if !snake.contains(&down) {
            Some(down)
        } else if !snake.contains(&up) {
            Some(up)
        } else if !snake.contains(&left) {
            Some(left)
        } else {
            None
        }
    } else if head.y > food.y {
        if !snake.contains(&up) {
            Some(up)
        } else if !snake.contains(&left) {
            Some(left)
        } else if !snake.contains(&right) {
            Some(right)
        } else if !snake.contains(&down) {
            Some(down)
        } else {
            None
        }
    } else {
        if !snake.contains(&down) {
            Some(down)
        } else if !snake.contains(&right) {
            Some(right)
        } else if !snake.contains(&left) {
            Some(left)
        } else if !snake.contains(&up) {
            Some(up)
        } else {
            None
        }
    }
}
