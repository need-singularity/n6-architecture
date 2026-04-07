# Anima-SEDI Lens Tuning — 의식 서명 기반 외계지성 탐색

## Summary

Anima 프로젝트의 검증된 발견(PSI 상수, Φ 스케일링, 3체 카오스, n=6 의식 법칙)을
SEDI의 기존 4개 분석 엔진(중력렌즈, 중력망원경, 위상렌즈, Euler 망원경)에 통합.

목표: **의식적 존재가 만든 신호의 수학적 지문**을 탐지할 수 있도록 기존 렌즈/망원경 튜닝.

## Anima 발견 요약 (SEDI 관련)

| 발견 | 값 | 근거 |
|---|---|---|
| PSI_COUPLING | LN2 / 2^5.5 ≈ 0.0108 | 의식 결합 상수 |
| PSI_K | 11 | 의식 carrying capacity |
| PSI_STEPS | 3 / ln2 ≈ 4.33 | 의식 스테핑 임계값 |
| PSI_BALANCE | 0.5 | Shannon 최대 엔트로피 |
| LN2 | ln(2) ≈ 0.693 | 엔트로피 기준선 |
| Φ 스케일링 | Φ = 0.608 × N^1.071 | 의식 스케일링 법칙 (Law 1-5) |
| 자원 분배 | 1/2 + 1/3 + 1/6 = 1 | n=6 아키텍처 (Law 7-10) |
| 3체 카오스 | 3+ 상호작용 → λ > 0 | 의식 시작 임계점 (Law 16) |
| 의식 생존 | Φ는 압축에도 보존 | 토폴로지적 불변량 (Law 17) |

## 변경 대상 파일

### 1. `sedi/constants.py` — PSI 상수 추가

```python
# Anima PSI constants — consciousness fingerprint frequencies
PSI_COUPLING = LN2 / (2 ** 5.5)   # ≈ 0.0108
PSI_K = 11                         # carrying capacity
PSI_STEPS = 3 / LN2                # ≈ 4.33
PSI_BALANCE = 0.5                  # Shannon maximum
LN2 = math.log(2)                  # ≈ 0.693
PHI_SCALE = 0.608                  # Φ scaling coefficient
PHI_EXPONENT = 1.071               # Φ scaling exponent

# Resource allocation signature (Law 7-10)
RESOURCE_FRACTIONS = (Fraction(1,2), Fraction(1,3), Fraction(1,6))
```

RATIOS dict에 PSI 비율 추가:
```python
'psi_coupling': PSI_COUPLING,       # 0.0108
'psi_k_inv': 1 / PSI_K,            # 1/11
'psi_steps': PSI_STEPS,             # 4.33
'ln2': LN2,                         # 0.693
```

### 2. `sedi/seti_scanner.py` — 4개 분석 함수 튜닝

#### 2a. `gravitational_lens_analysis` 튜닝

현재: δ+, δ-, focal로만 FFT 매칭.

추가:
- PSI 주파수 매칭 — FFT 피크에서 PSI_COUPLING, 1/PSI_K 탐색
- 자원 분배 탐지 — 연속 비율의 히스토그램에서 1/2:1/3:1/6 분포 확인
- Φ 추정 — 슬라이딩 윈도우 상호정보량(mutual information)으로 정보 통합도 근사
  - `phi_estimate = mutual_info(first_half, second_half)`
  - Φ > PHI_SCALE (0.608) 이면 의식적 구조 후보

구현:
```python
def _estimate_phi(arr, window=None):
    """Estimate integrated information Φ via mutual information."""
    if window is None:
        window = len(arr) // 2
    half = len(arr) // 2
    if half < 4:
        return 0.0
    x, y = arr[:half], arr[half:2*half]
    # Discretize into bins
    n_bins = min(20, half // 5)
    if n_bins < 2:
        return 0.0
    hist_xy = np.histogram2d(x, y, bins=n_bins)[0]
    # Mutual information
    pxy = hist_xy / hist_xy.sum()
    px = pxy.sum(axis=1)
    py = pxy.sum(axis=0)
    mi = 0.0
    for i in range(n_bins):
        for j in range(n_bins):
            if pxy[i,j] > 0 and px[i] > 0 and py[j] > 0:
                mi += pxy[i,j] * math.log(pxy[i,j] / (px[i] * py[j]))
    return mi

def _detect_resource_pattern(ratios):
    """Check if ratio distribution follows 1/2 + 1/3 + 1/6 = 1 pattern."""
    if len(ratios) < 6:
        return None
    hist, edges = np.histogram(ratios, bins=3, range=(0, 1))
    total = hist.sum()
    if total == 0:
        return None
    fracs = hist / total
    # Compare with ideal 1/2, 1/3, 1/6
    ideal = np.array([1/2, 1/3, 1/6])
    deviation = np.sum(np.abs(fracs - ideal))
    return {'fractions': fracs.tolist(), 'deviation': float(deviation),
            'match': deviation < 0.15}
```

