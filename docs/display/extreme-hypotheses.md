# N6 Display Extreme Hypotheses

> Split from docs/display-audio/extreme-hypotheses.md (H-DA-61~80)
> Contains display-related extreme hypotheses only.

---

## Source

Full combined extreme hypotheses: [docs/display-audio/extreme-hypotheses.md](../display-audio/extreme-hypotheses.md)

## Display-Relevant Extreme Hypotheses

The original extreme hypotheses (H-DA-61~80) cover both display and audio.
Display-specific items include:

- **H-DA-61**: H.264 slice types (video codec structure)
- **H-DA-63~65**: Video compression block sizes, GOP structures, DCT transforms
- **H-DA-71~75**: Color science deep hypotheses (CIE, gamut, spectral locus)
- **H-DA-76~78**: Display perceptual limits (CFF, JND, contrast sensitivity)

### Key Display Predictions from Extreme Hypotheses

1. **DCT 8×8 = σ-τ universality**: JPEG/MPEG/HEVC all use 8×8 base block = σ-τ=8
2. **Color JND saturation at σ=12-bit**: CIEDE2000 < 0.5 above 12-bit depth
3. **CFF convergence at σ²=144Hz**: Critical flicker fusion empirically saturates near 144Hz
4. **GOP optimal at σ=12**: Video compression efficiency peaks at GOP=12 for 24/60fps content

See original file for full derivations and honesty grades.
