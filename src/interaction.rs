use crate::particle::Particle;
use nalgebra::Vector2;

const EQUILIBRIUM_DISTANCE: f32 = 50.0;
const INTERACTION_COEFFICIENT: f32 = 0.05;

/// Applies a spring-like attraction/repulsion force between each unique pair of particles.
/// Particles separated by more than EQUILIBRIUM_DISTANCE experience an attractive force,
/// while those closer than that distance are repelled.
pub fn apply_attraction_repulsion(particles: &mut [Particle]) {
    let len = particles.len();
    for i in 0..len {
        for j in (i+1)..len {
            let (p_i, p_j) = {
                let (left, right) = particles.split_at_mut(j);
                (&mut left[i], &mut right[0])
            };
            
            let diff = p_j.position - p_i.position;
            let distance = diff.norm();
            if distance < 0.0001 {
                continue; // Prevent division by zero
            }
            
            let direction = diff / distance;
            // Linear (spring-like) force based on deviation from EQUILIBRIUM_DISTANCE:
            let force_magnitude = INTERACTION_COEFFICIENT * (distance - EQUILIBRIUM_DISTANCE);
            let force = force_magnitude * direction;
            
            // Apply equal and opposite forces to each particle:
            p_i.apply_force(force);
            p_j.apply_force(-force);
        }
    }
} 