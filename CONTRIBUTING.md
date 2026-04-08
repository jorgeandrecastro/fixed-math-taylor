# Contributing to Fixed-Math-Taylor

Thank you for considering a contribution to Fixed-Math-Taylor! We welcome contributions of all kinds, from bug reports and documentation improvements to new features and optimizations.

## Code of Conduct

Please be respectful and constructive in all interactions within this project.

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/fixed-math-taylor.git
   cd fixed-math-taylor
   ```
3. **Create a branch** for your feature or fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites
- Rust 1.56+ (2021 edition)
- `cargo` and `rustc`

### Building & Testing

```bash
# Build with all features
cargo build --all-features

# Run unit tests
cargo test --all-features

# Generate documentation
cargo doc --no-deps --all-features --open

# Check code for issues
cargo clippy --all-features

# Format code
cargo fmt
```

## Contribution Guidelines

### Bug Reports

When reporting a bug, please include:
- **Target platform** (RP2040, Cortex-M0+, x86_64, etc.)
- **Calculation engine** being used (lut, taylor, or fast-sin)
- **Minimal reproduction code** that demonstrates the issue
- **Expected behavior** vs. **actual behavior**
- **Precision metrics** if applicable (error percentage, specific angle values)

### Feature Requests

For new features:
- Describe the use case clearly
- Explain why it's important for embedded systems
- Discuss potential performance impact
- Consider Flash/RAM constraints

### Code Changes

When submitting code:

1. **Follow the existing style**:
   - Use `cargo fmt` before committing
   - Run `cargo clippy` and fix warnings
   - Keep naming conventions consistent

2. **Add tests** for new functionality:
   - Test valid inputs and edge cases
   - Verify quadrant correctness for trigonometric functions
   - Include precision validation if relevant

3. **Document your code**:
   - Add doc comments for all public functions
   - Include examples in doc comments
   - Update README.md if adding public APIs

4. **Maintain no_std compatibility**:
   - No `std` library dependencies
   - No dynamic allocations
   - Test with `#![no_std]` enabled

5. **Update CHANGELOG.md**:
   - Add your change under the "Unreleased" section
   - Follow the format of previous entries
   - Group changes by type (Added, Changed, Fixed, etc.)

### Performance Considerations

Given the embedded systems focus:

- **Prefer bit shifts** over multiplication/division where possible
- **Minimize branching** in hot paths (compiler will unroll loops)
- **Use inline hints** appropriately (`#[inline(always)]` for performance-critical code)
- **Benchmark against baseline** before and after optimizations
- Document performance characteristics in comments and doc

### Precision & Accuracy

- **LUT engine**: Target ~0.1% error across all quadrants
- **Taylor engine**: Target high precision with pure integer arithmetic
- **Fast-Sin engine**: Target ~0.5% error for speed trade-off
- Provide error metrics in documentation

## Commit Guidelines

- Write clear, descriptive commit messages
- Keep commits focused on a single change
- Reference issue numbers when applicable: `Fixes #123`
- Example:
  ```
  Add radians_to_angle() conversion utility

  Fixes #42. Allows users to easily convert standard radian values
  to the Angle (u16) format used by the library.
  ```

## Branch Naming

Use descriptive branch names:
- `feature/feature-name` – New features
- `fix/bug-description` – Bug fixes
- `docs/improvement` – Documentation updates
- `perf/optimization` – Performance improvements
- `test/coverage` – Test additions

## Pull Request Process

1. **Push your branch** to your fork
2. **Open a Pull Request** on the main repository
3. **Fill out the PR template** with:
   - Description of changes
   - Related issue(s)
   - Testing performed
   - Performance impact (if applicable)
4. **Address feedback** from code review
5. **Ensure CI passes** (tests, clippy, format)

## Testing Strategy

### Unit Tests
Located in `src/lib.rs` under `#[cfg(test)]`:
```rust
#[test]
fn test_your_feature() {
    // Arrange
    let input = /* test value */;
    
    // Act
    let result = your_function(input);
    
    // Assert
    assert_eq!(result, expected_value);
}
```

### Edge Cases to Test
- Quadrant boundaries (0, π/2, π, 3π/2)
- Angle wrapping (65535 → 0)
- Q15 overflow scenarios (-32768, 32767)
- Precision across the full angle range

### Supported Features Testing
Test each feature independently:
```bash
cargo test --features lut
cargo test --features taylor
cargo test --features fast-sin
```

## Documentation

- **README.md**: High-level overview, quick start, use cases
- **CHANGELOG.md**: Track all version changes
- **Doc comments**: Technical details, examples, performance notes
- **Examples**: Complete, runnable code samples

## Performance Benchmarking

When optimizing, include before/after metrics:

```bash
# Profile the code
cargo build --release --features lut
```

Use `#[bench]` tests or external tools to measure:
- Execution time (µs per call)
- Code size (bytes)
- Memory usage (bytes)

## Licensing

By contributing to this project, you agree to license your contributions under the **GNU General Public License v2.0 or later**. All derivative works must remain free and open source.

## Recognition

Contributors will be recognized in:
- CHANGELOG.md (per-release contributors)
- GitHub commit history
- Project documentation

---

## Questions or Discussions?

- **Issues**: For bugs and feature requests
- **GitHub Discussions** (if enabled): For general questions and ideas

We're looking forward to your contributions! 🎉
