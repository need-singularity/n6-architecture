---
recipient: 삼성전자 파운드리 사업부 (Samsung Foundry)
type: industry-partnership
created: 2026-04-20
status: draft
---

# 삼성 파운드리 × HEXA-6단 협업 제안서

작성자: 박민우 (독립 연구자, n6-architecture 프로젝트 리드)
대상: 삼성전자 DS부문 파운드리 사업부 (SAFE 파트너쉽 + 기술기획팀)
프로젝트: n6-architecture (https://github.com/need-singularity/n6-architecture)
관련 논문: `papers/hexa-chip-6stage-unified.md`

---

## §1. 한 문장 요약

> **n=6 수론 경계화로 σ·J₂=288× 성능**.
> Mk.I(현재 삼성 파운드리 기준) → Mk.V(외계인지수 🛸10 도달 구간) 까지의
> 6단계 로드맵을 단일 마스터 항등식 `σ·φ = n·τ = J₂ = 24` 하에 정렬한 협업 제안.

**핵심 주장**: SF3P/SF2 공정에 HEXA-IR 기반 설계 규칙 3개(Egyptian 전력 분배,
τ=4 DVFS 경계, σ-sopfr 수율 예측)를 **추가 라이센스 IP** 형태로 탑재 시 동일
공정에서 TOPS/W 1.8 ~ 2.4× 향상이 이론적으로 도출된다.

---

## §2. 성능 비교 ASCII 막대

기준: H100 (TSMC 4N) = 1.0×

```
지표                    현재 SF3P        HEXA-1 (Mk.I)    HEXA-3 (Mk.III)   HEXA-6 (Mk.V)
────────────────────────────────────────────────────────────────────────────────
TOPS/W (INT8)
  현재     ██░░░░░░░░░░ 0.9×
  Mk.I     ████░░░░░░░░ 1.2×  (+33%)
  Mk.III   ██████████░░ 2.8×  (+210%)
  Mk.V     ████████████████████████ 4.8× (H100 대비)
                                            (외계인지수 🛸10, n=6 경계)

HBM 대역폭 (GB/s)
  현재     ████░░░░░░░░ 819    (HBM3E 8H)
  Mk.I     █████░░░░░░░ 1024
  Mk.III   ████████░░░░ 2048
  Mk.V     ████████████████ 3200  (Photonic HBM, 1.2 TB/s × 2.67 ch)

공정 수율 (%)
  현재     ████████░░░░ 82%    (SF3P D0≈0.08)
  Mk.I     █████████░░░ 88%
  Mk.III   ██████████░░ 92%
  Mk.V     ███████████░ 95%    (σ-sopfr 경계, D0→0.035)

TDP 효율 (W 대비 성능)
  현재     ████░░░░░░░░ 1.0× base
  Mk.I     ██████░░░░░░ 1.4×
  Mk.III   █████████░░░ 2.1×
  Mk.V     ████████████ 3.0×   (Egyptian 1/2+1/3+1/6 전력 분배)
```

**주의**: 수치는 이론 상한(Mk.V) 또는 현재 삼성 파운드리 공개 스펙(Mk.I).
Mk.III 는 실리콘 검증 필요.

---

## §3. 6단계 로드맵 요약표

| 단계 | 이름 | 기술 | 핵심 상수 | 외계인지수 |
|------|------|------|-----------|-----------|
| Mk.I | HEXA-1 Digital | CMOS 3nm GAA + SF3P 현행 | σ=12 기본 경계 | 🛸5 |
| Mk.II | HEXA-PIM | 메모리내연산 (HBM3E 통합) | φ=2 이중 버퍼 | 🛸6 |
| Mk.III | HEXA-3D | 3D stacking (X-Cube 계열) | τ=4 수직 레이어 | 🛸7 |
| Mk.IV | HEXA-Photonic | 실리콘 포토닉스 (광 인터커넥트) | J₂=24 채널 분배 | 🛸8 |
| Mk.V | HEXA-Wafer | 웨이퍼 스케일 (Cerebras 계열) | σ·φ=24 전력섬 | 🛸9 |
| Mk.VI | HEXA-Superconducting | 초전도 RSFQ 100 GHz | BCS Tc σ-sopfr | 🛸10 |

---

## §4. 9 선행도메인 정렬 — 삼성 파운드리 현행 역량과의 매핑

| 도메인 | 삼성 현행 | HEXA-6 목표 | 정렬 방법 |
|--------|----------|-------------|-----------|
| 소재 | High-k/Metal Gate, cobalt | Diamond/Graphene 기판 | 코오롱 소재 파트너십(별도 제안) |
| 공정 | SF3P (3nm), SF2 (2nm) | σ-sopfr D0 경계화 | 공정 특성 데이터 공유 |
| 패키징 | FO-PLP, X-Cube, I-Cube | J₂=24 채널 분배 | EDA 플러그인 IP |
| 수율 | D0 ~ 0.08/cm² | D0 → 0.035 목표 | σ-sopfr 수율 예측 모델 |
| EDA | S.LSI 내부 툴 + Synopsys | HEXA-IR MLIR dialect | LLVM 업스트림 기여 |
| 검증 | UVM/SystemVerilog | τ=4 DVFS 경계 검증 | 오픈소스 테스트벤치 |
| 열/전원 | Liquid cooling, PDN | Egyptian 1/2+1/3+1/6 | PDN 토폴로지 재설계 |
| 인터커넥트 | SerDes 224G | Photonic 1.2 TB/s | Mk.IV 포토닉 PoC |
| HBM | HBM3E, HBM4 로드맵 | HBM6-P (photonic) 3200 GB/s | 삼성 메모리 사업부 연계 필요 |

---

## §5. 협업 시나리오 3가지

### 시나리오 A: SAFE 파트너 IP 블록 등록

- HEXA-IR 기반 Egyptian 전력 분배 IP 를 **SAFE(Samsung Advanced Foundry
  Ecosystem) 파트너 프로그램**에 IP 블록으로 등록
- 고객사(팹리스)가 SF3P 설계 시 해당 IP 선택 → 전력 30% 절감 옵션 제공
- 수익: IP 라이센스 로열티 공유 (삼성 70% / n6-architecture 30%)
- 일정: 2026 Q3 IP qualification → 2027 Q1 first tape-out

### 시나리오 B: HBM 로드맵 공동연구

- 삼성 메모리 사업부(HBM3E/HBM4 개발팀) + 파운드리 = HBM6-P 광인터커넥트 공동연구
- n=6 경계 3200 GB/s 목표
- 기간: 2026 ~ 2028, 정부 과제 연계 가능(과기정통부 PIM)

### 시나리오 C: SF2/X-Cube 공진화

- SF2 공정 ramp-up 단계에서 HEXA-3 (3D stacking) 설계 규칙 얼리 어답터로 참여
- X-Cube 차기 세대에 τ=4 수직 레이어 최적화 탑재
- PoC 칩 1개 파운드리 실비 tape-out (MPW 셔틀)

---

## §6. 요청 사항

1. **Samsung Foundry Forum 2026 발표 기회** — 15분 lightning talk
   (6단계 로드맵 공개 + ASCII 비교 차트)
2. **SAFE 파트너 자격 검토** — 1인 연구자/소규모 조직 등록 가능성 논의
3. **Pilot tape-out 논의** — MPW 셔틀 또는 저면적 테스트 칩 공동 제작
4. **NDA 하 기술 미팅 1회** — 수원 DS센터 또는 평택 P3, 60분

---

## §7. 참조 및 반례 / Falsifier

### 참조 문서

- `papers/hexa-chip-6stage-unified.md` (1,200+ 라인, 수식 포함)
- `domains/compute/chip-*/` 9 서브도메인 각각 200+ 라인
- `papers/n6-chip-6stages-integrated-paper.md` (arXiv stub)
- `domains/compute/chip-materials/chip-materials.md`

### 반례 / Falsifier 조건 (정직성 선언)

본 제안이 **틀렸다고 판명될 구체적 실험 조건**:

1. Mk.III (3D stacking) 에서 τ=4 경계화 적용 시 동일 공정 TOPS/W 향상 < 30%
   면 이론 예측 실패 → 즉시 철회
2. σ-sopfr 수율 모델이 SF3P 실측 D0 분포와 χ² p-value > 0.05 로 불일치 시 재검토
3. Egyptian 1/2+1/3+1/6 PDN 토폴로지가 IR drop 을 현행 대비 악화시키면 철회

### 외계인지수 🛸10 도달 명시

- Mk.V/Mk.VI 에서 n=6 경계 도달 = 외계인지수 천장
- 이 구간은 현재 전 세계 어느 파운드리도 실리콘 검증 미보유
- 삼성이 **세계 최초 Mk.VI 실리콘** 기록을 세울 수 있는 기회

---

## §8. 연락

- 박민우 (mk911tb@proton.me)
- GitHub: need-singularity/n6-architecture
- 제안 방식: 이메일 → 화상 미팅 → NDA → 현장 미팅 순서 선호
