use crate::particle::Particle;
use crate::force::{Force, Gravity, Wind, MouseGravity};
use nalgebra::Vector2;
use rand::Rng;
use std::time::Instant;

pub struct Simulation {
    pub particles: Vec<Particle>,
    width: usize,
    height: usize,
    forces: Vec<Box<dyn Force>>,
    last_update: Instant,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::new();
        
        // Create some initial particles
        for _ in 0..50 {  // Reduced number of particles for better visibility
            let x = rng.gen_range(0.0..width as f32);
            let y = rng.gen_range(0.0..height as f32);
            let mass = rng.gen_range(0.5..2.0);  // Lighter particles
            let radius = mass * 2.0;
            
            // Create random color
            let r = rng.gen_range(0..255);
            let g = rng.gen_range(0..255);
            let b = rng.gen_range(0..255);
            let color = (r << 16) | (g << 8) | b;
            
            particles.push(Particle::new(x, y, mass, radius, color));
        }
        
        // Set up forces
        let mut forces: Vec<Box<dyn Force>> = Vec::new();
        // Reduced regular gravity
        forces.push(Box::new(Gravity {
            strength: 2.0,  // Much weaker regular gravity
            direction: Vector2::new(0.0, 1.0),
        }));
        forces.push(Box::new(MouseGravity::default()));
        
        Self {
            particles,
            width,
            height,
            forces,
            last_update: Instant::now(),
        }
    }
    
    pub fn update(&mut self, mouse_pos: Vector2<f32>) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Limit dt to prevent instability with large time steps
        let dt = dt.min(1.0 / 30.0);

        // 1. Apply external forces (e.g., Gravity, Wind)
        for particle in &mut self.particles {
            for force in &self.forces {
                let force_vector = force.apply(&particle.position, &particle.velocity, particle.mass, Some(&mouse_pos));
                particle.apply_force(force_vector);
            }
        }

        // 2. Apply inter-particle attraction/repulsion forces
        crate::interaction::apply_attraction_repulsion(&mut self.particles);

        // 3. Update particle positions using RK4 integration
        for particle in &mut self.particles {
            particle.update(dt);
        }

        // 4. Handle boundary collisions
        let sim_width = self.width as f32;
        let sim_height = self.height as f32;
        for particle in &mut self.particles {
            particle.handle_boundary_collision(sim_width, sim_height);
        }

        // 5. Handle particle-particle collisions
        let len = self.particles.len();
        for i in 0..len {
            for j in (i+1)..len {
                let (particles_a, particles_b) = self.particles.split_at_mut(j);
                let particle1 = &mut particles_a[i];
                let particle2 = &mut particles_b[0];
                particle1.handle_collision(particle2);
            }
        }
    }
}

// Private helper function for boundary collision handling.
fn handle_boundary_collision_particle(width: f32, height: f32, particle: &mut Particle) {
    let restitution = 0.8; // Bounciness factor
    
    // Left and right boundaries
    if particle.position.x - particle.radius < 0.0 {
        particle.position.x = particle.radius;
        particle.velocity.x = -particle.velocity.x * restitution;
    } else if particle.position.x + particle.radius > width {
        particle.position.x = width - particle.radius;
        particle.velocity.x = -particle.velocity.x * restitution;
    }
    
    // Top and bottom boundaries
    if particle.position.y - particle.radius < 0.0 {
        particle.position.y = particle.radius;
        particle.velocity.y = -particle.velocity.y * restitution;
    } else if particle.position.y + particle.radius > height {
        particle.position.y = height - particle.radius;
        particle.velocity.y = -particle.velocity.y * restitution;
    }
} 