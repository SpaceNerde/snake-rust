extern crate piston_window;
use piston_window::*;
use piston_window::Button::Keyboard;
use piston_window::Key::*;
use piston_window::types::Color;

mod constants;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

struct Snake {
    moving_direction: Direction,
    body: Vec<(i32, i32)>,
}

impl Snake {
    // creates a new snake
    pub fn spawn() -> Self{
        Snake {
            moving_direction: Direction::Right,
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

    pub fn head_position(&self) -> (i32, i32){
        let head = self.body[0];
        (head.0, head.1)
    }

    pub fn move_towards(&mut self) {
        let (last_x, last_y) = self.head_position();

        let new_pos = match self.moving_direction {
            Direction::Up => (last_x, last_y - 1),
            Direction::Down => (last_x, last_y + 1),
            Direction::Left => (last_x - 1, last_y),
            Direction::Right => (last_x + 1, last_y),
        };

        self.body.insert(0, new_pos);
        self.body.remove(self.body.len() - 1);
    }
}

fn input_handler(snake: &mut Snake, key: Key)  {
    let input = match key {
        Key::Up => Some(Direction::Up),
        Key::Down => Some(Direction::Down),
        Key::Left => Some(Direction::Left),
        Key::Right => Some(Direction::Right),
        _ => return,
    };

    if input.unwrap() == snake.moving_direction.opposite() {
        return;
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
    let mut window_settings = WindowSettings::new("Rust Snake!", (constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT))
        .exit_on_esc(true);

    window_settings.set_vsync(true);

    let mut window: PistonWindow = window_settings
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    window.set_max_fps(4u64);

    let mut snake = Snake::spawn();
    snake.expand_snake(7, 8);

    while let Some(e) = window.next() {

        if let Some(Button::Keyboard(keyboard)) = e.press_args() {
            input_handler(&mut snake, keyboard);
        }

        window.draw_2d(&e, |context, graphics, device| {
            // update screen
            clear([0.2, 0.2, 0.2, 1.0], graphics);


            snake.draw_snake(context, graphics);
            snake.move_towards();
        });
    }
}
