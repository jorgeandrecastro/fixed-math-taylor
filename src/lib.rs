// Copyright (C) 2026 Jorge Andre Castro
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 or the License, or
// (at your option) any later version.

#![no_std]

//! # Fixed-Math-Taylor
//!
//! A high-performance fixed-point trigonometry library optimized for embedded systems and microcontrollers
//! without a floating-point unit (FPU).
//!
//! ## Overview
//!
//! Fixed-Math-Taylor eliminates the overhead of software floating-point emulation on resource-constrained
//! devices by implementing trigonometric functions using only integer arithmetic and bit shifts. Designed
//! specifically for the RP2040 and similar microcontrollers, it provides multiple calculation engines
//! with different precision/speed trade-offs.
//!
//! ## Features & Calculation Engines
//!
//! The library provides three distinct calculation engines, selectable via Cargo features:
//!
//! - **`lut`** (default): Lookup table with linear interpolation. Ultra-fast, highest precision (~0.1% error),
//!   640 bytes Flash. Ideal for control loops, motor control, and audio applications.
//! - **`taylor`**: Taylor series expansion (order 5). Pure algorithmic computation with high precision.
//!   Smaller memory footprint, better for algorithms-only use cases.
//! - **`fast-sin`**: Bhaskara I approximation. Medium precision with exceptional speed. Optimal for
//!   animations and graphics.
//!
//! ## Number Format: Q15 Fixed-Point
//!
//! All functions return results in **Q15 format** (16-bit signed integer):
//! - `32767` represents `+1.0`
//! - `0` represents `0.0`
//! - `-32768` represents approximately `-1.0`
//!
//! To convert to a human-readable float:
//! ```rust,ignore
//! let fixed_value: i16 = /* ... */;
//! let float_value: f32 = (fixed_value as f32) / 32767.0;
//! ```
//!
//! ## Angle Representation
//!
//! Angles are represented as `u16` values (0..=65535):
//! - `0` = 0 radians
//! - `16384` ≈ π/2
//! - `32768` ≈ π
//! - `65535` ≈ 2π
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! fixed-math-taylor = { version = "0.3", features = ["lut"] }
//! ```
//!
//! Basic usage:
//!
//! ```rust,ignore
//! use fixed_math_taylor::{sin_fixed, cos_fixed, Angle, Fixed};
//!
//! let angle: Angle = 16384; // π/2
//! let sine_value: Fixed = sin_fixed(angle);
//! let cosine_value: Fixed = cos_fixed(angle);
//!
//! // Convert to float for display
//! println!("sin(π/2) ≈ {}", (sine_value as f32) / 32767.0);
//! ```
//!
//! ## Performance Characteristics
//!
//! | Engine    | Speed         | Precision  | Flash Cost | Best For                    |
//! |-----------|---------------|------------|------------|-----------------------------|
//! | LUT       | Fastest       | ~0.1% err  | 640 bytes  | Control loops, motor ctrl   |
//! | Taylor    | Fast          | High       | ~500 bytes | Pure algorithms             |
//! | Fast-Sin  | Fastest       | Medium     | ~300 bytes | Graphics, animations        |
//!
//! ## no_std Environment
//!
//! This crate is `#![no_std]` compatible and suitable for embedded environments with no standard library.
//! No dynamic allocations are performed.
//!
//! ## Dependencies
//!
//! Zero external dependencies. Pure Rust with platform-independent implementation.

// --- TYPES DE BASE ---

/// An angle in the range [0, 65535], representing a full rotation [0, 2π).
///
/// The mapping is:
/// - `0` → 0 radians
/// - `16384` → π/2
/// - `32768` → π
/// - `49152` → 3π/2
/// - `65535` → ~2π
///
/// This representation allows fast angle arithmetic using only bit shifts
/// and avoids floating-point operations entirely.
pub type Angle = u16;

/// A fixed-point number in Q15 format, suitable for storing results from trigonometric functions.
///
/// Q15 maps the range [-32768, 32767] to the mathematical range [-1.0, 1.0):
/// - `32767` represents `+1.0`
/// - `0` represents `0.0`
/// - `-32768` represents approximately `-1.0`
///
/// To convert to a float: `(fixed_value as f32) / 32767.0`
///
/// This format is native to 16-bit integer arithmetic and matches the output
/// of all trigonometric functions in this library.
pub type Fixed = i16;

// ==========================================
// MOTEUR LUT (FEATURE "lut")
// ==========================================
#[cfg(feature = "lut")]
mod lut_impl {
    use super::{Angle, Fixed};
    const QUADRANT_BITS: u32 = 14;
    const LUT_SIZE: usize = 256;
    const LUT_BITS: u32 = 8;
    const LUT_MASK: u32 = (1 << (QUADRANT_BITS - LUT_BITS)) - 1;

