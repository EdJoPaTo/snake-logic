// run as: cargo run --example=println

use std::thread::sleep;
use std::time::Duration;

use snake_logic::{get_next_point, Point};

fn print_field(width: u8, height: u8, snake: &[Point], food: &Point) {
    print!("   ");
    for x in 0..width {
        print!("{:1}", x % 10);
    }
    println!();

    for y in 0..height {
        print!("{:>3}", y);
        let mut line = String::with_capacity(width.into());
        for x in 0..width {
            let p = Point::new(x, y);
            if &p == food {
                line += "F";
            } else if p == snake[0] {
                line += "H";
            } else if snake.contains(&p) {
                line += "x";
            } else {
                line += " ";
            }
        }
        println!("{}", line);
    }
}

fn main() {
    let width = 16;
    let height = 16;
    snake(width, height);
}

fn snake(width: u8, height: u8) {
    let maximal_length_possible = u16::from(width) * u16::from(height);
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
            print_field(width, height, &snake, &food);

            let next_point = if let Some(point) = get_next_point(&snake, &food, height, width) {
                // Hits itself
                if snake.contains(&point) {
                    eprintln!(
                        "snake hit itself at length {:3}/{}",
                        snake.len(),
                        maximal_length_possible
                    );
                    return;
                }

                point
            } else {
                eprintln!(
                    "snake has nowhere to go {:3}/{}",
                    snake.len(),
                    maximal_length_possible
                );
                return;
            };

            println!(
                "snake length {:3}/{:5} goes to {:3} {:3}  food is at {:3} {:3}",
                snake.len(),
                maximal_length_possible,
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
            sleep(Duration::from_millis(200));
        }
    }
}
