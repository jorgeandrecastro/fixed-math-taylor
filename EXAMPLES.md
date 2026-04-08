# Usage Examples

This document provides practical examples of how to use Fixed-Math-Taylor in embedded applications.

## Basic Trigonometry

### Simple Sine Calculation

```rust
use fixed_math_taylor::{sin_fixed, Angle, Fixed};

// Create an angle at 45 degrees
let angle_45deg: Angle = 8192; // 45° out of 65536 total

let sine_value: Fixed = sin_fixed(angle_45deg);

// Convert to float for display
let sine_f32 = (sine_value as f32) / 32767.0;
println!("sin(45°) ≈ {:.4}", sine_f32); // ~0.7071
```

### Simultaneous Sine and Cosine

```rust
use fixed_math_taylor::{sin_cos, Angle};

let angle: Angle = 16384; // π/2

// More efficient than calling sin_fixed() and cos_fixed() separately
let (sin_val, cos_val) = sin_cos(angle);
```

## Motor Control

### PWM Sine Wave Generation

A common use case: generating smooth PWM pulses for motor control.

```rust
#![no_std]

use fixed_math_taylor::{sin_fixed, Angle, Fixed};

fn generate_pwm_samples(sample_count: usize, max_pwm: u16) -> [u16; 256] {
    let mut pwm_values = [0u16; 256];
    
    for i in 0..sample_count {
        // Map sample index to angle (0 to 2π)
        let angle = ((i as u32 * 65536) / sample_count as u32) as u16;
        
        // Get sine value
        let sine_fixed = sin_fixed(angle);
        
        // Convert to PWM duty cycle
        // Map [-32768, 32767] to [0, max_pwm]
        let pwm_duty = (((sine_fixed as i32 + 32768) * max_pwm as i32) / 65536) as u16;
        pwm_values[i] = pwm_duty;
    }
    
    pwm_values
}

// Use in your main loop:
// for sample in pwm_samples.iter() {
//     pwm.set_duty(*sample);
// }
```

### Smooth Motor Speed Control

```rust
use fixed_math_taylor::{sin_fixed, Angle};

fn smooth_speed_ramp(current_phase: &mut Angle, target_speed_rpm: u16) -> u16 {
    // Increment phase proportionally to target speed
    let phase_increment = ((target_speed_rpm as u32 * 256) / 60) as u16;
    *current_phase = current_phase.wrapping_add(phase_increment);
    
    // Convert sine output to speed command [0..100]%
    let sine_val = sin_fixed(*current_phase);
    let speed_percent = (((sine_val as i32 + 32768) * 100) / 65536) as u16;
    
    speed_percent.min(100)
}
```

## Audio Synthesis

### Tone Generation

```rust
use fixed_math_taylor::{sin_fixed, Angle};

struct ToneGenerator {
    phase: Angle,
    phase_increment: u16,
}

impl ToneGenerator {
    fn new(frequency_hz: u16, sample_rate_hz: u16) -> Self {
        // phase_increment = (frequency / sample_rate) * 65536
        let phase_increment = ((frequency_hz as u32 * 65536) / sample_rate_hz as u32) as u16;
        
        ToneGenerator {
            phase: 0,
            phase_increment,
        }
    }
    
    fn next_sample(&mut self) -> i16 {
        let sample = sin_fixed(self.phase);
        self.phase = self.phase.wrapping_add(self.phase_increment);
        sample
    }
}

// Usage:
// let mut generator = ToneGenerator::new(440, 16000); // 440 Hz at 16kHz
// for _ in 0..16000 {
//     let sample = generator.next_sample();
// }
```

## Graphics & Animation

### Rotating Circle Animation

```rust
use fixed_math_taylor::{sin_fixed, cos_fixed, radians_to_angle, Angle, Fixed};
use core::f32::consts::PI;

struct Point {
    x: i16,
    y: i16,
}

fn rotate_point(x: i16, y: i16, angle_increment: Angle) -> Point {
    static mut ANGLE: Angle = 0;
    
    unsafe { ANGLE = ANGLE.wrapping_add(angle_increment); }
    
    let sin_val = sin_fixed(unsafe { ANGLE }) as i32;
    let cos_val = cos_fixed(unsafe { ANGLE }) as i32;
    
    // Rotate using Q15 arithmetic
    let x_rot = ((x as i32 * cos_val - y as i32 * sin_val) >> 15) as i16;
    let y_rot = ((x as i32 * sin_val + y as i32 * cos_val) >> 15) as i16;
    
    Point {
        x: x_rot,
        y: y_rot,
    }
}
```

### Smooth Pulsing Animation

