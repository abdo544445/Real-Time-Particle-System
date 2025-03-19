use nalgebra::Vector2;

pub struct Particle {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub color: u32,
    damping: f32,         // Velocity damping coefficient
    restitution: f32,     // Collision elasticity
}

// State for physics calculations
#[derive(Clone, Copy)]
pub struct State {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
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
            damping: 0.98,      // Slight damping to simulate air resistance
            restitution: 0.8,   // Fairly elastic collisions
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Apply damping to velocity
        self.velocity *= self.damping;

        // RK4 integration with improved state handling
        let initial_state = State {
            position: self.position,
            velocity: self.velocity,
        };

        let k1 = self.evaluate(0.0, &initial_state, &StateDerivative {
            dp: Vector2::new(0.0, 0.0),
            dv: Vector2::new(0.0, 0.0),
        });
        
        let k2 = self.evaluate(
            dt * 0.5,
            &self.state_at(dt * 0.5, &initial_state, &k1),
            &k1
        );
        
        let k3 = self.evaluate(
            dt * 0.5,
            &self.state_at(dt * 0.5, &initial_state, &k2),
            &k2
        );
        
        let k4 = self.evaluate(
            dt,
            &self.state_at(dt, &initial_state, &k3),
            &k3
        );

        // Combine derivatives with proper weights
        let dp_dt = (k1.dp + (k2.dp + k3.dp) * 2.0 + k4.dp) * (1.0 / 6.0);
        let dv_dt = (k1.dv + (k2.dv + k3.dv) * 2.0 + k4.dv) * (1.0 / 6.0);

        // Update position and velocity
        self.position += dp_dt * dt;
        self.velocity += dv_dt * dt;
        
        // Enforce maximum velocity to prevent instability
        let max_velocity = 1000.0;
        if self.velocity.norm() > max_velocity {
            self.velocity = self.velocity.normalize() * max_velocity;
        }

        // Reset acceleration for next frame
        self.acceleration = Vector2::new(0.0, 0.0);
    }

    // Calculate state at a given time offset
    fn state_at(&self, dt: f32, initial: &State, derivative: &StateDerivative) -> State {
        State {
            position: initial.position + derivative.dp * dt,
            velocity: initial.velocity + derivative.dv * dt,
        }
    }

    // Evaluate derivatives at a given time offset and with a given state
    fn evaluate(&self, dt: f32, state: &State, derivative: &StateDerivative) -> StateDerivative {
        let position = state.position + derivative.dp * dt;
        let velocity = state.velocity + derivative.dv * dt;
        
        StateDerivative {
            dp: velocity,
            dv: self.acceleration,
        }
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        // Apply force with improved numerical stability
        let force = force.map(|x| x.min(1000.0).max(-1000.0)); // Limit force magnitude
        self.acceleration += force / self.mass;
    }

    // Get kinetic energy of the particle
    pub fn kinetic_energy(&self) -> f32 {
        0.5 * self.mass * self.velocity.norm_squared()
    }

    // Handle collision with another particle
    pub fn handle_collision(&mut self, other: &mut Particle) {
        let diff = self.position - other.position;
        let distance = diff.norm();
        let min_distance = self.radius + other.radius;

        if distance < min_distance && distance > 0.0 {
            let normal = diff / distance;
            
            // Relative velocity
            let rel_velocity = self.velocity - other.velocity;
            let vel_along_normal = rel_velocity.dot(&normal);
            
            // Don't resolve if particles are moving apart
            if vel_along_normal > 0.0 {
                return;
            }

            // Calculate impulse
            let restitution = self.restitution.min(other.restitution);
            let impulse_scalar = -(1.0 + restitution) * vel_along_normal /
                               (1.0/self.mass + 1.0/other.mass);
            let impulse = normal * impulse_scalar;

            // Apply impulse
            self.velocity += impulse / self.mass;
            other.velocity -= impulse / other.mass;

            // Positional correction to prevent sinking
            let percent = 0.2; // Penetration percentage to correct
            let correction = normal * percent * (min_distance - distance) /
                           (1.0/self.mass + 1.0/other.mass);
            self.position += correction / self.mass;
            other.position -= correction / other.mass;
        }
    }

    // Handle collision with boundaries
    pub fn handle_boundary_collision(&mut self, width: f32, height: f32) {
        // X-axis boundaries
        if self.position.x - self.radius < 0.0 {
            self.position.x = self.radius;
            self.velocity.x = -self.velocity.x * self.restitution;
        } else if self.position.x + self.radius > width {
            self.position.x = width - self.radius;
            self.velocity.x = -self.velocity.x * self.restitution;
        }

        // Y-axis boundaries
        if self.position.y - self.radius < 0.0 {
            self.position.y = self.radius;
            self.velocity.y = -self.velocity.y * self.restitution;
        } else if self.position.y + self.radius > height {
            self.position.y = height - self.radius;
            self.velocity.y = -self.velocity.y * self.restitution;
        }
    }
} 