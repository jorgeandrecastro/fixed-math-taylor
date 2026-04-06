// Copyright (C) 2026 Jorge Andre Castro
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 or the License, or
// (at your option) any later version.

#![no_std]

//! # Fixed-Math-Taylor (Modular Edition)
//! 
//! Bibliothèque de trigonométrie haute performance.
//! Activez les moteurs souhaités via les Cargo Features :
//! - `lut` : Virgule fixe Q15 ultra-rapide (recommandé pour MCU).
//! - `taylor` : Série de Taylor (f32) pour la précision.
//! - `fast-sin` : Approximation de Bhaskara I (f32) pour la vitesse.

// --- TYPES DE BASE ---
pub type Angle = u16; // 0..65535 = 0..2π
pub type Fixed = i16; // Q15

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
            1 => interpolate(SIN_LUT[LUT_SIZE - lut_idx], SIN_LUT[LUT_SIZE - lut_idx - 1], frac),
            2 => -interpolate(SIN_LUT[lut_idx], SIN_LUT[lut_idx + 1], frac),
            _ => -interpolate(SIN_LUT[LUT_SIZE - lut_idx], SIN_LUT[LUT_SIZE - lut_idx - 1], frac),
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
#[cfg(feature = "lut")]
pub use lut_impl::sin_fixed;

#[cfg(feature = "lut")]
#[inline(always)]
pub fn cos(angle: Angle) -> Fixed {
    sin_fixed(angle.wrapping_add(16384))
}

#[cfg(feature = "lut")]
#[inline(always)]
pub fn sin_cos(angle: Angle) -> (Fixed, Fixed) {
    (sin_fixed(angle), cos(angle))
}

// ==========================================
// MOTEUR TAYLOR (FEATURE "taylor")
// ==========================================
#[cfg(feature = "taylor")]
pub mod taylor_impl {
    pub fn sin_taylor(x: f32) -> f32 {
        let x2 = x * x;
        // Approximation d'ordre 9 via méthode de Horner
        x * (1.0 + x2 * (-0.166666666 + x2 * (0.008333333 + x2 * (-0.000198412 + x2 * 0.000002755))))
    }
}

// ==========================================
// MOTEUR FAST (FEATURE "fast-sin")
// ==========================================
#[cfg(feature = "fast-sin")]
pub mod fast_impl {
    use core::f32::consts::PI;
    pub fn sin_fast(x: f32) -> f32 {
        let num = 16.0 * x * (PI - x.abs());
        let den = 5.0 * PI * PI - 4.0 * x * (PI - x.abs());
        num / den
    }
}

// ==========================================
// UTILITAIRES COMMUNS
// ==========================================

#[inline(always)]
pub fn to_fixed(x: f32) -> Fixed { (x * 32767.0) as Fixed }

#[inline(always)]
pub fn from_fixed(x: Fixed) -> f32 { (x as f32) / 32767.0 }

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
        assert!((sin_fixed(32768) - 0).abs() <= 1);     // PI (0.0)
        assert!((sin_fixed(49152) - (-32767)).abs() <= 1); // 3PI/2 (-1.0)

        // Test à 45° (Angle 8192 = Index 128 dans une table de 256 pts)
        // Dans ta table, SIN_LUT[128] est exactement 23203.
        let res_raw = sin_fixed(8192); 
        let expected_raw = 23203; 
        
        assert_eq!(res_raw, expected_raw, "Erreur de précision à 45°");
    }

    #[cfg(feature = "lut")]
    #[test]
    fn test_cos_fixed() {
        assert!((cos(0) - 32767).abs() <= 1);
        assert!(cos(16384).abs() <= 1);
        assert!((cos(32768) - (-32767)).abs() <= 1);
    }

    #[cfg(feature = "taylor")]
    #[test]
    fn test_taylor_accuracy() {
        let test_val = PI / 4.0;
        let res = taylor_impl::sin_taylor(test_val);
        let expected = test_val.sin();
        assert!((res - expected).abs() < 0.0001);
    }

    #[cfg(feature = "fast-sin")]
    #[test]
    fn test_fast_sin_approximation() {
        let res = fast_impl::sin_fast(PI / 6.0);
        assert!((res - 0.5).abs() < 0.005);
    }

    #[test]
    fn test_radians_to_angle_wrapping() {
        assert_eq!(radians_to_angle(0.0), 0);
        assert_eq!(radians_to_angle(2.0 * PI), 0);
        // Utilisation de la tolérance pour f32
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
}