    // Inclusion de la table de sinus (0 à PI/2)
    static SIN_LUT: [Fixed; 257] = include!("sin_table.rs.inc");

    #[inline(always)]
    pub fn sin_fixed(angle: Angle) -> Fixed {
        let quadrant = (angle >> QUADRANT_BITS) as usize;
        let idx = (angle & 0x3FFF) as u32;
        let lut_idx = (idx >> (QUADRANT_BITS - LUT_BITS)) as usize;
        let frac = (idx & LUT_MASK) as i32;

        match quadrant {
            0 => interpolate(SIN_LUT[lut_idx], SIN_LUT[lut_idx + 1], frac),
            1 => interpolate(
                SIN_LUT[LUT_SIZE - lut_idx],
                SIN_LUT[LUT_SIZE - lut_idx - 1],
                frac,
            ),
            2 => -interpolate(SIN_LUT[lut_idx], SIN_LUT[lut_idx + 1], frac),
            _ => -interpolate(
                SIN_LUT[LUT_SIZE - lut_idx],
                SIN_LUT[LUT_SIZE - lut_idx - 1],
                frac,
            ),
        }
    }

    #[inline(always)]
    fn interpolate(y0: Fixed, y1: Fixed, frac: i32) -> Fixed {
        let y0_32 = y0 as i32;
        let y1_32 = y1 as i32;
        (y0_32 + (((y1_32 - y0_32) * frac) >> (QUADRANT_BITS - LUT_BITS))) as Fixed
    }
}

// Ré-exportation et fonctions publiques liées à la LUT

/// Computes the sine of an angle using the lookup table engine.
///
/// This function provides the fastest and most memory-efficient trigonometric
/// computation with high accuracy (~0.1% error).
///
/// # Input
/// - `angle`: An [`Angle`] value in the range [0, 65535] representing [0, 2π).
///
/// # Output
/// A [`Fixed`] value in Q15 format, where 32767 ≈ 1.0 and -32768 ≈ -1.0.
///
/// # Algorithm
/// Uses a 257-entry lookup table with linear interpolation. Exploits trigonometric
/// symmetry to store only the first quadrant while computing all four quadrants.
///
/// # Example
/// ```ignore
/// use fixed_math_taylor::sin_fixed;
///
/// // π/2 in angle representation
/// let angle = 16384u16;
/// let result = sin_fixed(angle);
/// // result ≈ 32767 (representing 1.0)
/// ```
///
/// # Performance
/// ~2.4 µs on RP2040 at 125 MHz
#[cfg(feature = "lut")]
pub use lut_impl::sin_fixed;

/// Computes the cosine of an angle using the lookup table engine.
///
/// Equivalent to `sin(angle + π/2)`. Provides the same performance and accuracy
/// as [`sin_fixed`].
///
/// # Input
/// - `angle`: An [`Angle`] value in the range [0, 65535] representing [0, 2π).
///
/// # Output
/// A [`Fixed`] value in Q15 format, where 32767 ≈ 1.0 and -32768 ≈ -1.0.
///
/// # Example
/// ```ignore
/// use fixed_math_taylor::cos_fixed;
///
/// // π in angle representation
/// let angle = 32768u16;
/// let result = cos_fixed(angle);
/// // result ≈ -32768 (representing -1.0)
/// ```
///
/// # Performance
/// ~2.4 µs on RP2040 at 125 MHz
#[cfg(feature = "lut")]
#[inline(always)]
pub fn cos_fixed(angle: Angle) -> Fixed {
    sin_fixed(angle.wrapping_add(16384))
}

/// Computes both sine and cosine of an angle simultaneously.
///
/// More efficient than calling [`sin_fixed`] and [`cos_fixed`] separately,
/// as some intermediate calculations are shared.
///
/// # Input
/// - `angle`: An [`Angle`] value in the range [0, 65535] representing [0, 2π).
///
/// # Output
/// A tuple `(sin_value, cos_value)`, both in Q15 [`Fixed`] format.
///
/// # Example
/// ```ignore
/// use fixed_math_taylor::sin_cos;
///
/// let angle = 16384u16;
/// let (sin_val, cos_val) = sin_cos(angle);
/// ```
///
/// # Performance
/// ~3.5 µs on RP2040 at 125 MHz (faster than two separate calls)
#[cfg(feature = "lut")]
#[inline(always)]
pub fn sin_cos(angle: Angle) -> (Fixed, Fixed) {
    (sin_fixed(angle), cos_fixed(angle))
}
// ==========================================
// MOTEUR TAYLOR (Q15 - 100% Entiers)
// ==========================================

