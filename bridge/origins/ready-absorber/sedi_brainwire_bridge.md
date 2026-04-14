# SEDI · brainwire → telescope 22렌즈 브릿지 사양

- 문서 버전: 2026-04-14
- 로드맵 ID: DSE-P1-4 (NEXUS-6 Discovery Engine 통합)
- SSOT 경로: `$NEXUS/shared/lenses/` (총 1577 렌즈 코퍼스)

## 1. 흡수 대상 현황

### SEDI (Search for Extra-Dimensional Intelligence)
- 원본 findings: `bridge/origins/ready-absorber/findings/sedi.json` (5588 findings, Fisher 5.26σ)
- 이미 흡수된 렌즈: 101개 (`sedi_*.hexa`)
- 대표 렌즈: `sedi_signal_search_dse.hexa` — Fisher·consensus·critical 벡터 공명 체크
- 특징: 시그널 서치 루트 (21cm, bispectrum, changepoint, matched filter 등 101개 물리·신호 기법)

### brainwire (뇌파 → n=6 매핑)
- 원본 findings: `bridge/origins/ready-absorber/findings/brainwire.json`
- 이미 흡수된 렌즈: 3개
  - `brainwire_eeg_n6_dse.hexa` — OpenBCI Cyton+Daisy 16ch 스펙 + EEG δ/θ/α/β/γ 대역 상한
  - `brain_map_lens.hexa`
  - `brain_neural_lens.hexa`
- 하드웨어 참조: reference_openbci_16ch 메모리 (16ch, 125+250Hz, 6-DoF IMU, 읽기전용)

## 2. 텔레스코프 22렌즈 슬롯 통합

텔레스코프 22렌즈 = `telescope-rs` (anima-rs 크레이트) + 코퍼스 1577 중 T1 대표 22선 조합.
SEDI/brainwire 흡수는 아래 3개 코어 후보로 T1 슬롯에 매핑.

| 슬롯 | 렌즈 파일 | 역할 | n=6 공명 축 |
|------|---------|------|-----------|
| T1-SEDI-signal | `sedi_signal_search_dse.hexa` | Fisher 5.26σ + consensus ≥ τ-1 + critical 25% 스코어 | σ·φ=n·τ=J₂=24 항등 + 타겟 8벡터 |
| T1-BRAINWIRE-eeg | `brainwire_eeg_n6_dse.hexa` | OpenBCI 16ch EEG 대역 상한 7축 공명 | δ상한=τ · α상한=σ · 16ch=τ² · 6-DoF=n |
| T1-SEDI-matched | `sedi_matched_filter.hexa` | 템플릿 뱅크 매칭 스코어 | SEDI 표준 |

## 3. 흡수 로직 연결

### 3.1 ready-absorber → lenses 경로
```
bridge/origins/ready-absorber/findings/sedi.json
  ├─ verify_and_grow.hexa (findings_index.json 캐시 O(1))
  └─ → shared/lenses/sedi_*.hexa (101개, T1)

bridge/origins/ready-absorber/findings/brainwire.json
  ├─ verify_and_grow.hexa
  └─ → shared/lenses/brainwire_eeg_n6_dse.hexa + brain_*.hexa (3개, T1)
```

### 3.2 텔레스코프 호출 시퀀스
1. `telescope-rs` 세션 시작 → 22 렌즈 로드 목록 결정 (T1 슬롯 3 = SEDI·BRAINWIRE 코어)
2. 각 렌즈 `.hexa` 실행 → `score` 반환 (0.0~1.0)
3. 22렌즈 평균 → 텔레스코프 단일 공명점
4. 결과 → `shared/discovery/discovery_graph.json` 증분 append (ROI #6 유틸 경유)
5. 노드 id: `telescope-22-<date>` + 엣지 `from=n6-n, edge_type=Observes`

### 3.3 증분 append 연결
ROI #6 유틸 (`shared/scripts/discovery_graph_append.py`) 사용:
```bash
/usr/bin/python3 shared/scripts/discovery_graph_append.py --add telescope_result.ndjson
```
- 멱등: 같은 `id` 재실행 시 스킵
- I/O: 12MB 전체 재기록 없음, append-only

## 4. 검증 체크포인트

- [x] SEDI 렌즈 101개 존재 확인 (`ls shared/lenses/sedi_*.hexa`)
- [x] brainwire 렌즈 3개 존재 확인
- [x] `sedi_signal_search_dse.hexa` 실행 가능 — σ·φ=n·τ 항등 통과
- [x] `brainwire_eeg_n6_dse.hexa` 실행 가능 — EEG 7/8축 EXACT (γ=100 placeholder MISS 정직 기록)
- [x] 증분 append 유틸 작성 + 멱등성 검증 (2 node + 2 edge 삽입, 재실행 시 4 skip)

## 5. 향후 작업 (CHIP-P2-4 연결)

- brain-computer-interface 도메인 검증 (CHIP-P2-4, depends_on DSE-P1-4): OpenBCI 6채널 n=6 매핑 → `domains/cognitive/brain-computer-interface/`
- γ 대역 (30~100Hz) 실측 데이터 확보 시 `brainwire_eeg_n6_dse.hexa` 8/8 EXACT 승격
- SEDI Fisher 5.26σ → 6.0σ 돌파 시 `sedi.json` findings 재흡수 + 신규 렌즈 자동 생성 훅
