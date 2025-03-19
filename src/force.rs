use nalgebra::Vector2;

// Force trait for implementing different types of forces
pub trait Force {
    fn apply(&self, position: &Vector2<f32>, velocity: &Vector2<f32>, mass: f32, mouse_pos: Option<&Vector2<f32>>) -> Vector2<f32>;
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
    fn apply(&self, _position: &Vector2<f32>, _velocity: &Vector2<f32>, mass: f32, _mouse_pos: Option<&Vector2<f32>>) -> Vector2<f32> {
        self.direction * self.strength * mass
    }
}

// Wind implementation
pub struct Wind {
    pub strength: f32,
    pub direction: Vector2<f32>,
}

impl Force for Wind {
    fn apply(&self, _position: &Vector2<f32>, _velocity: &Vector2<f32>, _mass: f32, _mouse_pos: Option<&Vector2<f32>>) -> Vector2<f32> {
        self.direction * self.strength
    }
}

// Mouse-based gravity implementation
pub struct MouseGravity {
    pub strength: f32,
}

impl Default for MouseGravity {
    fn default() -> Self {
        Self {
            strength: 5000.0, // Much stronger to make the effect more noticeable
        }
    }
}

impl Force for MouseGravity {
    fn apply(&self, position: &Vector2<f32>, _velocity: &Vector2<f32>, mass: f32, mouse_pos: Option<&Vector2<f32>>) -> Vector2<f32> {
        if let Some(mouse) = mouse_pos {
            let diff = mouse - position;
            let distance_squared = diff.norm_squared();
            
            // Avoid divide by zero and extreme forces when too close
            if distance_squared < 1.0 {
                return Vector2::new(0.0, 0.0);
            }
            
            // Inverse square law (like real gravity)
            let force_magnitude = self.strength * mass / (distance_squared.sqrt());
            let direction = diff.normalize();
            
            direction * force_magnitude
        } else {
            Vector2::new(0.0, 0.0) // No force if mouse position not available
        }
    }
} 