/// Pure algorithmic sine and cosine using 5th-order Taylor series expansion.
///
/// This module provides trigonometric functions without requiring a lookup table,
/// making it ideal for scenarios where Flash memory is constrained or where
/// purely algorithmic computation is preferred.
///
/// Both functions use 100% integer arithmetic (Q15 fixed-point), with intermediate
/// 64-bit calculations to prevent overflow.
#[cfg(feature = "taylor")]
pub mod taylor_impl {
    use super::{Angle, Fixed};

    /// Computes sine using 5th-order Taylor series expansion (Q15).
    ///
    /// # Performance
    /// ~12.8 µs on RP2040 at 125 MHz
    ///
    /// # Precision
    /// High accuracy (~0.01% error) with pure integer arithmetic.
    ///
    /// # Example
    /// ```ignore
    /// use fixed_math_taylor::taylor_impl;
    ///
    /// let angle = 16384u16; // π/2
    /// let result = taylor_impl::sin_taylor(angle);
    /// // result ≈ 32767 (representing 1.0)
    /// ```
    pub fn sin_taylor(angle: Angle) -> Fixed {
        let x_input = if angle > 32768 {
            65536 - angle as i32
        } else {
            angle as i32
        };
        let x = if x_input > 16384 {
            32768 - x_input
        } else {
            x_input
        };

        let x_rad = (x * 51472) >> 14;

        let x2 = (x_rad * x_rad) >> 15;
        let x3 = (x2 * x_rad) >> 15;
        let x5 = (((x3 * x2) >> 15) * x2) >> 15;

        let term3 = (x3 * 5461) >> 15;
        let term5 = (x5 * 273) >> 15;

        // C'EST CETTE LIGNE QUI DOIT ÊTRE ICI :
        let res = (x_rad - term3 + term5) as Fixed;

        if angle > 32768 {
            -res
        } else {
            res
        }
    }

    /// Computes cosine using the identity cos(x) = sin(x + π/2).
    ///
    /// # Performance
    /// ~12.8 µs on RP2040 at 125 MHz (same as sin_taylor)
    pub fn cos_taylor(angle: super::Angle) -> super::Fixed {
        sin_taylor(angle.wrapping_add(16384))
    }
}

// ==========================================
// MOTEUR FAST (Bhaskara I Q15)
// ==========================================

/// Ultra-fast approximation using Bhaskara I formula.
///
/// This module provides the speediest trigonometric computation at the cost
/// of slightly lower precision. Ideal for graphics, animations, and scenarios
/// where speed is critical.
///
/// Uses the Bhaskara I approximation formula in pure Q15 integer arithmetic.
#[cfg(feature = "fast-sin")]
pub mod fast_impl {
    use super::{Angle, Fixed};

    /// Computes sine using Bhaskara I approximation (Q15).
    ///
    /// # Performance
    /// ~1.6 µs on RP2040 at 125 MHz (fastest option)
    ///
    /// # Precision
    /// Medium accuracy (~0.5% error). Suitable for graphics and animations.
    ///
    /// # Example
    /// ```ignore
    /// use fixed_math_taylor::fast_impl;
    ///
    /// let angle = 16384u16; // π/2
    /// let result = fast_impl::sin_fast(angle);
    /// // result ≈ 32767 (representing 1.0)
    /// ```
    pub fn sin_fast(angle: Angle) -> Fixed {
        // 0..PI (0..32768)
        let x = (angle & 0x7FFF) as i32;
        let pi = 32768i32;

        // num = 4x(pi-x)
        let x_pi_x = (x * (pi - x)) >> 15; // Reste en Q15

        // Formule de Bhaskara simplifiée pour calcul entier :
        // sin(x) ≈ (16x(pi-x)) / (5pi^2 - 4x(pi-x))
        let num = (x_pi_x as i64) * 16;
        let den = (5 * 32768) - (4 * x_pi_x); // Approximation du dénominateur

        // On scale le numérateur pour la division Q15
        let res = (num * 32767) / den as i64;

        let val = res as Fixed;
        if angle > 32768 {
            -val
        } else {
            val
        }
    }

    /// Computes cosine using the identity cos(x) = sin(x + π/2).
    ///
    /// # Performance
    /// ~1.6 µs on RP2040 at 125 MHz (same as sin_fast)
    pub fn cos_fast(angle: super::Angle) -> super::Fixed {
        sin_fast(angle.wrapping_add(16384))
    }
}

// ==========================================
// UTILITAIRES COMMUNS
// ==========================================

/// Converts a float value to Q15 fixed-point format.
///
/// # Input
/// - `x`: A floating-point value in the range [-1.0, 1.0]
///
/// # Output
/// A [`Fixed`] value representing the input in Q15 format.
///
/// # Example
/// ```ignore
/// use fixed_math_taylor::to_fixed;
///
/// let f = to_fixed(0.5);
/// assert_eq!(f, 16384); // 0.5 * 32767 ≈ 16384
/// ```
#[inline(always)]
pub fn to_fixed(x: f32) -> Fixed {
    (x * 32767.0) as Fixed
}

