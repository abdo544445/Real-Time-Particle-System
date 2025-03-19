use nalgebra::Vector2;

pub struct Particle {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub color: u32,
}

// State derivative for RK4 integration
struct StateDerivative {
    dp: Vector2<f32>, // derivative of position
    dv: Vector2<f32>, // derivative of velocity
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
        // RK4 integration
        let k1 = self.evaluate(0.0, &StateDerivative {
            dp: Vector2::new(0.0, 0.0),
            dv: Vector2::new(0.0, 0.0),
        });
        
        let k2 = self.evaluate(dt * 0.5, &k1);
        let k3 = self.evaluate(dt * 0.5, &k2);
        let k4 = self.evaluate(dt, &k3);

        // Combine all derivatives with proper weights
        let dp_dt = (k1.dp + (k2.dp + k3.dp) * 2.0 + k4.dp) * (1.0 / 6.0);
        let dv_dt = (k1.dv + (k2.dv + k3.dv) * 2.0 + k4.dv) * (1.0 / 6.0);

        // Update position and velocity
        self.position += dp_dt * dt;
        self.velocity += dv_dt * dt;
        
        // Reset acceleration for next frame
        self.acceleration = Vector2::new(0.0, 0.0);
    }

    // Evaluate derivatives at a given time offset and with a given state derivative
    fn evaluate(&self, dt: f32, derivative: &StateDerivative) -> StateDerivative {
        // Calculate intermediate position
        let position = self.position + derivative.dp * dt;
        
        // Calculate intermediate velocity
        let velocity = self.velocity + derivative.dv * dt;
        
        StateDerivative {
            dp: velocity,                // velocity is derivative of position
            dv: self.acceleration,       // acceleration is derivative of velocity
        }
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        // F = ma, so a = F/m
        self.acceleration += force / self.mass;
    }
} 