# NEXUS-6 Web Dashboard Design

## Overview
NEXUS-6 자율성장 시스템의 브라우저 기반 모니터링 + 제어 대시보드.
`~/.nexus6/` 데이터를 시각화하고, 데몬 제어 및 실시간 로그 스트리밍 제공.

## Scope
- **모니터링**: 15차원 성장 타임라인, 최근 성장 이벤트, 벤치마크
- **제어**: 데몬 시작/정지, 수동 성장 트리거, 차원 우선순위 조정
- **로그**: 실시간 SSE 로그 스트리밍
- **Out of scope**: 렌즈 스캔 실행, OUROBOROS 진화 트리거, 외부 접근

## Tech Stack
- **Server**: Rust + Axum (async, SSE native)
- **Frontend**: Single HTML file + Chart.js + vanilla JS
- **Data source**: `~/.nexus6/*.json`, `tools/nexus6/scripts/growth_log.jsonl`
- **Port**: localhost:6600

## Layout

```
┌─────────────────────────────────────────────────┐
│  NEXUS-6 Growth Monitor        [●Live] [⏸Stop]  │
├─────────────────────────────────────────────────┤
│                                                  │
│  15차원 성장 타임라인 차트 (Chart.js line)        │
│  X=시간, Y=성장값, 15 라인 (토글 가능)            │
│                                                  │
├─────────────────────────────────────────────────┤
│  최근 성장 (시간순 ↓)                            │
│  시각 | 차원 | 이전→현재 | Δ | 상세              │
├─────────────────────────────────────────────────┤
│  실시간 로그 (SSE)                [Clear][Pause] │
│  > [14:32] growth cycle #47 started...           │
└─────────────────────────────────────────────────┘
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Dashboard HTML (embedded, single file) |
| GET | `/api/dimensions` | 15차원 현재값 + 목표값 + 달성률 |
| GET | `/api/history` | 타임라인 데이터 (growth_log.jsonl 파싱) |
| GET | `/api/recent` | 최근 성장 이벤트 목록 (시간 역순) |
| GET | `/api/benchmark` | 최신 벤치마크 결과 |
| GET | `/api/logs/stream` | SSE 실시간 로그 스트리밍 |
| POST | `/api/daemon/start` | 데몬 프로세스 시작 |
| POST | `/api/daemon/stop` | 데몬 프로세스 정지 |
| GET | `/api/daemon/status` | 데몬 PID + uptime + 상태 |
| POST | `/api/grow/:dimension` | 특정 차원 수동 성장 트리거 |

## Data Sources

### `~/.nexus6/` 파일 매핑
- `growth_system_arch.json` → 시스템 구조 메타데이터
- `benchmark-results.json` → 벤치마크 (scan/evolve/verify 시간)
- `benchmark_history.jsonl` → 벤치마크 히스토리
- `domain_completeness.json` → 도메인별 완성도
- `discovery_graph.json` → 디스커버리 그래프
- `engine_fitness.json` → 엔진 적합도
- `strategy.json` → 성장 전략 상태

### `tools/nexus6/scripts/`
- `growth_log.jsonl` → 성장 이벤트 타임라인 (메인 데이터)
- `growth_dashboard.sh` 내 targets dict → 15차원 목표값 참조

## Server Architecture

```
main.rs
├── routes/
│   ├── dashboard.rs    — GET / (HTML 임베딩 서빙)
│   ├── dimensions.rs   — GET /api/dimensions
│   ├── history.rs      — GET /api/history
│   ├── recent.rs       — GET /api/recent
│   ├── benchmark.rs    — GET /api/benchmark
│   ├── logs.rs         — GET /api/logs/stream (SSE)
│   └── daemon.rs       — POST start/stop, GET status, POST grow
├── data/
│   ├── reader.rs       — ~/.nexus6/ JSON 파일 읽기
│   └── targets.rs      — 15차원 목표값 정의
└── static/
    └── index.html      — Dashboard (Chart.js + vanilla JS)
```

## 15 Growth Dimensions (targets from registry.rs)

| Dimension | Target | Description |
|-----------|--------|-------------|
| performance | 10,000 | 스캔 성능 (ops/sec) |
| architecture | 100 | 아키텍처 갭 해결 수 |
| lenses | 200 | 구현된 렌즈 수 |
| modules | 4.0 | 평균 모듈 성숙도 |
| tests | 1,000 | 테스트 수 |
| hypotheses | 150 | 검증된 가설(BT) 수 |
| dse | 322 | DSE 도메인 수 |
| experiments | 50 | 실험 수 |
| calculators | 50 | 계산기 수 |
| cross_resonance | 100 | 교차공명 발견 수 |
| knowledge_graph | 500 | 지식그래프 노드 수 |
| red_team | 100 | 레드팀 검증 수 |
| atlas | 2,000 | 아틀라스 항목 수 |
| documentation | 90 | 문서 커버리지 % |
| integration | 50 | 통합테스트 수 |

## Daemon Control

- **Start**: `bash tools/nexus6/scripts/nexus6_growth_daemon.sh --max-cycles 999 --interval 30 --skip-commit &`
- **Stop**: PID from `~/.nexus6/daemon.pid` or `ps aux | grep nexus6_growth_daemon`
- **Status**: PID 존재 + `/proc/PID` 또는 `kill -0 PID` 확인
- **Manual grow**: `bash tools/nexus6/scripts/auto_grow.sh --dimension <dim> --cycles 1`

## Frontend Details

- Chart.js line chart: 15 라인, 각 차원별 색상, 범례 클릭 토글
- 최근 성장 테이블: 10초 폴링, 새 항목 하이라이트 애니메이션
- 로그 패널: SSE로 실시간, auto-scroll, 최대 500줄 유지
- 다크 테마 (NEXUS-6 브랜딩)
- 반응형 불필요 (로컬 데스크탑 전용)

## Build & Run

```bash
cd tools/nexus6-dashboard
~/.cargo/bin/cargo build --release
./target/release/nexus6-dashboard
# → http://localhost:6600
```