```rust
use fixed_math_taylor::sin_fixed;

fn calculate_pulse_intensity(phase: &mut u16) -> u8 {
    *phase = phase.wrapping_add(256);
    
    // Map sine output [-32768, 32767] to brightness [0, 255]
    let sine_val = sin_fixed(*phase) as i32;
    (((sine_val + 32768) * 255) / 65536) as u8
}
```

## PID Control Loop

### Using Sine/Cosine for Reference Signals

```rust
use fixed_math_taylor::{sin_fixed, cos_fixed, Angle};

struct PIDController {
    setpoint: i16,
    error_integral: i32,
    last_error: i16,
    angle_ref: Angle,
}

impl PIDController {
    fn update(&mut self, measured_value: i16, kp: i16, ki: i16, kd: i16) -> i16 {
        // Use sine wave as reference signal
        let reference = sin_fixed(self.angle_ref);
        
        let error = reference - measured_value;
        self.error_integral += error as i32;
        let error_derivative = error - self.last_error;
        
        // PID calculation in fixed-point
        let p_term = ((kp as i32) * (error as i32)) >> 15;
        let i_term = ((ki as i32) * self.error_integral) >> 30;
        let d_term = ((kd as i32) * (error_derivative as i32)) >> 15;
        
        let output = (p_term + i_term + d_term) as i16;
        
        self.last_error = error;
        self.angle_ref = self.angle_ref.wrapping_add(128);
        
        output
    }
}
```

## Conversion Utilities

### Converting Between Formats

```rust
use fixed_math_taylor::{to_fixed, from_fixed, radians_to_angle};
use core::f32::consts::PI;

// Float to Fixed-Point
let float_val = 0.75;
let fixed_val = to_fixed(float_val);

// Fixed-Point to Float
let reconstructed = from_fixed(fixed_val);
assert!((reconstructed - float_val).abs() < 0.0001);

// Radians to Angle
let angle = radians_to_angle(PI / 4.0); // 45 degrees
assert_eq!(angle, 8192);
```

## Selection Guide by Use Case

| Use Case | Recommended Engine | Reason |
|----------|-------------------|--------|
| **Motor Control** | LUT | Stability, precision |
| **Audio Synthesis** | LUT | Smooth waveforms |
| **Graphics** | Fast-Sin | Speed over precision |
| **Algorithm** | Taylor | No Flash table needed |
| **General Purpose** | LUT | Best balance |

## Performance Considerations

### Memory Usage

```rust
// LUT engine: ~640 bytes static data
// Taylor engine: ~0 bytes (pure algorithm)
// Fast-Sin engine: ~0 bytes (pure algorithm)

// Typical stack usage: ~20 bytes per function call
```

### Execution Time

```
// On RP2040 (125 MHz):
// sin_fixed (LUT):    ~2.4 µs
// sin_taylor (Taylor): ~12.8 µs
// sin_fast (Fast):    ~1.6 µs
```

Choose your engine based on your priorities:
- **LUT**: Best balance of speed and precision
- **Taylor**: No Flash overhead, good precision
- **Fast-Sin**: Absolute speed, acceptable precision loss

## Embedded Targets

### RP2040 Example

```rust
#![no_std]
#![no_main]

use embedded_rp_pico::*;
use fixed_math_taylor::{sin_fixed, Angle};

#[entry]
fn main() -> ! {
    let mut angle: Angle = 0;
    
    loop {
        let val = sin_fixed(angle);
        // Use val for PWM, DAC, etc.
        angle = angle.wrapping_add(256);
    }
}
```

### ARM Cortex-M0+ Example

```rust
#![no_std]

use fixed_math_taylor::{sin_fixed, cos_fixed};

pub fn compute_orientation(gyro_data: &[i16; 3]) {
    for &sample in gyro_data {
        let angle = (sample as u16).wrapping_add(256);
        let sin_val = sin_fixed(angle);
        let cos_val = cos_fixed(angle);
        
        // Apply orientation calculation
    }
}
```

---

## Troubleshooting

### Q: Why is my result always zero?
**A:** Make sure you're converting Fixed-Point back to float for display:
```rust
let val_f32 = (fixed_val as f32) / 32767.0;
```

### Q: How do I get precision beyond binary degrees?
**A:** Use interpolation or the Taylor engine for finer granularity.

### Q: What's the maximum input angle?
**A:** Use `u16::MAX` (65535); angles wrap automatically at 2π.

### Q: Can I use this in real-time systems?
**A:** Yes! All engines have deterministic execution time (no branching in hot paths).

---

For more information, see the [main README](README.md) and [API documentation](https://docs.rs/fixed-math-taylor).
