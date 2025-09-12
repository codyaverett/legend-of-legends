# Legends of Legend

A 2D side-scrolling action game where humanity's last hope pilots giant mechs to defend Earth against colossal Kaiju emerging from the ocean depths.

## Game Features

- **Dual Gameplay Modes**: Switch between on-foot exploration and piloting massive Titan mechs
- **Destructible Environments**: Every building, mountain, and structure can be destroyed with realistic voxel-based physics
- **Dynamic Camera System**: Seamless zoom transitions from intimate on-foot sequences to tactical mech combat
- **Strategic Defense Building**: Construct automated defenses between Kaiju attacks
- **Narrative Choices**: Discover the truth behind the invasion and decide the fate of two civilizations

## Technical Features

- Built with Rust 2024 and SDL2 for maximum performance and safety
- Modular "System of Systems" architecture for extensibility
- Cross-platform support (Windows, macOS, Linux)
- Pixel art aesthetic with modern particle effects
- Entity Component System (ECS) for efficient game object management

## Quick Start

### Prerequisites

- Rust 1.75 or later
- SDL2 development libraries

#### Installing SDL2

**macOS:**
```bash
brew install sdl2 sdl2_image sdl2_mixer sdl2_ttf
```

**Ubuntu/Debian:**
```bash
sudo apt-get install libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev
```

**Windows:**
Download SDL2 development libraries from https://www.libsdl.org/download-2.0.php and follow the setup instructions for your toolchain.

### Building and Running

```bash
# Clone the repository
git clone https://github.com/yourusername/legends-of-legend.git
cd legends-of-legend

# Build the project
cargo build --release

# Run the game
cargo run --release
```

### Development Mode

```bash
# Run with hot-reload and debug features
cargo run --features dev

# Run tests
cargo test

# Check code quality
cargo clippy
```

## Project Structure

```
legends-of-legend/
├── docs/           # Game design and technical documentation
├── src/
│   ├── engine/     # Core engine systems
│   ├── systems/    # Game mechanics
│   └── game/       # Game-specific logic
├── assets/         # Sprites, audio, and level data
└── Cargo.toml      # Project dependencies
```

## Documentation

- [Game Lore and Story](docs/LORE.md)
- [Technical Architecture](docs/ARCHITECTURE.md)

## Controls

### On-Foot Mode
- **Arrow Keys/WASD**: Move
- **Space**: Jump
- **E**: Enter Titan mech
- **Tab**: Open inventory

### Mech Mode
- **Arrow Keys/WASD**: Move mech
- **Mouse**: Aim weapons
- **Left Click**: Primary weapon
- **Right Click**: Secondary weapon
- **E**: Exit mech
- **Shift**: Boost

## Development Status

🚧 **Early Development** 🚧

This project is in active development. Current focus:
- [ ] Core engine implementation
- [ ] Basic player movement
- [ ] Mech transformation system
- [ ] Voxel destruction physics
- [ ] First Kaiju enemy

## Contributing

This is currently a personal project, but suggestions and bug reports are welcome! Please open an issue to discuss any changes you'd like to propose.

## License

This project is currently under development. License to be determined.

## Acknowledgments

- Inspired by classic games like Rampage, Metal Slug, and modern indie titles
- Built with the amazing Rust and SDL2 communities
- Special thanks to the pixel art community for inspiration

## Contact

For questions or feedback, please open an issue on GitHub.

---

*"In the darkest depths, legends rise."*