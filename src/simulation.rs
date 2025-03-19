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
        for _ in 0..100 {
            let x = rng.gen_range(0.0..width as f32);
            let y = rng.gen_range(0.0..height as f32);
            let mass = rng.gen_range(1.0..5.0);
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
        forces.push(Box::new(Gravity::default()));
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

        // 1. Apply external forces (e.g., Gravity, Wind)
        for particle in &mut self.particles {
            for force in &self.forces {
                let force_vector = force.apply(&particle.position, &particle.velocity, particle.mass, Some(&mouse_pos));
                particle.apply_force(force_vector);
            }
        }

        // 2. Apply inter-particle attraction/repulsion forces
        crate::interaction::apply_attraction_repulsion(&mut self.particles);

        // 3. Update particle positions using Euler integration
        for particle in &mut self.particles {
            particle.update(dt);
        }

        // 4. Handle boundary collisions
        let sim_width = self.width as f32;
        let sim_height = self.height as f32;
        for particle in &mut self.particles {
            handle_boundary_collision_particle(sim_width, sim_height, particle);
        }

        // 5. Handle particle collisions (particle-particle collisions)
        self.handle_particle_collisions();
    }
    
    fn handle_particle_collisions(&mut self) {
        let restitution = 0.8;
        let len = self.particles.len();
        for i in 0..len {
            for j in (i+1)..len {
                // Safely split two mutable references
                let (p1, p2) = {
                    let (left, right) = self.particles.split_at_mut(j);
                    (&mut left[i], &mut right[0])
                };

                let diff = p2.position - p1.position;
                let distance = diff.norm();
                let min_distance = p1.radius + p2.radius;
                if distance < min_distance && distance > 0.0 {
                    let normal = diff / distance;

                    // Relative velocity along the collision normal
                    let rel_vel = p2.velocity - p1.velocity;
                    let vel_along_normal = rel_vel.dot(&normal);
                    if vel_along_normal > 0.0 {
                        continue;
                    }

                    let impulse_scalar = -(1.0 + restitution) * vel_along_normal / (1.0/p1.mass + 1.0/p2.mass);
                    let impulse = impulse_scalar * normal;
                    p1.velocity -= impulse / p1.mass;
                    p2.velocity += impulse / p2.mass;

                    // Positional correction to reduce sinking
                    let percent = 0.2; // 20% correction
                    let correction = normal * percent * (min_distance - distance) / (1.0/p1.mass + 1.0/p2.mass);
                    p1.position -= correction / p1.mass;
                    p2.position += correction / p2.mass;
                }
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