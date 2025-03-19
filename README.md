# Real-Time Particle System

A high-performance, real-time particle simulation system implemented in Rust. This system simulates thousands of particles influenced by various forces including gravity, inter-particle attraction/repulsion, and mouse-based gravitational attraction. The simulation uses advanced numerical methods and physics calculations to ensure accurate and stable behavior.

![Particle System](/image.png)

## Notice

This project is available at: https://github.com/abdo544445/Real-Time-Particle-System.git

For Linux users, you may need to set the following environment variable before running the application:
```bash
export WINIT_UNIX_BACKEND=x11
```

This is required for proper window management on some Linux systems.

## Features

- **Advanced Physics Simulation**
  - 4th order Runge-Kutta (RK4) integration for accurate trajectories
  - Gravitational forces (regular and mouse-controlled)
  - Spring-like inter-particle attraction/repulsion
  - Accurate collision detection and resolution with impulse-based physics
  - Boundary collision handling with restitution (bounciness)
  - Energy conservation and stability controls

- **Interactive Control**
  - Control gravity center using mouse position
  - Watch particles dynamically respond to your cursor movements
  - Real-time physics interaction

- **Efficient Implementation**
  - Fast rendering using minifb
  - Vector math using nalgebra
  - Optimized collision detection algorithm
  - Numerical stability safeguards
  - Adaptive time-stepping

## Technical Details

### Physics Implementation

1. **Integration Method**
   - 4th order Runge-Kutta (RK4) integration
   - Adaptive time-stepping with maximum dt of 1/30 second
   - State-space formulation for accurate derivatives

2. **Force System**
   - Gravitational force with inverse square law
   - Mouse-based attraction force
   - Inter-particle forces with equilibrium distance
   - Air resistance through velocity damping

3. **Collision System**
   - Impulse-based collision resolution
   - Penetration correction to prevent sinking
   - Configurable restitution coefficients
   - Separate handling for particle-particle and particle-boundary collisions

4. **Stability Features**
   - Velocity capping to prevent extreme speeds
   - Force magnitude limiting
   - Continuous collision detection
   - Energy conservation monitoring

## Requirements

- Rust 1.67+
- Cargo package manager

## Dependencies

- minifb 0.24: Window and pixel buffer rendering
- nalgebra 0.32: Vector and math operations
- rand 0.8: Random number generation
- rayon 1.7: Parallel computation (for future optimizations)

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/abdo544445/Real-Time-Particle-System.git
   cd Real-Time-Particle-System
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

## Usage

Once running, the application will display a window containing particles in motion. The particles are influenced by:

1. A mild downward gravity (configurable strength)
2. A strong attractive force toward your mouse cursor (inverse square law)
3. Inter-particle forces that maintain equilibrium distances
4. Collision forces with proper momentum and energy conservation

### Controls

- **Mouse Movement**: Move the mouse to change the center of gravitational attraction
- **ESC Key**: Close the application

## Physics Parameters

You can modify these parameters in the code to adjust the simulation behavior:

- **Particle Properties**
  - Mass Range: 0.5 to 2.0 units
  - Radius: Proportional to mass (2x mass)
  - Damping: 0.98 (air resistance)
  - Restitution: 0.8 (collision elasticity)

- **Force Properties**
  - Regular Gravity: 2.0 units downward
  - Mouse Gravity: 5000.0 units (inverse square law)
  - Inter-particle Equilibrium Distance: 50.0 units
  - Maximum Force: 1000.0 units

## Future Plans

- Spatial partitioning with quadtrees for improved collision detection performance
- GPU-accelerated rendering using wgpu
- User interface for adjusting simulation parameters using egui
- Additional physics features:
  - Rotational motion and torque
  - Fluid dynamics effects
  - Constraint-based physics
- SIMD and parallel processing optimizations

## Contribution

Contributions are welcome! Please feel free to submit a Pull Request. 