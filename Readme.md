Fixed-Math-Taylor (v0.3.0)

Fixed-Math-Taylor is a 100% fixed-point, ultra-optimized trigonometry library for embedded systems (#![no_std]).

Designed specifically for the RP2040 and microcontrollers without a floating-point unit (FPU), it eliminates the cost of software f32 emulation by using only integers and bit shifts.

🚀 Calculation Engines (Features)

All engines now use Angle (u16) for input and Fixed (i16, Q15) for output.

| Feature  | Method                       | Calculation Type | Precision | Ideal Use                                         |
| -------- | ---------------------------- | ---------------- | --------- | ------------------------------------------------- |
| lut      | Lookup Table + Interpolation | Integer (Q15)    | ~0.1%     | Reference: Audio, Motor Control                   |
| taylor   | Taylor Series (Order 5)      | Integer (Q15)    | High      | Pure algorithmic computations without Flash table |
| fast-sin | Bhaskara I                   | Integer (Q15)    | Medium    | Animation, fast graphical calculations            |


🛠 Installation

Add this to your Cargo.toml:

# Choose your engine via features
fixed-math-taylor = { git = "https://github.com/jorgeandrecastro/fixed-math-taylor", features = ["lut"] }

You can choose "fast-sin" or "taylor" as features depending on your needs.

📖 Usage

The library uses strong typing to guarantee raw performance without ambiguity.

use fixed_math_taylor::{sin_fixed, cos_fixed, Angle, Fixed};

fn main() {
    // Angle scale (u16):
    // 0 = 0 rad, 16384 = PI/2, 32768 = PI, 65535 ≈ 2*PI
    let angle: Angle = 16384; 

    // All engines return a Fixed (i16 / Q15)
    // 32767 represents 1.0 (or 100%)
    let s = sin_fixed(angle);  // LUT engine
    let c = cos_fixed(angle);  // LUT engine
    
    #[cfg(feature = "taylor")]
    let s_t = fixed_math_taylor::taylor_impl::sin_taylor(angle);
}
🔬 Technical Details
Q15 Format

Output is in Q15: values range from -32768 to 32767.
Meaning: 32767 ≈ 1.0, 0 = 0.0, -32767 ≈ -1.0.
Conversion: For human-readable values:

value_f32 = (fixed_value as f32) / 32767.0
LUT Implementation
Memory Optimization: Uses quadrant symmetry. Only the first quadrant (0 to π/2) is stored (257 points).
Interpolation: Linear interpolation between table points ensures higher precision without increasing Flash footprint.
Stability: At 45° the returned value is 23203 (~0.7081). This choice guarantees a stable response for PID/control loops.
Taylor & Bhaskara

Rewritten entirely in integer arithmetic. They temporarily use 64-bit registers for intermediate calculations to prevent overflow while maintaining the speed of a 32-bit processor.

🧪 Validation

The library is validated with a suite of unit tests covering mathematical precision and quadrant sign handling.

cargo test --all-features
⚖️ License

Copyright © 2026 Jorge Andre Castro.

This software is distributed under the GNU General Public License (GPL) version 2 or later. All derivative code must remain free and open.

French Version 

Fixed-Math-Taylor (v0.3.0)
Fixed-Math-Taylor est une bibliothèque de trigonométrie 100% virgule fixe, ultra-optimisée pour les systèmes embarqués (#![no_std]).

Conçue spécifiquement pour le RP2040 et les microcontrôleurs sans unité de calcul flottant (FPU), elle élimine le coût de l'émulation logicielle des f32 en utilisant exclusivement des entiers et des décalages de bits.

🚀 Moteurs de calcul (Features)
Tous les moteurs utilisent désormais le type Angle (u16) pour l'entrée et Fixed (i16 Q15) pour la sortie.

Feature,Méthode,Type de calcul,Précision,Usage idéal
lut,Table de recherche + Interpolation,Entier (Q15),~0.1%,"Référence : Audio, Contrôle moteur"
taylor,Série de Taylor (Ordre 5),Entier (Q15),Haute,Algorithmique pure sans table Flash
fast-sin,Bhaskara I,Entier (Q15),Moyenne,"Animation, calculs graphiques rapides"


🛠 InstallationAjoutez ceci à votre Cargo.toml :Ini, TOML[dependencies]
# Choisissez votre moteur via les features
fixed-math-taylor = { git = "https://github.com/jorgeandrecastro/fixed-math-taylor", features = ["lut"] }
on peut choisir "fast-sin" or "taylor" comme  features selon le sohait .

📖 UtilisationLa bibliothèque utilise un typage fort pour garantir la performance brute sans ambiguïté.Rustuse fixed_math_taylor::{sin_fixed, cos_fixed, Angle, Fixed};

fn main() {
    // Échelle de l'Angle (u16) : 
    // 0 = 0 rad, 16384 = PI/2, 32768 = PI, 65535 = ~2*PI
    let angle: Angle = 16384; 

    // Tous les moteurs retournent un Fixed (i16 / Q15)
    // 32767 représente 1.0 (ou 100%)
    let s = sin_fixed(angle);  // Moteur LUT
    let c = cos_fixed(angle);  // Moteur LUT
    
    #[cfg(feature = "taylor")]
    let s_t = fixed_math_taylor::taylor_impl::sin_taylor(angle);
}
🔬 Détails TechniquesFormat Q15La sortie est au format Q15 : les valeurs sont comprises entre -32768 et 32767.Signification : 32767 ≈ 1.0, 0 = 0.0, -32767 ≈ -1.0.Conversion : Pour une lecture humaine, valeur_f32 = (valeur_fixe as f32) / 32767.0.Implémentation LUTOptimisation mémoire : Utilise la symétrie des quadrants. 

Seul le premier quadrant (0 à $\pi/2$) est stocké (257 points).Interpolation : Une interpolation linéaire est effectuée entre les points de la table pour une précision accrue sans augmenter l'empreinte Flash.Stabilité : À $45^\circ$, la valeur renvoyée est 23203 (~0.7081). 

Ce choix assure une réponse stable pour les boucles de régulation (PID).Taylor & BhaskaraRéécrits intégralement en arithmétique entière. Ils utilisent des registres 64 bits de manière temporaire pour les calculs intermédiaires afin d'éviter tout dépassement de capacité (overflow) tout en conservant la vitesse d'un processeur 32 bits.

🧪 ValidationLa bibliothèque est validée par une suite de tests unitaires couvrant la précision mathématique et la gestion des signes par quadrant.Bashcargo test --all-features

⚖️ LicenceCopyright © 2026 Jorge Andre Castro. Ce logiciel est distribué sous la Licence Publique Générale GNU (GPL) version 2.0 ou ultérieure. Tout code dérivé doit rester libre et ouvert.