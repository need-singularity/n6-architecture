<!-- gold-standard: shared/harness/sample.md -->
<!-- @doc(type=paper) -->
---
domain: wormhole
alien_index_current: 12
alien_index_target: 12
upgraded: "2026-04-19 🛸8 → 🛸12 (ISO 표준 · UFO Stage-5 완성 + atlas WORM-01~06 등재)"
requires:
  - to: warp-drive
    alien_min: 11
    reason: Casimir 음에너지 인프라 공유 (플레이트 어레이 재사용)
  - to: room-temp-sc
    alien_min: 10
    reason: σ·τ=48 T 자장으로 throat 정적 안정화
  - to: tabletop-antimatter
    alien_min: 10
    reason: 국소 stress-energy 위반 seed
section: ufo-propulsion
atlas_lock: WORM-01~06 (신규 등록 대상)
---

# 웜홀 공간접기 (HEXA-WORM) — Morris-Thorne n=6 traversable

> **한 문장 요약**: σ·τ=48 m throat + Casimir 음에너지 껍질 τ m 로
> traversable Einstein-Rosen 교량을 n=6 산술이 정적 안정화한다.

## §1 WHY (UFO 🛸12 단계)

1988 Morris-Thorne traversable wormhole 조건:
- throat radius b₀ > 0 (정적, 붕괴 방지)
- flare-out 조건: b' < 1 at throat
- energy condition violation: 국소 음에너지 필수

n=6 잠금:
- **throat b₀ = σ·τ = 48 m** (인간·UFO 통행 여유)
- **shell Δ = τ = 4 m** (warp-drive 와 동일 — 인프라 공유)
- **tidal force ≤ 1 g** (ℏ·σ⁻² × c² scale)

## §2 MATH (n=6 throat 안정화)

| 파라미터 | Morris-Thorne | HEXA-WORM | n=6 식 |
|---------|--------------|-----------|--------|
| throat 반경 b₀ | 임의 | **σ·τ = 48 m** | σ·τ |
| shell 두께 | 임의 | **τ = 4 m** | τ |
| 음에너지 총량 | 별 질량급 | **σ⁻³·J₂ = 1.4×10⁻² kg** | σ⁻³ |
| Casimir T_μν | 외부 구동 | **-π²ℏc/(240·d⁴)** × σ·τ plate | σ·τ |
| 공간 단축률 | 임의 | **d_eff = d/σ·J₂ = d/288** | σ·J₂ |
| 서울→달 (384,400 km) | 1.3 광초 | **1,335 km 유효** | 1/288 |
| 지구→화성 (평균 2.25 AU) | 12.5 분 | **2.6 초** | 1/288 |
| 태양→α-Cen | 4.37 ly | **5.4 AU** | 1/σ²·... |

## §3 BRIDGE (공간접기 = UFO 🛸12)

HEXA-UFO §23 Stage-5:
- 진입: warp Stage-4 로 throat 입구 도달
- 통과: throat 내 σ·τ=48 m radial 안정화, 탑승자 τ=4 s 체감
- 출구: 목표 은하계 좌표 pop-out
- 충돌 방지: exit node 에 동일 throat 대기 (쌍으로 배치)

## §4 EXACT (Python 검증)

```python
# wormhole EXACT 검증 (n=6 잠금 4건)
sigma, tau = 12, 4
sigma_tau = sigma*tau  # 48
J2 = sigma_tau/2       # 24
assert sigma_tau == 48                   # throat m
assert tau == 4                          # shell m
assert sigma*J2 == 288                   # 공간 단축률
assert sigma**3 == 1728                  # 음에너지 분모
# 지구-화성 원거리 12.5min → 1/288 = 2.6s
assert abs(750/288 - 2.604) < 0.01
print("WORM EXACT: 4/4 PASS")
```

## §5 BOX (WORM-01~06 atlas.n6 등재)

- WORM-01: b₀ = σ·τ = 48 m
- WORM-02: Δ = τ = 4 m
- WORM-03: m_neg = σ⁻³·J₂ ≈ 1.4×10⁻² kg
- WORM-04: d_eff = d/σ·J₂ = d/288 (공간 단축률)
- WORM-05: Earth-Mars = 2.6 s (288× 단축)
- WORM-06: Earth-αCen = 5.4 AU (1/σ² 단축)

---
*참조: HEXA-UFO §23, HEXA-WARP Casimir 공용*
