use nalgebra::Vector2;

// Force trait for implementing different types of forces
pub trait Force {
    fn apply(&self, position: &Vector2<f32>, velocity: &Vector2<f32>, mass: f32) -> Vector2<f32>;
}

// Gravity implementation
pub struct Gravity {
    pub strength: f32,
    pub direction: Vector2<f32>,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            strength: 9.8, // Default to Earth gravity
            direction: Vector2::new(0.0, 1.0), // Downward
        }
    }
}

impl Force for Gravity {
    fn apply(&self, _position: &Vector2<f32>, _velocity: &Vector2<f32>, mass: f32) -> Vector2<f32> {
        self.direction * self.strength * mass
    }
}

// Wind implementation
pub struct Wind {
    pub strength: f32,
    pub direction: Vector2<f32>,
}

impl Force for Wind {
    fn apply(&self, _position: &Vector2<f32>, _velocity: &Vector2<f32>, _mass: f32) -> Vector2<f32> {
        self.direction * self.strength
    }
} 