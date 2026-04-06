Fixed-Math-Taylor

Fixed-Math-Taylor is an ultra-optimized trigonometry library for embedded systems (no_std). It provides multiple calculation engines (LUT, Taylor, Bhaskara) configurable via Cargo features.

Designed for the RP2040 and architectures without an FPU, it prioritizes speed and predictability.

🚀 Available Engines
| Feature  | Method                  | Type | Precision | Ideal Use                       |
| -------- | ----------------------- | ---- | --------- | ------------------------------- |
| lut      | Q15 Lookup Table        | i16  | ~0.1%     | Motor control, Audio, Real-time |
| taylor   | Taylor Series (Order 9) | f32  | High      | Scientific calculations         |
| fast-sin | Quadratic Approximation | f32  | Medium    | Animation, Simple graphics      |

🛠 Installation

Add this to your Cargo.toml and choose your engine:

[dependencies]
fixed-math-taylor = { git = "https://github.com/jorgeandrecastro/fixed-math-taylor", features = ["lut"] }
"fast-sin" or "taylor"

📖 Usage Example (Q15 LUT Mode)

The lut mode is the fastest. It uses strong typing to prevent unit errors.

use fixed_math_taylor::{sin_fixed, cos, Angle, to_fixed, from_fixed};

fn main() {
    // 0 = 0 rad, 16384 = PI/2, 65535 = ~2*PI
    let angle: Angle = 16384; 

    let s = sin_fixed(angle); // Returns 32767 (1.0 in Q15)
    let c = cos(angle);       // Returns 0
}
🔬 LUT Implementation Details

The implementation relies on quadrant symmetry to minimize memory footprint:

Storage: Only the first quadrant (0 to π/2) is stored (257 points).

Interpolation: Linear interpolation between table points ensures smooth transitions.
Precision: At 45° (index 128), the value is 23203 (~0.7081). This slight deviation from the ideal sine (0.7071) is a deliberate design choice based on the table used, providing  stability for PI/PID control loops.

🧪 Tests and Validation

The library is fully tested to guarantee robustness of wrapping (periodicity) and accuracy of conversions.

# To test all engines
cargo test --all-features
⚖️ License

Copyright © 2026 Jorge Andre Castro.

This software is distributed under the GNU General Public License (GPL) version 2 or later. This ensures that the code remains free and any improvements contributed by the community benefit everyone.



French Version 

Fixed-Math-TaylorFixed-Math-Taylor est une bibliothèque de trigonométrie ultra-optimisée pour les systèmes embarqués (no_std).
 Elle propose plusieurs moteurs de calcul (LUT, Taylor, Bhaskara) configurables via des Cargo features.Conçue pour le RP2040 et les architectures sans FPU, elle privilégie la vitesse et la prédictibilité.
 
 🚀 Moteurs disponiblesFeatureMéthodeTypePrécisionUsage idéallutTable de recherche Q15i16~0.1%Contrôle moteur, Audio, Temps-réeltaylorSérie de Taylor (Ordre 9)f32HauteCalculs scientifiquesfast-sinApproximation quadratiquef32MoyenneAnimation, Graphismes simples
 
 
 🛠 InstallationAjoute ceci à ton Cargo.toml en choisissant ton moteur :Ini, TOML[dependencies]
fixed-math-taylor = { git = "https://github.com/jorgeandrecastro/fixed-math-taylor", features = ["lut"] }
ou "fast-sin" ou  "taylor"


📖 Exemple d'utilisation (Mode LUT Q15)Le mode lut est le plus performant. Il utilise un typage fort pour éviter les erreurs d'unités.Rustuse fixed_math_taylor::{sin_fixed, cos, Angle, to_fixed, from_fixed};

fn main() {
    // 0 = 0 rad, 16384 = PI/2, 65535 = ~2*PI
    let angle: Angle = 16384; 

    let s = sin_fixed(angle); // Retourne 32767 (1.0 en Q15)
    let c = cos(angle);       // Retourne 0
}

🔬 Détails de l'implémentation LUTL'implémentation repose sur une symétrie de quadrant pour minimiser l'empreinte mémoire :Stockage : Seul le premier quadrant ($0$ à $\pi/2$) est stocké (257 points).
Interpolation : Une interpolation linéaire entre les points de la table garantit une transition fluide.Précision : À $45^\circ$ (index 128), la valeur est de 23203 (soit ~0.7081). Cette légère divergence par rapport au sinus idéal ($0.7071$) est un choix de conception lié à la table de données utilisée, offrant une excellente stabilité pour les boucles de contrôle PI/PID.

🧪 Tests et ValidationLa bibliothèque est intégralement testée pour garantir la robustesse du "wrapping" (périodicité) et la précision des conversions.Bash# Pour tester tous les moteurs
cargo test --all-features


⚖️ LicenceCopyright © 2026 Jorge Andre Castro.Ce logiciel est distribué sous la Licence Publique Générale GNU (GPL) version 2.0 ou ultérieure. Cela garantit que le code reste libre et que toute amélioration apportée par la communauté profite à tous.