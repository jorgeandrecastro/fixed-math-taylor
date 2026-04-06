Fixed-Math-Taylor (v0.2.0)

The major change is that all f32 references have been removed from the calculation engines. Everything now works in Angle (u16) and Fixed (i16), making the library fully consistent with its name: Fixed-Math.

Fixed-Math-Taylor is a 100% fixed-point, ultra-optimized trigonometry library for embedded systems (no_std).

Designed specifically for the RP2040 and microcontrollers without an FPU, it eliminates the cost of software emulation of f32 by using only integers and bit shifts.

🚀 Calculation Engines (Features)

All engines now use Angle (u16) for input and Fixed (i16, Q15) for output.

Feature	Method	Calculation Type	Precision	Ideal Use
lut	Lookup Table + Interpolation	Integer (Q15)	~0.1%	Reference: Audio, Motor Control
taylor	Taylor Series (Order 5)	Integer (Q15)	High	Pure algorithmic computations without LUT
fast-sin	Bhaskara I Approximation	Integer (Q15)	Medium	Animation, fast graphical calculations
🛠 Installation

Add this to your Cargo.toml:

[dependencies]
fixed-math-taylor = { git = "https://github.com/jorgeandrecastro/fixed-math-taylor", features = ["lut"] }

You can also choose "fast-sin" or "taylor" as features.

📖 Usage

The library uses strong typing to guarantee performance.

use fixed_math_taylor::{sin_fixed, cos, Angle, Fixed};

fn main() {
    // Scale: 0 = 0 rad, 16384 = PI/2, 65535 = ~2*PI
    let angle: Angle = 16384; 

    // All engines return an i16 (Q15)
    // 32767 represents 1.0
    let s = sin_fixed(angle);         // LUT engine
    let c = cos(angle);               // LUT engine
    
    #[cfg(feature = "taylor")]
    let s_t = taylor_impl::sin_taylor(angle);
}
🔬 Technical Details
Q15 Format

Output is in Q15: values range from -32768 to 32767.
To get the real value:

real_value_f32 = (fixed_value as f32) / 32767.0
LUT Implementation
Memory Optimization: Uses quadrant symmetry. Only the first quadrant (0 to π/2) is stored (257 points).
Interpolation: Linear interpolation between table points ensures high precision without increasing Flash footprint.
Stability: At 45° the returned value is 23203 (~0.7081). This design choice guarantees stable and predictable PID/control loop responses.
Taylor & Bhaskara

Rewritten entirely using integer arithmetic. They use temporary 64-bit registers for intermediate calculations to prevent overflow while maintaining the speed of a 32-bit processor.

🧪 Validation

The library is fully validated with unit tests covering mathematical precision and correct quadrant handling.

cargo test --all-features
⚖️ License

Copyright © 2026 Jorge Andre Castro.

This software is distributed under the GNU General Public License (GPL) version 2 or later. All derivative code must remain free and open.


French Version 

Le changement majeur est que nous avons supprimé toute mention de f32 dans les moteurs de calcul. Désormais, tout fonctionne en Angle (u16) et Fixed (i16), ce qui rend la bibliothèque cohérente avec son nom : Fixed-Math.Fixed-Math-Taylor (v0.2.0)

Fixed-Math-Taylor est une bibliothèque de trigonométrie 100% virgule fixe ultra-optimisée pour les systèmes embarqués (no_std).Conçue spécifiquement pour le RP2040 et les microcontrôleurs sans unité de calcul flottant (FPU), elle élimine le coût de l'émulation logicielle des f32 en utilisant exclusivement des entiers et des décalages de bits.

🚀 Moteurs de calcul (Features)Tous les moteurs utilisent désormais le type Angle (u16) pour l'entrée et Fixed (i16 Q15) pour la sortie.FeatureMéthodeType de calculPrécisionUsage idéallutTable de recherche + InterpolationEntier (Q15)~0.1%Référence : Audio, Contrôle moteurtaylorSérie de Taylor (Ordre 5)Entier (Q15)HauteAlgorithmique pure sans tablefast-sinApproximation de Bhaskara IEntier (Q15)MoyenneAnimation, calculs graphiques rapides🛠 InstallationAjoutez ceci à votre Cargo.toml :Ini, TOML[dependencies]
fixed-math-taylor = { git = "https://github.com/jorgeandrecastro/fixed-math-taylor", features = ["lut"] }

"fast-sin" or "taylor"

📖 UtilisationLa bibliothèque utilise un typage fort pour garantir la performance.Rustuse fixed_math_taylor::{sin_fixed, cos, Angle, Fixed};

fn main() {
    // Échelle : 0 = 0 rad, 16384 = PI/2, 65535 = ~2*PI
    let angle: Angle = 16384; 

    // Tous les moteurs retournent un i16 (Q15)
    // 32767 représente 1.0
    let s = sin_fixed(angle);         // Moteur LUT
    let c = cos(angle);               // Moteur LUT
    
    #[cfg(feature = "taylor")]
    let s_t = taylor_impl::sin_taylor(angle);
}
🔬 Détails TechniquesFormat Q15La sortie est au format Q15 : les valeurs sont comprises entre -32768 et 32767.Pour obtenir la valeur réelle : valeur_f32 = (valeur_fixe as f32) / 32767.0.Implémentation LUTOptimisation mémoire : Utilise la symétrie des quadrants. Seul le premier quadrant (0 à $\pi/2$) est stocké (257 points).

Interpolation : Une interpolation linéaire est effectuée entre les points de la table pour une précision accrue sans augmenter l'empreinte Flash.Stabilité : À $45^\circ$, la valeur renvoyée est 23203 (~0.7081). Ce choix de conception assure une réponse stable et prédictible pour les boucles de régulation (PID)
.Taylor & BhaskaraRéécrits intégralement en arithmétique entière. Ils utilisent des registres 64 bits de manière temporaire pour les calculs intermédiaires afin d'éviter tout dépassement de capacité (overflow) tout en conservant la vitesse d'un processeur 32 bits.

🧪 ValidationLa bibliothèque est validée par une suite de tests unitaires couvrant la précision mathématique et la gestion des signes par quadrant.Bashcargo test --all-features

⚖️ LicenceCopyright © 2026 Jorge Andre Castro.Ce logiciel est distribué sous la Licence Publique Générale GNU (GPL) version 2.0 ou ultérieure. Tout code dérivé doit rester libre et ouvert.