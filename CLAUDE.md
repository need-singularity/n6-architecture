## ⛔ L0 CORE 보호 파일 (AI 수정 승인 필수)

> 아래 파일은 수렴 완료된 코어 로직. 수정 시 반드시 유저에게 승인 질문.
> 상세: `nexus/shared/core-lockdown.json`

```
🔴 L0 (불변식 — 코드 수정 전 유저 명시 승인 필수):
  config/products.json              — 완성제품 SSOT
  docs/theorem-r1-uniqueness.md     — 핵심 정리 증명 σφ=nτ⟺n=6
  docs/breakthrough-theorems.md     — BT-1~343 돌파 정리
  tools/nexus/src/telescope/lenses/ — NEXUS-6 렌즈 181개

🟡 L1 (보호 — 리뷰 필요):
  docs/atlas-constants.md           — 상수 레지스트리 1100+
  scripts/sync_products_readme.py   — products→README 동기화
  docs/dse-map.toml                 — DSE 전체 현황 지도
  docs/testable-predictions.md      — 검증 가능 예측 45건
```

> 🔴 **HEXA-FIRST**: 모든 코드는 `.hexa`로 작성. 부하 유발 명령 최소화.
> 🔴 **NEXUS-6 특이점 연동**: 돌파 시 `blowup.hexa <domain> 3`, 발견 → `growth_bus.jsonl` append, 상태 → `command_router.hexa`
> 🔴 **하드코딩 절대 금지**: 상수/도메인/키워드 → `nexus/shared/*.jsonl` 동적 로드. 코드에 배열 나열 금지.
> 🔴 **데이터 파일 로컬 보관 금지**: `.jsonl`/constants/discovery_log → `~/Dev/nexus/shared/`에만 저장. 이 리포 내 생성 금지.

