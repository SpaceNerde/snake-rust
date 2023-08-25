extern crate piston_window;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust Snake!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    while let Some(e) = window.next() {}
}
