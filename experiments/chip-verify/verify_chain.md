# 검증 체인: chip-3d → smr-datacenter → digital-twin

- 작성일: 2026-04-14
- 로드맵: CHIP-P1-3
- 트리거: `experiments/chip-verify/verify_chip-3d.hexa`
- 피드백 대상: `n6shared/config/dse-map.toml` ([chip-3d.feedback], [smr-datacenter.feedback], [digital-twin.feedback], [cross-dse.chip-3d-x-smr-x-twin])

## §1 체인 개요

```
┌──────────────────────────────────────────────────────────────────────┐
│  [hop 1] chip-3d         │  HEXA-3D-CHIP n=6 산술 정렬 5축 검증       │
│         │                │  → 메탈 6, SM σ²=144, MAC σ·J₂=288,        │
│         │                │     파이프 τ=4, 전원 σ-τ=8                  │
│         ▼                │                                            │
│  [hop 2] smr-datacenter  │  SMR 6 모듈 데이터센터 — 전력/열 인터페이스 │
│         │                │  → 1/2+1/3+1/6 Egyptian × n=6 모듈          │
│         ▼                │                                            │
│  [hop 3] digital-twin    │  τ=4 시뮬 단계 × σ=12 채널 × J₂=24 센서 폭  │
│                          │  → 검증값 트윈 모니터에 반영                │
└──────────────────────────────────────────────────────────────────────┘
```

## §2 hop 별 산출 인터페이스

### hop 1 — chip-3d 검증

| 항목 | 측정 | n=6 수식 | 판정 |
|------|------|---------|------|
| 메탈 레이어 | 6 | n | EXACT |
| SM 배열 | 144 | σ² | EXACT |
| MAC 어레이 | 288 | σ·J₂ | EXACT |
| 파이프 단 | 4 | τ | EXACT |
| 전원 도메인 | 8 | σ-τ | EXACT |

총합 5/5 EXACT → 다운스트림 호출 권한 획득.

### hop 2 — smr-datacenter 인수

- 입력: chip-3d 의 σ-τ=8 전원 도메인 + Egyptian 1/2+1/3+1/6 분배
- 매핑: SMR 6 unit 모듈 ↔ n=6 데이터센터 토폴로지
- 결과: SMR 모듈 1 개당 chip-3d 클러스터 σ²/n = 24 SM 공급, 6 unit × 24 = 144 SM 완전 일치

### hop 3 — digital-twin 입수

- 입력: smr-datacenter 의 전력/열 시계열 + chip-3d 의 144 SM 가용성
- 매핑: τ=4 시뮬 스테이지 × σ=12 트윈 채널 × J₂=24 센서 폭
- 결과: 트윈이 chip-3d 검증값을 5축 모두 EXACT 로 모니터 — 드리프트 발생 시 dse-map 의 chip-3d.feedback 갱신 신호 발행

## §3 피드백 메커니즘

```
verify_chip-3d.hexa  ──pass=5/5──>  dse-map.toml [chip-3d.feedback]
                                       │
                                       ├─> [smr-datacenter.feedback] (chain_position=2)
                                       │
                                       └─> [digital-twin.feedback] (chain_position=3)
                                              │
                                              └─> [cross-dse.chip-3d-x-smr-x-twin]
```

- 갱신 트리거: 도메인별 `verified_at` 필드 비교
- 회귀: 어느 hop 이라도 fail 시 전체 체인 status = "regression" 으로 전환
- 추가 검증: hop 2/3 는 본 1차 라운드에서 chip-3d.hexa 결과를 직접 인수 (정적 검증). 동적 hexa 스텁은 P1-4 단계에서 추가 예정.

## §4 BT 연결

| BT | 연결점 |
|----|-------|
| BT-28 | 캐시 계위 Egyptian — chip-3d ↔ smr-datacenter 전력 분배 |
| BT-56 | GPU 산술 σ²=144 SM — chip-3d ↔ digital-twin 자원 모델 |
| BT-1414 | A1 검증 인증 체인 — 본 chain 의 인증 메타 |

## §5 다음 단계

- P1-3 종료 (이 문서 작성 + dse-map feedback 3 entry append)
- P1-4 진입 시: smr-datacenter / digital-twin 각각 verify_*.hexa 동적 스텁 추가