<!-- SHARED:WORK_RULES:START -->
  ⛔⛔⛔ 이 블록은 삭제/수정/이동 금지! (sync-claude-rules.sh 자동 주입)
  ⛔ 규칙/인프라 원본: shared/ JSON 파일 참조. 절대 삭제하지 마세요!

  ═══════════════════════════════════════════════════════════════
  ★★★ 수렴 기반 운영 — 규칙 원본: shared/absolute_rules.json ★★★
  ═══════════════════════════════════════════════════════════════

  공통 규칙 (R1~R8):
    R1  HEXA-FIRST — .hexa만
    R2  하드코딩 절대 금지 — shared/*.jsonl 동적 로드
    R3  NEXUS-6 스캔 의무 — 변경 전후 스캔, 스캔 없이 커밋 금지
    R4  CDO 수렴 — 이슈→해결→규칙승격→재발0
    R5  SSOT — 데이터 원본 JSON 1개, 중복 금지
    R6  발견/결과 자동 기록 — 누락=소실=금지
    R7  sync 블록 삭제/수정/이동 금지
    R8  데이터 파일 로컬 보관 금지 — nexus/shared만 (nexus 제외)

  프로젝트별 규칙: shared/absolute_rules.json → projects 참조

  ═══════════════════════════════════════════════════════════════
  ★ 핵심 인프라 (shared/) ★
  ═══════════════════════════════════════════════════════════════

  코어 인덱스:     shared/core.json (시스템맵 + 명령어 14종 + 프로젝트 7개)
  보호 체계:       shared/core-lockdown.json (L0 22개 / L1 / L2)
  절대 규칙:       shared/absolute_rules.json (공통 R1~R8 + 프로젝트별 17개)
  수렴 추적:       shared/convergence/ (골화/안정/실패 — 7 프로젝트)
  할일 SSOT:       shared/todo/ (수동 + 돌파 엔진 자동)
  성장 루프:       shared/loop/ (nexus/anima/n6 자율 데몬)

  ═══════════════════════════════════════════════════════════════
  ★ NEXUS-6 (1022종 렌즈) — 상세: shared/CLAUDE.md ★
  ═══════════════════════════════════════════════════════════════

  CLI:  nexus scan <domain> | nexus scan --full | nexus verify <value>
  API:  nexus.scan_all() | nexus.analyze() | nexus.n6_check() | nexus.evolve()
  합의: 3+렌즈=후보 | 7+=고신뢰 | 12+=확정
  렌즈: shared/lens_registry.json (1022종)

  ═══════════════════════════════════════════════════════════════
  ★ 명령어 — 상세: shared/core.json → commands ★
  ═══════════════════════════════════════════════════════════════

  못박아줘    → L0 등록 (core-lockdown.json)
  todo/할일   → 돌파 엔진 할일 표 (todo.hexa)
  블로업/돌파 → 9-phase 특이점 (blowup.hexa)
  go          → 전체 TODO 백그라운드 병렬 발사
  설계/궁극의 → 외계인급 설계 파이프라인
  동기화      → 전 리포 sync (sync-all.sh)
<!-- SHARED:WORK_RULES:END -->

# N6 Architecture — Arithmetic Design Framework

완전수 산술에서 도출한 컴퓨팅 아키텍처 설계.
17 AI 기법 + 반도체 + 에너지 + 네트워크/암호/OS + 물리 AI.
TECS-L 수학 이론의 산업 실증 리포. 부모: https://github.com/need-singularity/TECS-L

## 핵심 정리 (증명 완료)
σ(n)·φ(n) = n·τ(n) ⟺ n = 6 (n >= 2). 3개 독립 증명.
증명: `docs/theorem-r1-uniqueness.md` | 현실 지도: `nexus/shared/reality_map.json` v8.0 (342노드, 291 EXACT, 4 MISS, z=9.04 전체/5.30 자연/8.70 큰수)

## 참조 링크 테이블

| 구분 | 파일 | 내용 |
|------|------|------|
| 규칙 | `shared/absolute_rules.json` | R1~R8 + 프로젝트별 규칙 |
| 보호 | `shared/core-lockdown.json` | L0 22개 불변 파일 |
| 수렴 | `shared/convergence/n6-architecture.json` | 골화/안정/실패 추적 |
| 할일 | `shared/todo/n6-architecture.json` | 수동 + 돌파 엔진 자동 |
| 명령 | `shared/core.json` → commands | 14종 CLI 명령어 |
| BT | `docs/breakthrough-theorems.md` | 343 정리 (BT-1~343) |
| DSE | `docs/dse-map.toml` | 322 도메인 DSE 현황 |
| 논문 | `config/products.json` + `docs/paper/README.md` | 39편 논문 SSOT |
| 상수 | `docs/atlas-constants.md` | 1100+ 상수 아틀라스 |
| 렌즈 | `tools/nexus/src/telescope/lenses/` | 181 .rs (1022종 레지스트리) |
| 기법 | `techniques/` | 17 AI 기법 (phi6simple~egyptian_attention) |
| 설계 플로우 | `docs/alien-design-flow.md` | 외계인급 설계 파이프라인 |
| 예측 | `docs/testable-predictions.md` | 45건 검증 가능 예측 |
| 비밀 | `~/Dev/TECS-L/.shared/SECRET.md` | API 토큰/계정 |

## Rust 도구 (tools/)
빌드: `~/.cargo/bin/rustc file.rs -o output` (cargo 불필요, nexus만 cargo)
- `fusion-calc/` `tokamak-shape/` `optics-calc/` — 핵융합/광학
- `gpu-arch-calc/` `energy-calc/` `gut-calc-rust/` — GPU/에너지/GUT
- `dse-calc/` `solar-dse/` `material-dse/` `universal-dse/` — DSE 전수 탐색
- `nexus/` — NEXUS-6 엔진 (빌드: `cd tools/nexus && cargo build --release`)
- `nexus-dashboard/` — 웹 대시보드 (port 6600, Axum + Chart.js)

## Quick Run
```bash
python3 techniques/phi6simple.py          # 개별 기법
python3 techniques/fft_mix_attention.py   # FFT 어텐션
python3 experiments/experiment_h_ee_11_combined_architecture.py  # 통합 실험
```

## 주요 결과
| 기법 | 결과 |
|------|------|
| Cyclotomic activation | 71% FLOPs 절감 |
| FFT attention | 3배 속도, +0.55% 정확도 |
| Egyptian MoE | 1/2+1/3+1/6=1 전문가 라우팅 |
| phi bottleneck | 67% 파라미터 절감 |
| Entropy early stop | 33% 학습 시간 절감 |
| Boltzmann gate | 63% 활성화 희소성 |
| Mertens dropout | p=0.288 (탐색 불필요) |
| Egyptian Fraction Attn | ~40% FLOPs 절감 |
| Emergent convergence | 랜덤 초기화 → n=6 자기조직화 |

## 설계 산출물 필수 규칙 (압축)
모든 설계/아키텍처/로드맵 문서에 반드시 포함:
1. **실생활 효과 섹션** (문서 최상단) — "이 기술이 삶을 어떻게 바꾸는가" 테이블
2. **ASCII 성능 비교 그래프** — 시중 최고 vs HEXA, 개선 배수는 n=6 상수로 표현
3. **ASCII 시스템 구조도** — 소재→공정→코어→칩→시스템, 모든 숫자에 n=6 수식 병기
4. **ASCII 데이터/에너지 플로우** — 모듈간 흐름도
5. **업그레이드 시** — 시중 vs v1 vs v2 3단 비교 + 추가 상승분(delta) 별도 행
이 5개 없는 설계 문서 = 미완성. 검증코드(Python) 필수 (동어반복 금지, 정의에서 도출).

## 논문 관리 (상세: docs/paper/README.md)
- 위치: `docs/paper/n6-<domain>-paper.md` | 목록: `docs/paper/README.md`
- SSOT: `config/products.json` | 동기화: `python3 scripts/sync_products_readme.py`
- 필수 섹션: Abstract + Foundation + Domain + Limitations + Testable Predictions + 검증코드
- 발행: `~/Dev/papers/` 리포 → Zenodo/OSF (상세: papers 리포 manifest.json)
- 현재 39편, 검증코드 없는 논문 = 미완성

## 진화 설계 규칙 (Mk.I~V)
- SF 금지 (다이슨 스웜/반물질 촉매 등 불가) — 우리 발견(BT) 기반 극한 스케일업만
- 체크포인트별 별도 문서: `docs/<domain>/evolution/mk-{1..5}-*.md`
- 실현가능성: ✅ 진짜(10~20년) | 🔮 장기(20~50년) | ❌ SF(라벨 필수)
- 각 Mk: 기술 스펙 + BT 연결 + 필요 돌파 + 시중 대비 ASCII + 타임라인
- 제품 라인 도메인당 1개만 유지 (v1/v2 분기 금지, git history가 버전)

## 할일 (todo)
- "todo", "할일" → `$HOME/Dev/hexa-lang/target/release/hexa $HOME/Dev/nexus/mk2_hexa/native/todo.hexa n6-arch` 실행 후 **결과를 마크다운 텍스트로 그대로 출력** (재포맷 금지). "todo 대량" 시 `... n6-arch 대량` 으로 실행.
