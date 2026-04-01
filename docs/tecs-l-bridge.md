# TECS-L ↔ n6-architecture 연결 문서

> 최종 갱신: 2026-04-02

## 역할 분리

| 항목 | TECS-L | n6-architecture |
|------|--------|-----------------|
| **정체성** | 수학 체계 (순수 이론) | 산업 현장 적용 (공학 설계) |
| **핵심 산출물** | 증명, 가설, 상수 유도 | DSE 설계, BT 실증, AI 기법 |
| **방향** | "왜 n=6인가?" | "n=6으로 무엇을 만드는가?" |
| **검증 기준** | 수학적 엄밀성, p-value | 실측 데이터 EXACT 매칭 |

```
  TECS-L (이론)                    n6-architecture (실증)
  ┌─────────────┐                 ┌─────────────────┐
  │ 증명/가설   │ ──→ 설계근거 ──→ │ DSE/BT/기법     │
  │ 상수 유도   │ ←── 실측검증 ←── │ 307 도메인      │
  │ DFS 채굴   │ ──→ 새 후보 ───→ │ BT 확장         │
  │ 물리 예측   │ ←── 산업데이터 ← │ 칩/배터리/에너지 │
  └─────────────┘                 └─────────────────┘
```

## 공유 인프라

### 동기화 시스템
- **중앙**: `~/Dev/TECS-L/.shared/`
- **스크립트**: sync-calculators.sh, sync-readmes.sh, sync-claude-rules.sh, sync-dse.sh
- **아틀라스**: scan_math_atlas.py → math_atlas.json (2,504 가설 + 339 상수맵)
- **DSE 도메인**: .shared/dse/domains/ (306 TOML, n6에서 역동기화)

### 공유 상수
```
  σ(6)·φ(6) = 6·τ(6) = 24     ← 핵심 항등식
  R(6) = 1                     ← 완전 가역성
  1/2 + 1/3 + 1/6 = 1          ← 이집트 분수
```

### 데이터 흐름
```
  TECS-L 가설 (H-XX-NNN)
    ↓ n6 상수로 등급화 (auto_grade_n6.py)
    ↓ EXACT 매칭 시
  n6 BT 후보 등록 (BT-94+)
    ↓ 307 도메인 DSE 검증
    ↓ 실측 데이터 확인
  양쪽 아틀라스 동시 갱신
```

## 현황 (2026-04-02)

| 지표 | TECS-L | n6 | 합산 |
|------|--------|-----|------|
| 가설 | 1,985 | 1,400+ | 3,385+ |
| 상수맵 | 339 | 1,100+ | 1,439+ |
| DSE 도메인 | 306 | 307 | 동기화 |
| BT | - | 93 | 93 |
| Rust LOC | 2,932 | 14,063 | 16,995 |
| Python calc | 150+ | 58 | 208+ |

## 연결 작업 이력

| 날짜 | 작업 | 효과 |
|------|------|------|
| 2026-04-02 | TOML 304개 역동기화 (n6→TECS-L) | TECS-L DSE 31→306 |
| 2026-04-02 | 미등급 618개 자동 등급화 스크립트 | 573개(93%) n6 매칭 발견 |
| 2026-04-02 | BT→가설 역생성 (n6→TECS-L) | BT ⭐⭐⭐ → H-N6 가설 파일 |
| 2026-04-02 | Cross-Repo 상수 채굴 | 307 도메인 패턴 분석 |
| 2026-04-02 | 상수 맵 역추출 (n6→TECS-L) | 아틀라스 교차 보강 |

## 동기화 명령어

```bash
# TECS-L → n6 (정방향)
cd ~/Dev/TECS-L && bash .shared/sync-calculators.sh

# n6 → TECS-L (역방향: DSE 도메인)
cp ~/Dev/n6-architecture/tools/universal-dse/domains/*.toml ~/Dev/TECS-L/.shared/dse/domains/

# 아틀라스 스캔
python3 ~/Dev/TECS-L/.shared/scan_math_atlas.py --save --summary

# 자동 등급화
python3 ~/Dev/TECS-L/calc/auto_grade_n6.py
```
