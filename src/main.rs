extern crate piston_window;
use piston_window::*;

mod constants;

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
