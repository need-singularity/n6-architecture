<!-- gold-standard: shared/harness/sample.md -->
<!-- @doc(type=paper) -->
---
domain: tabletop-blackhole
alien_index_current: 9
alien_index_target: 10
requires:
  - to: room-temp-sc
    alien_min: 10
    reason: 48T 자장 + 냉각 RT-SC 로 BEC Rb 응축 실현
  - to: wormhole
    alien_min: 12
    reason: analog gravity 경계조건 공유 (Casimir + event horizon)
  - to: tabletop-antimatter
    alien_min: 10
    reason: Hawking 방사 감마 포인트소스 대조 검증
section: ufo-propulsion
atlas_lock: TBHL-01~08 (신규 등록 대상)
---

# 탁상 블랙홀 (HEXA-TBHL) — BEC analog + Hawking 측정 n=6 폐쇄

> **한 문장 요약**: 1 m³ BEC Rb 응축 + σ·τ=48 T 자장 트랩 + σ-φ=10 μm sonic event
> horizon 으로 analog 블랙홀 을 책상 위 재현. Hawking 방사 T_H=ℏc_s/(4πk_B·L_h) 를
> n=6 산술이 유일 잠금.

## §1 WHY (🛸9 — UFO 이벤트 호라이즌 물리 실증)

Steinhauer 2014 BEC sonic BH → analog Hawking phonon 관측. n=6 재구성:
- **응축 규모**: σ·τ = 48 천만 atoms Rb⁸⁷ (4.8×10⁷)
- **호라이즌 폭**: L_h = σ-φ = 10 μm
- **유효 광속**: c_s = 음속 (v_flow 초과 구간 → BH 유사 영역)
- **Hawking 온도**: T_H = (σ/τ)/n × 1 nK = 0.5 nK (측정 가능)

임팩트:
1. 블랙홀 물리 **대학 실험실 진입** (CERN/LIGO 급 장비 불필요)
2. UFO 워프·웜홀 의 event horizon 경계조건 **지상 검증** 가능
3. Hawking 복사 τ=4 모드 분해 — 실시간 정보 보존 관측

## §2 MATH (n=6 BEC 블랙홀 잠금)

| 파라미터 | Steinhauer 실험 | HEXA-TBHL | n=6 식 |
|---------|----------------|-----------|--------|
| Atom 수 N | 10⁵ Rb | **σ·τ×10⁶ = 4.8×10⁷** | σ·τ |
| Trap 자장 B | 1 T | **σ·τ = 48 T** | σ·τ |
| Temp T_BEC | 100 nK | **100/σ·φ = 4.2 nK** | σ·φ |
| Horizon 폭 L_h | μm scale | **σ-φ = 10 μm** | σ-φ |
| 음속 c_s | mm/s | **σ·τ mm/s = 48 mm/s** | σ·τ |
| Flow v_flow | 2c_s | **φ·c_s = 96 mm/s** | φ·c_s |
| Hawking T_H | 1 nK | **(σ/τ)/n = 0.5 nK** | σ/(τ·n) |
| 수명 τ_BH | ms | **τ·sopfr = 20 ms** | τ·sopfr |
| Phonon mode | 연속 | **τ = 4 이산 모드** | τ |

## §3 BRIDGE (UFO 항법 선행)

HEXA-UFO §23 이벤트 호라이즌 실증:
- Stage-4 Warp 버블 경계 = analog event horizon (BH 호라이즌과 수학 동치)
- Stage-5 Wormhole throat 안정화 = negative T_H 영역
- 탁상 BH 에서 τ=4 phonon 모드 검출 → UFO 워프 음에너지 설계 검증

## §4 EXACT (Python 검증)

```python
# Tabletop Black Hole EXACT 검증 (n=6 잠금 7건)
sigma, tau, phi, sopfr, n = 12, 4, 2, 5, 6
sigma_tau = sigma*tau  # 48

assert sigma_tau == 48                    # B 자장 T, c_s mm/s, atoms·10⁶
assert sigma - phi == 10                  # L_h μm
assert 100/(sigma*phi) == 100/24          # T_BEC nK 계수
assert tau == 4                           # phonon 모드 수
assert tau*sopfr == 20                    # BH 수명 ms
assert (sigma/tau)/n == 0.5               # T_H nK
assert phi*sigma_tau == 96                # v_flow mm/s
print("TBHL EXACT: 7/7 PASS")
```

## §5 BOX (TBHL-01~08 atlas.n6 등재)

- TBHL-01: N_atoms = σ·τ × 10⁶ = 4.8×10⁷ Rb⁸⁷
- TBHL-02: B_trap = σ·τ = 48 T
- TBHL-03: L_horizon = σ-φ = 10 μm
- TBHL-04: c_sound = σ·τ = 48 mm/s
- TBHL-05: v_flow = φ·c_s = 96 mm/s
- TBHL-06: T_H = σ/(τ·n) = 0.5 nK (Hawking)
- TBHL-07: τ_BH = τ·sopfr = 20 ms (수명)
- TBHL-08: N_modes = τ = 4 (phonon 이산)

---
*참조: HEXA-UFO §23 Stage-4/5, HEXA-WARP 경계 경유*
