# Rust Test Word Game

A simple Wordle-like word guessing game written in Rust. The game challenges players to guess a 5-letter word within 6 attempts, providing feedback through colored emojis.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Prerequisites
- Rust 1.89.0+ is required (installed and available)
- Cargo 1.89.0+ is required (installed and available)

### Core Development Commands
- Build the project: `cargo build` -- takes ~0.5 seconds for debug builds
- Build optimized release: `cargo build --release` -- takes ~0.5 seconds
- Run the game: `cargo run` -- starts immediately after build
- Run tests: `cargo test` -- takes ~0.3 seconds, runs 4 unit tests
- Run release tests: `cargo test --release` -- takes ~0.3 seconds
- Quick syntax check: `cargo check` -- takes ~0.04 seconds (fastest way to check compilation)
- Clean build artifacts: `cargo clean` -- removes target/ directory (19MB of build files)

### Code Quality Commands
- Lint with Clippy: `cargo clippy` -- takes ~0.4 seconds, catches common issues
- Format code: `cargo fmt` -- formats all Rust code to standard style
- Check formatting: `cargo fmt --check` -- validates formatting without changing files
- View dependency tree: `cargo tree` -- shows dependency hierarchy (this project has no external dependencies)

All build and test commands complete in under 1 second. NEVER CANCEL these commands - they are very fast.

### Manual Testing the Game
- Run `cargo run` to start the interactive word game
- The target word is "HELLO" (hardcoded for testing)
- Test inputs:
  - Enter "WORLD" - should show: ❌🟡❌✅❌ (O is correct position, R is in word but wrong position)
  - Enter "HELLO" - should show: ✅✅✅✅✅ (exact match, game wins)
  - Enter "quit" - exits the game
  - Test edge cases: words with wrong length should prompt for 5-letter words

## Validation

Always validate changes by running the complete test workflow:
1. `cargo fmt` -- format the code
2. `cargo clippy` -- check for linting issues  
3. `cargo test` -- run all unit tests (4 tests should pass)
4. `cargo run` -- manually test the game functionality

### Manual Testing Scenarios
ALWAYS test these scenarios after making changes to ensure the game works correctly:
1. **Basic gameplay**: Start game, enter "WORLD", then "HELLO", verify emojis and win message
2. **Quit functionality**: Start game, enter "quit", verify clean exit
3. **Input validation**: Enter words of wrong length (< or > 5 letters), verify error handling
4. **Game over scenario**: Make 6 wrong guesses, verify game over message reveals target word

### CI/Build Requirements
- Always run `cargo fmt` before committing or CI will fail on formatting checks
- Always run `cargo clippy` before committing or CI will fail on linting
- Always run `cargo test` before committing or CI will fail on test failures

## Project Structure

### Repository Root
```
.
├── .git/              # Git repository metadata
├── .github/          # GitHub configuration and workflows
├── .gitignore        # Ignores target/ build directory
├── Cargo.toml        # Rust project configuration
├── README.md         # Basic project description
└── src/              # Rust source code
    └── main.rs       # Main game logic and tests
```

### Key Files
- `Cargo.toml` - Project metadata, dependencies, and build configuration
- `src/main.rs` - Contains the entire game: main() function, game logic, and unit tests
- `.gitignore` - Excludes `/target` directory (build artifacts)

### Source Code Organization
- `main()` function: Interactive game loop and user interface
- `check_word()` function: Core game logic for comparing guesses to target word
- `tests` module: Unit tests covering exact matches, partial matches, and no matches
- No external dependencies - uses only Rust standard library

## Common Tasks

### Starting Development
1. Clone the repository: `git clone <repo-url>`
2. Navigate to directory: `cd rust-test-word-game`
3. Build and test: `cargo build && cargo test`
4. Run the game: `cargo run`

### Making Changes
1. Edit code in `src/main.rs`
2. Quick check: `cargo check` (fastest compilation check)
3. Run tests: `cargo test` (verify functionality)
4. Lint: `cargo clippy` (catch common issues)
5. Format: `cargo fmt` (apply standard formatting)
6. Manual test: `cargo run` (play the game to verify changes)

### Release Preparation
- Build optimized version: `cargo build --release`
- Run release tests: `cargo test --release`
- Final validation: Run manual testing scenarios listed above

## Development Tips

### Fast Development Cycle
- Use `cargo check` for quick syntax validation during development (0.04s)
- Use `cargo test` to verify logic changes (0.3s)
- Use `cargo clippy` to catch potential issues (0.4s)
- Use `cargo run` for end-to-end testing (immediate startup)

### Code Style
- The project follows standard Rust formatting (enforced by `cargo fmt`)
- Clippy warnings should be addressed (currently warns about using `vec!` instead of arrays)
- All functions are documented with comments explaining the game logic

### Testing Philosophy
- Unit tests cover the core `check_word()` function with various scenarios
- Manual testing validates the complete user experience
- Tests use emoji strings to match the actual game output format

## Troubleshooting

### Common Issues
- **Build failures**: Run `cargo clean` then `cargo build` to start fresh
- **Test failures**: Check that emoji handling is consistent across platforms
- **Formatting issues**: Run `cargo fmt` to auto-fix formatting
- **Clippy warnings**: Address suggestions like using arrays instead of vectors for fixed-size data

### Performance Notes
- All commands complete in under 1 second on typical hardware
- Build artifacts are stored in `target/` directory (~19MB when built)
- No network dependencies - all builds work offline
- Clean rebuilds take the same time as incremental builds due to project simplicity