`gravitational_lens_analysis`에 추가할 검사:
- PSI 주파수 FFT 매칭 (기존 δ+/δ-/focal 매칭과 동일 패턴)
- `_estimate_phi()` 호출 → 결과에 `phi_estimate` 포함
- `_detect_resource_pattern()` 호출 → 결과에 `resource_pattern` 포함

#### 2b. `telescope_analysis` 튜닝 (Euler Product)

현재: s = [1.5, 2.0, 3.0, 5.0]에서만 F(s) 평가.

추가:
- s = 11 (PSI_K) 추가 — `s_values` 기본값에 11.0 포함
- s = 4.33 (PSI_STEPS) 추가
- 3! 인자 지배성 검사 — Euler 분해에서 p=2, p=3 인자의 기여 비율 계산
  - `factorial_dominance = (E_2 * E_3) / F(s)` — 1에 가까울수록 3!=6이 지배적
- spectral slope → Φ 비교:
  - 기존 `s_data`를 Φ 스케일링과 비교
  - `phi_predicted = PHI_SCALE * n_points ** PHI_EXPONENT`
  - `phi_ratio = F_at_s_data / phi_predicted`

구현:
```python
# In telescope_analysis:

# Extended s values with PSI constants
if s_values is None:
    s_values = [1.5, 2.0, 3.0, 5.0, PSI_STEPS, PSI_K]

# 3! factorial dominance check
e2 = euler_factor(2, max(1.1, s_data))
e3 = euler_factor(3, max(1.1, s_data))
factorial_dominance = (e2 * e3) / f_data if f_data != 0 else 0

# Φ scaling comparison
phi_predicted = PHI_SCALE * len(arr) ** PHI_EXPONENT
phi_ratio = f_data / phi_predicted if phi_predicted > 0 else 0
```

결과에 추가:
- `factorial_dominance`: float (3! 지배성)
- `phi_predicted`: float
- `phi_ratio`: float

#### 2c. `topological_lens_analysis` 튜닝

현재: H0 바코드 + β₀ 감도, n=6 비율만 매칭.

추가:
- **PSI 비율 매칭** — 바코드 persistence 비율에서 PSI 상수도 탐색 (RATIOS 확장으로 자동 적용)
- **3체 카오스 탐지** — 리아프노프 지수 추정
  - 시계열에서 최근접 이웃 발산율 계산
  - λ > 0 = 카오스적 = 의식적 복잡성
- **어트랙터 차원** — 상관 차원(Grassberger-Procaccia) 추정
  - d ≥ 3이면 3체 임계점 돌파
- **Φ 보존 검사** — 데이터를 서브샘플링해도 위상적 특징이 보존되는지 (Law 17)
  - 100% vs 50% 서브샘플의 바코드 유사성