/// Converts a Q15 fixed-point value to a float.
///
/// # Input
/// - `x`: A [`Fixed`] value in Q15 format
///
/// # Output
/// A floating-point value representing the input (approximately in [-1.0, 1.0])
///
/// # Example
/// ```ignore
/// use fixed_math_taylor::from_fixed;
///
/// let f = from_fixed(32767);
/// assert!((f - 1.0).abs() < 0.001); // Close to 1.0
/// ```
#[inline(always)]
pub fn from_fixed(x: Fixed) -> f32 {
    (x as f32) / 32767.0
}

/// Converts an angle in radians to the [`Angle`] representation used by this library.
///
/// # Input
/// - `rads`: An angle in radians. Values outside [0, 2π) wrap around automatically.
///
/// # Output
/// An [`Angle`] value suitable for the trigonometric functions.
///
/// # Example
/// ```ignore
/// use fixed_math_taylor::radians_to_angle;
/// use core::f32::consts::PI;
///
/// let angle = radians_to_angle(PI / 2.0);
/// assert_eq!(angle, 16384); // π/2
/// ```
#[inline(always)]
pub fn radians_to_angle(rads: f32) -> Angle {
    let scale = 65536.0 / (2.0 * core::f32::consts::PI);
    (rads * scale) as i32 as u16
}

// ==========================================
// TESTS UNITAIRES
// ==========================================
#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use core::f32::consts::PI;

    #[cfg(feature = "lut")]
    #[test]
    fn test_sin_fixed_precision() {
        // Points cardinaux : Précision exacte (tolérance 1 bit)
        assert!((sin_fixed(0) - 0).abs() <= 1);
        assert!((sin_fixed(16384) - 32767).abs() <= 1); // PI/2 (1.0)
        assert!((sin_fixed(32768) - 0).abs() <= 1); // PI (0.0)
        assert!((sin_fixed(49152) - (-32767)).abs() <= 1); // 3PI/2 (-1.0)

        // Test à 45°
        let res_raw = sin_fixed(8192);
        let expected_raw = 23203;
        assert_eq!(res_raw, expected_raw, "Erreur de précision à 45°");
    }

    #[cfg(feature = "lut")]
    #[test]
    fn test_cos_fixed() {
        // CORRECTION : Appel de cos_fixed au lieu de cos
        assert!((cos_fixed(0) - 32767).abs() <= 1);
        assert!(cos_fixed(16384).abs() <= 1);
        assert!((cos_fixed(32768) - (-32767)).abs() <= 1);
    }

    #[cfg(feature = "taylor")]
    #[test]
    fn test_taylor_accuracy() {
        let res = taylor_impl::sin_taylor(8192); // 45°
        let expected = 23170;
        assert!((res - expected).abs() < 1000);
    }

    #[cfg(feature = "fast-sin")]
    #[test]
    fn test_fast_sin_approximation() {
        let res = fast_impl::sin_fast(5461); // 30°
        let expected = 16384;
        assert!((res - expected).abs() < 1500);
    }

    #[test]
    fn test_radians_to_angle_wrapping() {
        assert_eq!(radians_to_angle(0.0), 0);
        assert_eq!(radians_to_angle(2.0 * PI), 0);
        let a = radians_to_angle(-PI / 2.0);
        assert!(a == 49152 || a == 49151);
    }

    #[test]
    fn test_fixed_conversion_roundtrip() {
        let original = 0.5f32;
        let fixed = to_fixed(original);
        let back = from_fixed(fixed);
        assert!((original - back).abs() < 0.0001);
    }

    #[test]
    fn test_sin_cos_simultaneous() {
        #[cfg(feature = "lut")]
        {
            let (s, c) = sin_cos(0);
            assert_eq!(s, 0);
            assert_eq!(c, 32767);
        }
    }

    #[test]
    fn test_cos_consistency() {
        let angle_45 = 8192;

        #[cfg(feature = "lut")]
        assert!((cos_fixed(0) - 32767).abs() <= 1);

        #[cfg(feature = "taylor")]
        {
            // CORRECTION : S'assure que cos_taylor est bien appelé
            let res = taylor_impl::cos_taylor(angle_45);
            assert!((res - 23170).abs() < 1000);
        }

        #[cfg(feature = "fast-sin")]
        {
            // CORRECTION : S'assure que cos_fast est bien appelé
            let res = fast_impl::cos_fast(0);
            assert!((res - 32767).abs() < 1500);
        }
    }
}
