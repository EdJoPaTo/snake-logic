// run as: cargo run --example=println

use std::thread::sleep;
use std::time::Duration;

use snake_logic::{Point, get_next_point};

const RUN_SLEEP: Duration = Duration::from_millis(200);

fn main() {
    let width = 16;
    let height = 16;
    loop {
        let mut food = Point::random(width, height);
        let mut snake = {
            let start = Point::new(width / 2, height / 2);
            let end = {
                let x = if start.x < food.x {
                    start.x - 1
                } else {
                    start.x + 1
                };
                Point::new(x, start.y)
            };
            vec![start, end]
        };

        loop {
            let next_point = if let Some(point) = get_next_point(&snake, &food, height, width) {
                // Hits itself
                if snake.contains(&point) {
                    panic!("snake hit itself");
                }

                point
            } else {
                panic!("snake has nowhere to go");
            };

            println!(
                "snake length {:3} goes to {:3} {:3}  food is at {:3} {:3}",
                snake.len(),
                next_point.x,
                next_point.y,
                food.x,
                food.y
            );

            if next_point == food {
                food = Point::random(width, height);
            } else {
                snake.pop().unwrap();
            }

            snake.insert(0, next_point);
            sleep(RUN_SLEEP);
        }
    }
}
