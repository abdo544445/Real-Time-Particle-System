use nalgebra::Vector2;

pub struct Particle {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub color: u32,
}

impl Particle {
    pub fn new(x: f32, y: f32, mass: f32, radius: f32, color: u32) -> Self {
        Self {
            position: Vector2::new(x, y),
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            mass,
            radius,
            color,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Simple Euler integration for now
        // We'll upgrade to Runge-Kutta later
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
        
        // Reset acceleration for next frame
        self.acceleration = Vector2::new(0.0, 0.0);
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        // F = ma, so a = F/m
        self.acceleration += force / self.mass;
    }
} 