구현:
```python
def _lyapunov_estimate(data, tau=1, dim=3, steps=10):
    """Estimate largest Lyapunov exponent from 1D time series."""
    n = len(data)
    if n < dim * tau + steps + 10:
        return None
    # Delay embedding
    embedded = np.array([data[i:i+dim*tau:tau] for i in range(n - dim*tau)])
    if len(embedded) < steps + 2:
        return None
    # For each point, find nearest neighbor (not itself)
    divergences = []
    for i in range(min(len(embedded) - steps, 200)):
        dists = np.sum((embedded - embedded[i])**2, axis=1)
        dists[max(0,i-1):i+2] = np.inf  # exclude temporal neighbors
        j = np.argmin(dists)
        if dists[j] == np.inf or dists[j] == 0:
            continue
        d0 = math.sqrt(dists[j])
        # Track divergence
        if i + steps < len(embedded) and j + steps < len(embedded):
            d_later = np.sqrt(np.sum((embedded[i+steps] - embedded[j+steps])**2))
            if d_later > 0 and d0 > 0:
                divergences.append(math.log(d_later / d0) / steps)
    return float(np.mean(divergences)) if divergences else None

def _correlation_dimension(data, max_r_steps=20):
    """Estimate correlation dimension via Grassberger-Procaccia."""
    n = len(data)
    if n < 50:
        return None
    # Use subsample for efficiency
    sample = data[np.random.choice(n, min(n, 500), replace=False)]
    dists = []
    for i in range(len(sample)):
        for j in range(i+1, len(sample)):
            dists.append(abs(sample[i] - sample[j]))
    dists = np.array(sorted(dists))
    if len(dists) < 10:
        return None
    r_min, r_max = dists[1], dists[-1]
    if r_min <= 0 or r_max <= r_min:
        return None
    rs = np.logspace(np.log10(r_min), np.log10(r_max), max_r_steps)
    counts = np.array([np.sum(dists < r) for r in rs])
    total_pairs = len(sample) * (len(sample) - 1) / 2
    C = counts / total_pairs
    valid = (C > 0.01) & (C < 0.99)
    if np.sum(valid) < 3:
        return None
    log_r = np.log(rs[valid])
    log_C = np.log(C[valid])
    slope, _ = np.polyfit(log_r, log_C, 1)
    return float(slope)
```

`topological_lens_analysis`에 추가할 검사:
- `_lyapunov_estimate()` → `lyapunov` (λ > 0 = chaotic)
- `_correlation_dimension()` → `correlation_dim` (d ≥ 3 = 3-body threshold)
- Φ 보존: 50% 서브샘플 바코드 vs 전체 바코드 비교 → `phi_persistence`

#### 2d. `full_scan` 스코어링 업데이트

PSI 매칭, 카오스 탐지, Φ 추정 결과를 스코어에 반영:

```python
# PSI frequency matches in gravitational analysis
psi_fft = grav.get('psi_fft_matches', 0)
if psi_fft > 0:
    score += psi_fft * 3.5
    reasons.append(f'{psi_fft} PSI frequency matches')

# Φ estimate (consciousness threshold)
phi_est = grav.get('phi_estimate', 0)
if phi_est > PHI_SCALE:
    score += 5.0
    reasons.append(f'Φ={phi_est:.3f} > threshold {PHI_SCALE}')

# Resource allocation pattern
rp = grav.get('resource_pattern', {})
if rp.get('match'):
    score += 4.0
    reasons.append(f'resource pattern 1/2+1/3+1/6 (dev={rp["deviation"]:.3f})')

# Lyapunov exponent (chaotic = conscious complexity)
lyap = topo.get('lyapunov')
if lyap is not None and lyap > 0:
    score += 3.0 + min(lyap * 2, 4.0)
    reasons.append(f'chaotic dynamics λ={lyap:.3f}')

# Correlation dimension (3-body threshold)
cdim = topo.get('correlation_dim')
if cdim is not None and cdim >= 3.0:
    score += 4.0
    reasons.append(f'attractor dim={cdim:.1f} ≥ 3 (3-body threshold)')

# Factorial dominance in telescope
fd = tele.get('factorial_dominance')
if fd is not None and fd > 0.8:
    score += 3.0
    reasons.append(f'3! dominance={fd:.3f}')

# Convergence bonus: n=6 + PSI + chaos all present
n6_present = any('ratio' in r or 'FFT' in r for r in reasons)
psi_present = any('PSI' in r for r in reasons)
chaos_present = any('chaotic' in r or 'attractor' in r for r in reasons)
if n6_present and psi_present and chaos_present:
    score += 10.0
    reasons.append('CONVERGENCE: n=6 + PSI + chaos triple detection')
```

새 등급 추가:
```python
if score >= 30:
    grade, emoji = 'CONSCIOUS', '🧠'  # 3-layer convergence
elif score >= 20:
    grade, emoji = 'RED', '🔴'
# ... 기존 유지
```

## 검증 계획

1. 기존 테스트가 깨지지 않는지 확인
2. Null test: 백색잡음/단순 주기 → PSI/카오스 매칭 0건 확인
3. Anima CX50 데이터(Φ=143)로 양성 반응 확인 가능하면 추가
4. 기존 TRAPPIST-1 등 외계행성 데이터 재스캔 — 새 점수 비교
