mod particle;
mod simulation;
mod renderer;
mod force;
mod interaction;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use simulation::Simulation;
use renderer::Renderer;
use nalgebra::Vector2;

// Constants
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const FPS_CAP: u64 = 60;

fn main() {
    // Create window
    let mut window = Window::new(
        "Particle System",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed: {}", e);
    });

    // Set up fps cap
    window.limit_update_rate(Some(std::time::Duration::from_micros(1_000_000 / FPS_CAP)));

    // Enable mouse tracking
    window.set_position_polling(true);

    // Initialize simulation and renderer
    let mut simulation = Simulation::new(WIDTH, HEIGHT);
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    // Track mouse position for gravity center
    let mut mouse_pos = Vector2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update mouse position if available
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            mouse_pos.x = x;
            mouse_pos.y = y;
        }

        // Update simulation
        simulation.update(mouse_pos);

        // Render
        let buffer = renderer.render(&simulation);
        
        // Display
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| {
                panic!("Window update failed: {}", e);
            });
    }
} 