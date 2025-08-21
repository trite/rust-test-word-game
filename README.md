# Rust Test Word Game

A simple Bevy-based game written in Rust that demonstrates basic game mechanics. The game features "Hello World" text that can be moved around the screen using arrow keys.

## Features

- **Text Movement**: Use arrow keys (↑↓←→) to move the "Hello World" text around the screen
- **Boundary Detection**: Text movement is bounded to the edges of the game window
- **Web Support**: Compiles to WebAssembly for browser gameplay
- **GitHub Pages Deployment**: Automatically builds and deploys to GitHub Pages

## Controls

- **Arrow Keys**: Move the text in the corresponding direction
- Text stays within the window boundaries

## Development

### Prerequisites

- Rust (latest stable version)
- For web builds: `trunk` tool

### Running Locally

```bash
# Run the native version
cargo run

# For web development with hot reload
trunk serve
```

### Building for Web

```bash
# Build for web deployment
trunk build --release
```

The built web assets will be in the `dist/` directory.

## Deployment

The game automatically deploys to GitHub Pages when changes are pushed to the main branch. The GitHub Actions workflow:

1. Builds the game for WebAssembly target with intelligent dependency caching
2. Creates optimized web assets using `trunk`
3. Deploys to GitHub Pages

### Build Performance
The CI/CD pipeline uses intelligent caching to significantly reduce build times:
- **Cold builds** (when dependencies change): ~3-5 minutes
- **Warm builds** (code-only changes): ~1-2 seconds (99%+ faster!)

See [CACHING.md](CACHING.md) for detailed information about the caching implementation.

## Technology Stack

- **Rust**: Programming language
- **Bevy**: Game engine framework
- **WebAssembly**: For web deployment
- **trunk**: Web build tool for Rust
- **GitHub Actions**: CI/CD pipeline
- **GitHub Pages**: Hosting

## License

This project is a demonstration/test project.