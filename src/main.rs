extern crate piston_window;
use piston_window::*;
use piston_window::Key::*;
use piston_window::types::Color;

use rand::*;

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

struct Food {
    max_amount: i32,
    count: i32,
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

    pub fn const_movement(&mut self) {
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

    pub fn move_towards(&mut self, input: Option<Direction>) {
        match input{
            Some(d) => self.moving_direction = d,
            None => {}
        }

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

impl Food {
    pub fn food() -> Self {
        Food {
            max_amount: 5,
            count: 0,
        }
    }

    pub fn spawn_food(&mut self) -> (Vec<i32>, Vec<i32>){

        let mut x: Vec<i32> = vec![];
        let mut y: Vec<i32> = vec![];


        if self.count < self.max_amount {
            for _ in 0..self.max_amount {
                self.count += 1;

                x.push( rand::thread_rng().gen_range(0..16));
                y.push( rand::thread_rng().gen_range(0..16));
            }
        }

        (x, y)
    }


}

fn input_handler(snake: &mut Snake, key: Key) {

    // compare input with moving direction
    let input = match key {
        Up => Some(Direction::Up),
        Down => Some(Direction::Down),
        Left => Some(Direction::Left),
        Right => Some(Direction::Right),
        _ => return,
    };

    // check if snake wants to move into it self
    if input.unwrap() == snake.moving_direction.opposite() {
        return;
    }

    snake.move_towards(input);
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
    let mut food = Food::food();

    snake.expand_snake(7, 8);
    snake.expand_snake(6, 8);

    while let Some(e) = window.next() {

        if let Some(Button::Keyboard(keyboard)) = e.press_args() {
            input_handler(&mut snake, keyboard);
        }


        window.draw_2d(&e, |context, graphics, _device| {
            // update screen
            clear([0.2, 0.2, 0.2, 1.0], graphics);
            let (x, y) = food.spawn_food();
            for item_x in &x {
                for item_y in &y {

                    draw_rect(
                        color::RED,
                        *item_x,
                        *item_y,
                        context,
                        graphics,
                    )
                }
            }

            snake.draw_snake(context, graphics);
            snake.const_movement();
        });
    }
}
