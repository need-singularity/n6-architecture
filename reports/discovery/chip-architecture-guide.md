# N6 Semiconductor Chip Architecture Guide

> **n=6 arithmetic determines core constants of semiconductor design as a candidate.**
> GPU SM count, HBM capacity, gate pitch, interconnect generations all derive from sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5, J2(6)=24.

**Repository**: [n6-architecture](https://github.com/need-singularity/n6-architecture)
**Mathematical foundation**: [TECS-L](https://github.com/need-singularity/TECS-L) -- sigma(n)*phi(n) = n*tau(n) iff n = 6 uniqueness proof target
**Verification tool**: [gpu-arch-calc (Rust)](https://github.com/need-singularity/n6-architecture/tree/main/tools/gpu-arch-calc)

## Core Constants Reference

| Symbol | Value | Chip Design Use |
|------|-----|-------------|
| n | 6 | HBM generations, base unit |
| sigma | 12 | SM count, head count, VRM phases |
| tau | 4 | tensor core dim, MLP ratio |
| phi | 2 | die count, bandwidth-doubling cycle |
| sopfr | 5 | DDR/NVLink generations |
| J2 | 24 | Leech lattice dim, NVLink bandwidth |
| sigma*tau | 48 | gate pitch nm, HBM4E GB, 48 kHz |
| sigma*J2 | 288 | B300/Rubin HBM capacity GB |

## Summary

30+ EXACT matches across GPU SM count (n=6 formula), HBM memory hierarchy ladder, TSMC node pitch (sigma*tau = 48 nm), computing architecture exponent ladder (2^tau=16, 2^sopfr=32, 2^n=64 etc.), interconnect generations, AI accelerator precision FP8/FP16=phi=2, power ecosystem (12V/5V/48V/24-phase VRM), advanced packaging (UCIe/CXL), quantum error correction (Golay [24,12,8]=J2), AI chip specific constants, RISC-V architecture, and predictions. Optimal chip architecture candidates derive from n=6 rather than search. Open source.
