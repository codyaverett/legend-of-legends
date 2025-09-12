# Technical Architecture - Legends of Legend

## Overview
A modular 2D game engine built with Rust and SDL2, designed as a "System of Systems" allowing plug-and-play game mechanics.

## Core Design Principles

1. **Modularity**: Each system is independent and communicates through defined interfaces
2. **Data-Oriented**: ECS architecture for performance and flexibility
3. **Type Safety**: Leverage Rust's type system for compile-time guarantees
4. **Hot Reloading**: Support asset reloading during development
5. **Cross-Platform**: Target Windows, macOS, and Linux from day one

## Architecture Layers

### 1. Platform Layer (SDL2 Abstraction)
Wraps SDL2 functionality in safe Rust interfaces.

```rust
platform/
├── window.rs       // Window creation and management
├── input.rs        // Keyboard, mouse, gamepad handling
├── audio.rs        // Sound system wrapper
├── renderer.rs     // OpenGL context and rendering
└── events.rs       // Event pump and distribution
```

### 2. Core Engine Layer
Fundamental systems that all games need.

```rust
engine/core/
├── ecs.rs          // Entity Component System
├── math.rs         // Vectors, matrices, transforms
├── time.rs         // Frame timing and delta time
├── resources.rs    // Asset loading and caching
└── scene.rs        // Scene graph and management
```

### 3. Rendering System
Handles all visual output.

```rust
engine/rendering/
├── sprite.rs       // Sprite rendering and batching
├── tilemap.rs      // Efficient tile rendering
├── particles.rs    // Particle effects system
├── camera.rs       // View transformations and zoom
├── layers.rs       // Rendering layer management
└── animation.rs    // Sprite animation system
```

### 4. Physics System
2D physics and collision detection.

```rust
engine/physics/
├── rigid_body.rs   // Physics bodies
├── collider.rs     // Collision shapes
├── spatial.rs      // Spatial partitioning (quadtree)
├── solver.rs       // Collision resolution
└── destruction.rs  // Voxel destruction physics
```

### 5. Game Systems
Game-specific mechanics built on top of the engine.

```rust
systems/
├── player/
│   ├── movement.rs     // On-foot movement
│   ├── mech.rs         // Titan mech control
│   └── transition.rs   // Mode switching
├── combat/
│   ├── weapons.rs      // Weapon systems
│   ├── damage.rs       // Damage calculation
│   └── projectiles.rs  // Bullet/missile physics
├── destruction/
│   ├── voxel.rs        // Voxel-based destruction
│   ├── debris.rs       // Debris spawning
│   └── collapse.rs     // Structural integrity
├── ai/
│   ├── behavior.rs     // AI behavior trees
│   ├── pathfinding.rs  // A* pathfinding
│   └── kaiju.rs        // Kaiju-specific AI
└── ui/
    ├── hud.rs          // In-game HUD
    ├── menu.rs         // Menu systems
    └── dialog.rs       // Dialog/story system
```

## Entity Component System (ECS)

### Entities
Simple unique identifiers (u32 or u64).

### Components
Plain data structures with no logic.

```rust
// Example components
struct Transform {
    position: Vec2,
    rotation: f32,
    scale: Vec2,
}

struct Sprite {
    texture_id: TextureId,
    source_rect: Rect,
    color: Color,
}

struct Health {
    current: i32,
    maximum: i32,
}

struct Destructible {
    voxel_grid: VoxelGrid,
    material: MaterialType,
}
```

### Systems
Functions that operate on entities with specific component combinations.

```rust
// Example system
fn movement_system(
    entities: Query<(&mut Transform, &Velocity)>,
    delta_time: f32,
) {
    for (mut transform, velocity) in entities.iter_mut() {
        transform.position += velocity.linear * delta_time;
        transform.rotation += velocity.angular * delta_time;
    }
}
```

## Rendering Pipeline

### Render Stages
1. **Clear**: Clear framebuffer
2. **Background**: Render parallax backgrounds
3. **Tilemap**: Render world tiles
4. **Entities**: Render sprites sorted by Y-position
5. **Effects**: Render particles and effects
6. **UI**: Render HUD and menus
7. **Present**: Swap buffers

