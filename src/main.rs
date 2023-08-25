extern crate piston_window;
use piston_window::*;
use piston_window::types::Color;

mod constants;

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

    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, graphics, device| {
            // update screen
            clear([0.2, 0.2, 0.2, 1.0], graphics);
        });
    }
}
