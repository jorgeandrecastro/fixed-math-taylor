# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in Fixed-Math-Taylor, please **do not** open a public issue. Instead, please report it responsibly to:

**Email**: `georgeandrec@gmail.com`

### Reporting Guidelines

When reporting a vulnerability, please include:

1. **Description**: Clear description of the vulnerability
2. **Location**: Specific code file(s) and line number(s) if applicable
3. **Impact**: Potential impact on systems using the library
4. **Reproduction**: Steps to reproduce the issue (if possible)
5. **Suggested fix**: Optional proposed solution

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Assessment**: Within 5 business days
- **Fix & Release**: Timeline depends on vulnerability severity:
  - **Critical**: Expedited patch release (24-72 hours)
  - **High**: Patch release within 2 weeks
  - **Medium/Low**: Included in next scheduled release

## Security Considerations

### Integer Arithmetic

This library uses fixed-point Q15 arithmetic. Developers should be aware:

- **Overflow protection**: Intermediate calculations use 64-bit registers to prevent overflow
- **Range restrictions**: Input angles are 16-bit; values wrap at 65536
- **Output bounds**: Results are always in Q15 range [-32768, 32767]

### no_std Compatibility

Security implications of `#![no_std]`:

- **No panics on allocation**: Library has no heap allocations
- **Deterministic execution**: No runtime surprises
- **Embedded-safe**: Suitable for safety-critical systems

### Dependency Security

This library has **zero external dependencies**, eliminating the supply chain attack surface of dependency management.

## Security Practices

### Code Review

- All code changes via pull request
- Community code review before merge
- Static analysis via `cargo clippy`

### Testing

- Comprehensive unit test coverage
- Tests for edge cases and boundary conditions
- Validation across all supported platforms

### Continuous Improvement

- Regular security audits
- Dependency scanning (when applicable)
- Keeping Rust toolchain updated

## Platform-Specific Security Notes

### RP2040

When deploying on RP2040:
- Ensure Flash memory is protected if containing sensitive data
- Note that this library performs no hardware-level security

### Cortex-M0+

When deploying on Cortex-M0+:
- Stack overflow protection depends on platform and bootloader
- This library has minimal stack footprint (~20 bytes)

## Known Limitations

### No Cryptographic Use

This library is **not** suitable for cryptographic operations. It should not be used for:
- Encryption/decryption
- Authentication
- Key generation
- Any security-critical random number generation

### Precision Trade-offs

Different calculation engines have different precision levels. For this reason:
- Do not use for safety-critical control systems without independent verification
- Always validate calculations against known reference values
- Test on target hardware before deployment

## Version Support

| Version | Status | Security Support |
|---------|--------|------------------|
| 0.3.x | Current | Full support |
| 0.2.x | Outdated | Limited (update recommended) |
| 0.1.x | Outdated | No support (deprecated) |

## Acknowledgments

We appreciate coordinated vulnerability disclosure and will acknowledge security researchers in:
- Security advisories
- Changelog entries
- Project documentation (with permission)

---

Thank you for helping keep Fixed-Math-Taylor secure! 🛡️