### Camera System
- Smooth transitions between on-foot and mech views
- Screen shake for impacts
- Zoom levels:
  - On-foot: 1.0x zoom (close view)
  - Mech: 0.3x zoom (tactical view)
  - Map: 0.1x zoom (strategic view)

## Destruction System

### Voxel Grid
Each destructible object is a grid of voxels.

```rust
struct VoxelGrid {
    voxels: Vec<Vec<Voxel>>,
    width: u32,
    height: u32,
    voxel_size: f32,
}

struct Voxel {
    material: MaterialType,
    health: u8,
    connected: bool,
}
```

### Damage Propagation
1. Apply damage to hit voxels
2. Check structural integrity
3. Disconnect unsupported voxels
4. Spawn debris particles
5. Update collision mesh

## Game States

```rust
enum GameState {
    MainMenu,
    Playing(PlayState),
    Paused,
    GameOver,
}

enum PlayState {
    OnFoot,      // Human-scale gameplay
    InMech,      // Titan mech gameplay
    Transition,  // Switching between modes
    Cutscene,    // Story sequences
}
```

## Asset Pipeline

### Development
- Hot-reload sprites and config files
- Debug visualization for physics/AI
- Performance profiling overlay

### Production
- Pack sprites into atlases
- Compress audio files
- Bundle into single executable

## Dependencies

```toml
[dependencies]
# Core
sdl2 = { version = "0.37", features = ["image", "mixer", "ttf"] }

# ECS
hecs = "0.10"  # Or implement custom

# Math
glam = "0.25"  # Fast math library

# Physics
rapier2d = "0.18"  # Optional: for complex physics

# Serialization
serde = { version = "1.0", features = ["derive"] }
ron = "0.8"  # Rusty Object Notation for configs

# Utilities
anyhow = "1.0"  # Error handling
log = "0.4"  # Logging
env_logger = "0.11"  # Log implementation

# Development
hot-lib-reloader = "0.6"  # Hot reload (dev only)
```

## Performance Targets

- **Frame Rate**: 60 FPS minimum on mid-range hardware
- **Entity Count**: Support 1000+ active entities
- **Particle Count**: 10,000+ particles simultaneously
- **Destruction**: Real-time voxel updates for 100x100 grids

## Development Milestones

### Milestone 1: Foundation (Week 1-2)
- [ ] SDL2 window creation
- [ ] Basic ECS implementation
- [ ] Sprite rendering
- [ ] Input handling

### Milestone 2: Core Gameplay (Week 3-4)
- [ ] Player movement (on-foot)
- [ ] Mech transformation
- [ ] Camera system
- [ ] Basic physics

### Milestone 3: Combat & Destruction (Week 5-6)
- [ ] Weapon systems
- [ ] Voxel destruction
- [ ] Particle effects
- [ ] Basic enemies

### Milestone 4: AI & Systems (Week 7-8)
- [ ] Kaiju AI
- [ ] Wave spawning
- [ ] Defense building
- [ ] Resource system

### Milestone 5: Content & Polish (Week 9-10)
- [ ] Level design
- [ ] Story implementation
- [ ] Audio integration
- [ ] Performance optimization

### Milestone 6: Release Prep (Week 11-12)
- [ ] Bug fixes
- [ ] Platform testing
- [ ] Distribution setup
- [ ] Documentation

## Platform-Specific Considerations

### Windows
- DirectX fallback for older systems
- Windows installer with shortcuts

### macOS
- Metal rendering support (future)
- App bundle with code signing
- Retina display support

### Linux
- AppImage for distribution
- Wayland/X11 compatibility
- Package manager support (Flatpak)

## Future Expansions

### Potential DLC Systems
- Multiplayer co-op
- Level editor
- Mod support
- Procedural generation

### Engine Reusability
The modular design allows using this engine for:
- Other 2D action games
- Tower defense games
- Puzzle platformers
- Real-time strategy games