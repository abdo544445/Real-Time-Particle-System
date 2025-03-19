use crate::simulation::Simulation;
use nalgebra::Vector2;

pub struct Renderer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }
    
    pub fn render(&mut self, simulation: &Simulation) -> Vec<u32> {
        // Clear buffer
        self.buffer.fill(0);
        
        // Draw particles
        for particle in &simulation.particles {
            self.draw_particle(particle.position, particle.radius, particle.color);
        }
        
        self.buffer.clone()
    }
    
    fn draw_particle(&mut self, position: Vector2<f32>, radius: f32, color: u32) {
        let x_start = (position.x - radius).max(0.0) as usize;
        let y_start = (position.y - radius).max(0.0) as usize;
        let x_end = (position.x + radius).min(self.width as f32 - 1.0) as usize;
        let y_end = (position.y + radius).min(self.height as f32 - 1.0) as usize;
        
        for y in y_start..=y_end {
            for x in x_start..=x_end {
                let dx = x as f32 - position.x;
                let dy = y as f32 - position.y;
                if dx*dx + dy*dy <= radius*radius {
                    let idx = y * self.width + x;
                    if idx < self.buffer.len() {
                        self.buffer[idx] = color;
                    }
                }
            }
        }
    }
} 