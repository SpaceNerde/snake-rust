extern crate piston_window;
use piston_window::*;
use piston_window::types::Color;

mod constants;

struct Snake {
    body: Vec<(i32, i32)>,
}

impl Snake {
    // creates a new snake
    pub fn spawn() -> Self{
        Snake {
            body: vec![(8, 8)],
        }
    }

    //expands the snake at the give coordination's
    pub fn expand_snake(&mut self, x: i32, y: i32) {
        self.body.push((x, y));
    }

    // draws the snake onto the window
    pub fn draw_snake(&self, context: Context, graphics: &mut G2d) {
        for part in &self.body {
            draw_rect(
              color::GREEN,
                part.0,
                part.1,
                context,
                graphics
            );
        }
    }
}

fn draw_rect(color: Color, x: i32, y: i32, context: Context, graphics: &mut G2d) {
    rectangle(
        color,
        [((x as f64) * constants::GRID_BLOCK_SIZE - 32.0), ((y as f64) * constants::GRID_BLOCK_SIZE - 32.0), constants::GRID_BLOCK_SIZE, constants::GRID_BLOCK_SIZE],
        context.transform,
        graphics
    );
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust Snake!", (constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut snake = Snake::spawn();
    snake.expand_snake(7, 8);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, graphics, device| {
            // update screen
            clear([0.2, 0.2, 0.2, 1.0], graphics);

            snake.draw_snake(context, graphics);
        });
    }
}
