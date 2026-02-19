# Contributing to Blackjack

Thank you for your interest in contributing to the Blackjack project! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)

## Code of Conduct

This project follows a simple code of conduct:

- Be respectful and constructive
- Welcome newcomers and help them learn
- Focus on what's best for the project and community
- Accept constructive criticism gracefully

## Getting Started

1. **Fork the Repository**
   ```bash
   # Click the "Fork" button on GitHub
   ```

2. **Clone Your Fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/blackjack.git
   cd blackjack
   ```

3. **Add Upstream Remote**
   ```bash
   git remote add upstream https://github.com/skharchikov/blackjack.git
   ```

4. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Cargo (included with Rust)
- Git

### Building the Project

```bash
# Build all packages
cargo build

# Build specific package
cargo build -p blackjack-core
cargo build -p server
cargo build -p cli

# Build in release mode
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for specific package
cargo test -p blackjack-core

# Run tests with output
cargo test -- --nocapture
```

### Running the Application

```bash
# Start the server
cargo run --bin server

# In another terminal, start the CLI
cargo run --bin cli
```

## How to Contribute

### Reporting Bugs

When reporting bugs, please include:

- Rust version (`rustc --version`)
- Operating system and version
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Any error messages or logs

### Suggesting Features

Feature suggestions are welcome! Please:

- Check if the feature is already requested in issues
- Provide a clear description of the feature
- Explain the use case and benefits
- Consider providing implementation ideas

### Code Contributions

We welcome code contributions! Good first contributions include:

- Bug fixes
- Documentation improvements
- Test coverage improvements
- Performance optimizations
- New features (discuss in an issue first)

## Coding Standards

### Rust Style

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` to format code
- Use `cargo clippy` to lint code
- Run `cargo check` before committing

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Lint code
cargo clippy --all-targets --all-features

# Run all checks
cargo fmt --check && cargo clippy --all-targets --all-features && cargo test --workspace
```

### Documentation

- Add rustdoc comments for public APIs
- Keep README files up to date
- Include examples in documentation where helpful

Example:

```rust
/// Calculates the value of a hand.
///
/// # Examples
///
/// ```
/// let hand = Hand::new();
/// assert_eq!(hand.value(), 0);
/// ```
pub fn value(&self) -> u8 {
    // implementation
}
```

### Commit Messages

Write clear, descriptive commit messages:

```
Add feature to split pairs in blackjack

- Implement split logic in game engine
- Add UI controls for split action
- Add tests for split scenarios
```

Format:
- First line: Brief summary (50 characters or less)
- Blank line
- Detailed description if needed

## Testing

### Writing Tests

- Add unit tests for new functionality
- Add integration tests for complex interactions
- Ensure tests are deterministic and reproducible
- Use descriptive test names

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ace_can_be_one_or_eleven() {
        let card = Card::new(Rank::Ace, Suit::Spades);
        let values = card.value();
        assert_eq!(values, vec![1, 11]);
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture --test-threads=1
```

## Submitting Changes

### Pull Request Process

1. **Update Your Branch**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Ensure Quality**
   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features
   cargo test --workspace
   ```

3. **Push Your Changes**
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Create Pull Request**
   - Go to your fork on GitHub
   - Click "New Pull Request"
   - Fill out the PR template
   - Link any related issues

### Pull Request Guidelines

Your PR should:

- Have a clear title and description
- Reference related issues (e.g., "Fixes #123")
- Include tests for new functionality
- Update documentation if needed
- Pass all CI checks
- Have clean commit history

### Code Review

- Be patient - reviews may take time
- Respond to feedback constructively
- Make requested changes in new commits
- Squash commits before merge if requested

## Project Structure

Understanding the project structure helps with contributions:

```
blackjack/
├── core/       # Game logic library (no dependencies on UI/server)
├── server/     # WebSocket server (depends on core)
├── cli/        # Terminal UI client (depends on core)
└── docs/       # Additional documentation
```

### Module Boundaries

- **core**: Pure game logic, no IO or UI
- **server**: WebSocket server, state management
- **cli**: Terminal UI, rendering, input handling

## Questions?

If you have questions:

- Check existing issues and pull requests
- Open a new issue with the "question" label
- Be specific and provide context

## License

By contributing to this project, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Blackjack! 🎴
