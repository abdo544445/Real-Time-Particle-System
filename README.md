# Real-Time Particle System

A high-performance, real-time particle simulation system implemented in Rust. This system simulates thousands of particles influenced by various forces including gravity, inter-particle attraction/repulsion, and mouse-based gravitational attraction.

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
  - Gravitational forces (regular and mouse-controlled)
  - Spring-like inter-particle attraction/repulsion
  - Accurate collision detection and resolution
  - Boundary collision handling with restitution (bounciness)

- **Interactive Control**
  - Control gravity center using mouse position
  - Watch particles dynamically respond to your cursor movements

- **Efficient Implementation**
  - Fast rendering using minifb
  - Vector math using nalgebra
  - Optimized collision detection algorithm

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
   ```
   git clone https://github.com/abdo544445/Real-Time-Particle-System.git
   cd Real-Time-Particle-System
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the application:
   ```
   cargo run --release
   ```

## Usage

Once running, the application will display a window containing particles in motion. The particles are influenced by:

1. A mild downward gravity
2. A strong attractive force toward your mouse cursor
3. Inter-particle forces that cause particles to maintain certain distances from each other
4. Collision forces when particles collide with each other or the window boundaries

### Controls

- **Mouse Movement**: Move the mouse to change the center of gravitational attraction
- **ESC Key**: Close the application

## Physics Details

### Forces

- **Gravity**: A constant downward force scaled by particle mass
- **Mouse Gravity**: An inverse square law attraction toward the mouse position
- **Inter-particle Forces**: Spring-like forces that maintain an equilibrium distance between particles

### Collision Resolution

- **Impulse-Based**: Collisions are resolved using an impulse-based approach that conserves momentum
- **Position Correction**: Small position adjustments prevent particles from sinking into each other
- **Restitution**: Collisions have a bounciness factor (restitution) of 0.8

## Future Plans

- Spatial partitioning with quadtrees for improved collision detection performance
- GPU-accelerated rendering using wgpu
- User interface for adjusting simulation parameters using egui
- Runge-Kutta integration for more accurate physics
- SIMD and parallel processing optimizations

## Contribution

Contributions are welcome! Please feel free to submit a Pull Request. 