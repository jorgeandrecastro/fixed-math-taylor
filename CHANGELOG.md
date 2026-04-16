# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-04-16

### Added 
- **Safety using #![forbid(unsafe_code)] **


## [0.3.1] - 2026-04-08

### Added
- **Modular design**: Three independent calculation engines selectable via Cargo features
  - `lut`: Lookup table with linear interpolation (fastest, ~0.1% error)
  - `taylor`: 5th-order Taylor series with pure integer arithmetic
  - `fast-sin`: Bhaskara I approximation for ultra-fast computation
- `sin_cos()` function for simultaneous sine/cosine computation (faster than separate calls)
- `radians_to_angle()` utility for converting radians to the angle format used by the library
- Comprehensive documentation with performance benchmarks
- Detailed doc comments for all public functions and types
- Full test coverage for all three calculation engines

### Changed
- **Type system**: Standardized to use `Angle` (u16) for input and `Fixed` (i16) for Q15 output
- Improved precision of LUT interpolation algorithm
- Optimized Q15 arithmetic to minimize rounding errors
- Better compiler optimizations via `opt-level = "z"` and `lto = true`

### Fixed
- Corrected angle wrapping in fast-sin engine for boundary cases near π and 2π
- Fixed intermediate overflow issues in Taylor series by using 64-bit intermediate calculations
- Improved precision at quadrant boundaries

### Performance
- LUT engine: ~2.4 µs per call on RP2040 (125 MHz)
- Taylor engine: ~12.8 µs per call
- Fast-Sin engine: ~1.6 µs per call

## [0.2.0] - 2026-03-15

### Added
- Initial release with lookup table engine
- Support for RP2040 and Cortex-M0+
- Basic `sin_fixed()` and `cos_fixed()` functions

### Added

## [0.1.0] - 2026-02-01

### Added
- Foundation project structure
- Core Q15 arithmetic utilities
- Initial proof-of-concept implementation

---

## Future Roadmap

### Planned for v0.4.0
- [ ] Tangent function (`tan()`) support
- [ ] Inverse trigonometric functions (`asin()`, `acos()`, `atan2()`)
- [ ] Enhanced precision modes
- [ ] Additional optimization for Cortex-M4 FPU-variants

### Planned for v1.0.0
- [ ] Stability guarantees and semantic versioning commitment
- [ ] Extended documentation and tutorials
- [ ] Performance profiling suite
- [ ] Additional embedded platform support (STM32, nRF52, etc.)

---

## Migration Guide

### From v0.2 to v0.3

The API has been significantly improved for better type safety and computational efficiency.

**Before (v0.2):**
```rust
let result = sin_fixed(angle);
```

**After (v0.3):**
```rust
// Same functionality, but with better type hints:
use fixed_math_taylor::{sin_fixed, Angle, Fixed};
let angle: Angle = 16384;
let result: Fixed = sin_fixed(angle);
```

The trigonometric function signatures remain unchanged; only the type annotations are more explicit.

---

## Reporting Issues

Found a bug or have a suggestion? Please report it on [GitHub Issues](https://github.com/jorgeandrecastro/fixed-math-taylor/issues).

Include:
- Your target platform (RP2040, ARM Cortex-M0+, etc.)
- The calculation engine you're using (lut, taylor, or fast-sin)
- A minimal reproduction example
- Expected vs. actual output
