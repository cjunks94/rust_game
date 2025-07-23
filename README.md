# Rust Cat Clicker Game 🐱

A delightful cat clicker game built with [Bevy](https://bevyengine.org/), featuring animated sprites, random backgrounds, and interactive gameplay.

## Features

- **Animated Cat Sprite**: 8 different animations including idle, walk, sleep, groom, play, jump, cute, and box_play
- **Interactive Clicking**: Click on the cat to trigger cute animations and increment the counter
- **Random Backgrounds**: Rotating summer-themed backgrounds that change every 5 clicks
- **Debug Mode**: Comprehensive debugging tools for animation testing and sprite atlas visualization
- **Modular Architecture**: Clean, organized code structure following Rust best practices

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/cjunks94/rust_game.git
cd rust_game
```

2. Run the game:
```bash
cargo run
```

## How to Play

- **Click the cat** to trigger cute animations and increase your click counter
- **Watch the backgrounds change** automatically every 5 clicks
- **Use debug mode** (see below) to test different animations

## Debug Mode 🐛

Press `D` to toggle debug mode, which provides:

### Debug Features
- **Sprite Atlas Visualization**: See how the sprite sheet is divided into frames
- **Animation Information**: Current animation, frame index, and timing details
- **Grid Overlay**: Visual representation of the 64x64 pixel grid (8×9 layout)
- **Frame Numbers**: Each sprite frame is labeled with its index number

### Animation Testing Shortcuts

While in debug mode, use these keys to test different animations:

| Key | Animation | Description |
|-----|-----------|-------------|
| `1` | Idle | Default resting animation (6 frames) |
| `2` | Walk | Walking animation (3 frames) |
| `3` | Sleep | Sleeping animation (4 frames) |
| `4` | Groom | Grooming/cleaning animation (10 frames) |
| `5` | Play | Playful animation (6 frames) |
| `6` | Jump | Jumping animation (8 frames) |
| `7` | Cute | Box cat cute animation (8 frames) |
| `8` | Box Play | Box cat playing animation (8 frames) |

### Debug Console Output

The debug mode also prints frame information to the console:
```
Loading background: backgrounds/summer5/Summer5.png
Debug mode: true
Frame: 2 (animation: idle, current: 2)
Playing cute animation
Frame: 48 (animation: cute, current: 0)
```

## Project Structure

```
src/
├── main.rs          # Application entry point and plugin setup
├── animation.rs     # Animation system and sprite management
├── debug.rs         # Debug mode functionality and testing tools
└── game.rs          # Core game logic, clicking, and UI

assets/
├── cat_black/
│   └── cat_spritesheet.png  # Main sprite sheet (8×9 grid, 64×64 per frame)
└── backgrounds/
    ├── summer 1/Summer1.png
    ├── summer 2/Summer2.png
    └── ... (8 summer backgrounds total)
```

## Technical Details

### Animation System
- **Grid-based Sprite Sheet**: 8 columns × 9 rows, 64×64 pixels per frame
- **Named Animations**: Easy-to-manage animation library with custom frame sequences
- **Flexible Timing**: Different frame durations for each animation type
- **State Management**: Smooth transitions between animations with optional return states

### Architecture
- **Plugin-based Design**: Modular system using Bevy's plugin architecture
- **Single Responsibility**: Each module handles one specific aspect of the game
- **Resource Management**: Efficient handling of sprites, textures, and game state

## Development

### Running Tests
```bash
cargo test
```

### Building for Release
```bash
cargo build --release
```

### Adding New Animations

1. Add sprite frames to the sprite sheet in the appropriate grid positions
2. Update the `AnimationLibrary::new()` function in `src/animation.rs`:
```rust
animations.insert("my_animation".to_string(), Animation {
    name: "my_animation".to_string(),
    frames: (start_index..end_index).collect(),
    frame_duration: 0.2,
});
```
3. Add a keyboard shortcut in `src/debug.rs` for testing

### Adding New Backgrounds

1. Place background images in `assets/backgrounds/folder_name/`
2. Update the `BackgroundConfig::default()` in `src/game.rs`:
```rust
backgrounds: vec![
    "backgrounds/folder_name/image.png".to_string(),
    // ... other backgrounds
],
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- Built with [Bevy](https://bevyengine.org/) - A refreshingly simple data-driven game engine
- Sprite artwork and backgrounds sourced from various free game asset collections
- Inspired by classic clicker games and the joy of interactive pet simulations

---

**Happy Clicking! 🐾**