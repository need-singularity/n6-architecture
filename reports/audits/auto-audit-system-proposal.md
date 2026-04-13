# 주간 감사 자동화 시스템 설계 제안서

> **대상 프로젝트**: n6-architecture
> **작성일**: 2026-04-11
> **담당 축**: `reports/sessions` + `reports/changelogs`
> **설계 범위**: 지난 7일 범위 전방위 감사 → 자동 리포트 + 체인지로그 + 스냅샷

---

## 1. 배경 및 목표

### 1.1 배경

현재 `reports/sessions`, `reports/audits`, `reports/changelogs`, `reports/discovery` 4개 축은 **전부 수동 작성**이다.

- `reports/sessions/` 에는 `explosive-growth-2026-04-10.md`, `millennium-lemmas-2026-04-11.md` 등 수동 세션 리포트만 존재
- `reports/changelogs/` 에는 `3d-map-changelog-2026-04-08.md` 같은 피처별 체인지로그만 존재
- 주간 단위 (weekly) 감사 리포트는 **포맷이 통일되지 않았고 자동화도 없음**

### 1.2 목표

1. **지난 7일** 범위 전방위 감사 자동 생성
2. **한글 리포트** (box-drawing 표 포함)
3. **멱등 실행** (중복 실행 시 안전, 원본 보존)
4. **HEXA-FIRST** — 새 스크립트는 `.hexa` 로만 작성
5. **R14 준수** — shared SSOT 직접 참조, 하드코딩 금지
6. **체인지로그 템플릿 자동 골격** 생성
7. cron/hook 로 매주 자동 실행 가능

---

## 2. 집계 대상 데이터 소스

| # | 데이터 소스 | 경로 | 집계 방식 |
|---|------------|------|----------|
| 1 | **git log** | 로컬 `.git` | `git log --since='7 days ago'` — 커밋/저자/파일/9축 분포 |
| 2 | **growth_bus.jsonl** | `n6shared/growth_bus.jsonl` | awk 로 `ts` 필드 cutoff 이후 라인 수 + 타입/도메인 TOP |
| 3 | **discovery_graph.json** | `n6shared/discovery_graph.json` | `"id":` / `"from":` 출현 빈도로 노드/엣지 근사 → 스냅샷 대비 델타 |
| 4 | **atlas.n6** | `/Users/ghost/Dev/nexus/shared/n6/atlas.n6` | grep `[10*]`, `[10]`, `[9]`, `[7]`, `[N?]`, `[N!]` 등급 집계 |
| 5 | **cargo test** | `nexus/` 워크스페이스 | `timeout 180 cargo test --quiet --offline` 후 `test result: ok. X passed` 파싱 |
| 6 | **convergence** | `n6shared/convergence/n6-architecture.json` | awk 로 `ossified`/`stable`/`failed` 블록 키 수 집계 |

### 2.1 스냅샷 기반 델타

- `n6shared/logs/weekly_audit_state.json` — 이전 실행 시 저장한 지표
- 필드: `dg_nodes`, `dg_edges`, `atlas_exact`, `atlas_empirical`, `gb_total`, `updated`
- 매 실행마다 갱신 → 다음 주 실행 시 델타 계산 기준

---

## 3. 산출물 구조

### 3.1 주간 감사 리포트

**경로**: `reports/sessions/{YYYY-MM-DD}-weekly-audit.md`

**포맷 요약**:
```
# n6-architecture 주간 감사 리포트 — 2026-04-11

## 요약 스냅샷 (box-drawing)
## 1. Git 활동 (커밋/저자/파일/9축 분포/최근 커밋 12건)
## 2. growth_bus.jsonl (타입 TOP / 도메인 TOP)
## 3. discovery_graph.json (노드/엣지 델타)
## 4. atlas.n6 등급 집계
## 5. cargo test 결과
## 6. convergence 상태
## 7. 권장 후속 액션 (자동 규칙 기반)
```

### 3.2 체인지로그 템플릿

**경로**: `reports/changelogs/{YYYY-MM-DD}-weekly.md`

- 자동 집계 블록: 수치 채움
- 수동 보완 블록: `추가/수정/삭제/수렴/다음주 TODO` 빈 섹션 생성

### 3.3 스냅샷 JSON

**경로**: `n6shared/logs/weekly_audit_state.json`

```json
{
  "updated": "2026-04-11",
  "dg_nodes": 342,
  "dg_edges": 1205,
  "atlas_exact": 1542,
  "atlas_empirical": 38,
  "gb_total": 18425,
  "since_days": 7
}
```

### 3.4 템플릿 원본 (새로 생성됨)

- `reports/templates/weekly-audit-template.md`
- `reports/templates/changelog-template.md`

---

## 4. 멱등성 설계

| 시나리오 | 동작 |
|---------|------|
| 첫 실행 | `{DATE}-weekly-audit.md` 생성 |
| 동일일 재실행 (기본) | `{DATE}-weekly-audit_rerun-HHMM.md` 생성 (원본 보존) |
| `--force` 지정 | 원본 덮어쓰기 |
| `--dry-run` | stdout 출력만, 파일 작성 없음 |
| 스냅샷 JSON | 실행 시마다 최신 값으로 갱신 (항상) |

**안전 장치**:
- R19 SILENT EXIT 금지 → 모든 `exec()` 에러는 `log()` 로 stderr 출력
- R21 블로킹 금지 → `cargo test` 는 `timeout 180` 래핑
- R22 인터프리터 경로 → `bash` 만 사용, python/node 미호출

---

## 5. 스크립트 구조 (`nexus/scripts/weekly_audit.hexa`)

```
main
├── CLI 파싱 (--dry-run, --force, --no-cargo, --since N)
├── phase 1/6  git log 집계
├── phase 2/6  growth_bus.jsonl 스캔 (awk cutoff)
├── phase 3/6  discovery_graph.json 델타 (스냅샷 대비)
├── phase 4/6  atlas.n6 등급 집계 (grep)
├── phase 5/6  cargo test (옵션)
├── phase 6/6  convergence 상태 집계
├── render      md 본문 조립 (box-drawing 표 + 섹션별)
├── write       reports/sessions/*.md + changelogs/*.md + state.json
└── stdout 요약 출력
```

**총 라인 수**: 약 600줄 (기존 HEXA 포팅 규모 대비 중간 수준)

---

## 6. 실행 방법

### 6.1 수동 실행

```sh
cd /Users/ghost/Dev/n6-architecture
hexa nexus/scripts/weekly_audit.hexa                   # 이번 주 감사
hexa nexus/scripts/weekly_audit.hexa --dry-run         # 파일 작성 없이 미리보기
hexa nexus/scripts/weekly_audit.hexa --force           # 덮어쓰기
hexa nexus/scripts/weekly_audit.hexa --since 14        # 14일 범위
hexa nexus/scripts/weekly_audit.hexa --no-cargo        # cargo test 스킵
```

### 6.2 cron 등록 (매주 월요일 09:00)

```sh
# ~/.crontab 또는 crontab -e
0 9 * * 1 cd /Users/ghost/Dev/n6-architecture && /Users/ghost/Dev/hexa-lang/hexa nexus/scripts/weekly_audit.hexa >> n6shared/logs/weekly_audit_cron.log 2>&1
```

### 6.3 macOS launchd (권장 — cron 대체)

`~/Library/LaunchAgents/com.n6arch.weekly-audit.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.n6arch.weekly-audit</string>
    <key>ProgramArguments</key>
    <array>
        <string>/bin/bash</string>
        <string>-c</string>
        <string>cd /Users/ghost/Dev/n6-architecture && /Users/ghost/Dev/hexa-lang/hexa nexus/scripts/weekly_audit.hexa</string>
    </array>
    <key>StartCalendarInterval</key>
    <dict>
        <key>Weekday</key>
        <integer>1</integer>
        <key>Hour</key>
        <integer>9</integer>
        <key>Minute</key>
        <integer>0</integer>
    </dict>
    <key>StandardOutPath</key>
    <string>/Users/ghost/Dev/n6-architecture/n6shared/logs/weekly_audit_launchd.log</string>
    <key>StandardErrorPath</key>
    <string>/Users/ghost/Dev/n6-architecture/n6shared/logs/weekly_audit_launchd.err</string>
</dict>
</plist>
```

로드:
```sh
launchctl load ~/Library/LaunchAgents/com.n6arch.weekly-audit.plist
launchctl list | grep weekly-audit
```

### 6.4 Claude Code hook 등록 (선택 — SessionStart 등)

`.claude/settings.json` 에 추가 (기존 `UserPromptSubmit` 블록과 병행):

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "cd /Users/ghost/Dev/n6-architecture && /Users/ghost/Dev/hexa-lang/hexa nexus/scripts/weekly_audit.hexa --dry-run 2>&1 | head -30",
            "timeout": 15
          }
        ]
      }
    ]
  }
}
```

**주의**: `.claude/settings.json` 은 L0 보호 대상일 수 있음 — 직접 편집 전 `core-lockdown.json` 확인 및 사용자 승인 필요 (R25 준수).

**대안**: 프로젝트 로컬 `n6shared/config/` 수준 등록 (L2 자유 수정)
- `n6shared/config/weekly_audit_schedule.json` 생성
- `nexus` 데몬이 주기적으로 해당 config 읽어 실행 (nexus_growth_daemon.hexa 확장)

---

## 7. 규칙 준수 체크리스트

| 규칙 | 준수 여부 | 비고 |
|------|----------|------|
| R1 HEXA-FIRST | ✅ | `.hexa` 만 신규 생성 |
| R2 하드코딩 금지 | ✅ | 경로는 상대, 상수는 변수 |
| R5 SSOT | ✅ | `shared/` 원본 참조, 중복 생성 없음 |
| R6 자동 기록 | ✅ | `n6shared/logs/weekly_audit_state.json` 에 집계 결과 영구 보존 |
| R8 데이터 로컬 금지 | ✅ | `n6shared/logs/` 는 허용 범위 (nexus 내부 통합) |
| R14 공통 JSON | ✅ | 규칙은 기존 `absolute_rules.json` 참조만 |
| R18 미니멀 | ✅ | 감사 집계에만 집중, 기능 추가 없음 |
| R19 SILENT EXIT 금지 | ✅ | 모든 에러는 `log()` 로 stderr 출력 |
| R21 블로킹 금지 | ✅ | `cargo test` 타임아웃 180s |
| R22 인터프리터 경로 | ✅ | `bash` 만, python/node 호출 없음 |
| R24 n6shared/hooks 신규 .sh/.py 금지 | ✅ | 이 스크립트는 `nexus/scripts/` 에 생성 |
| 한글 필수 | ✅ | 모든 출력/주석 한글 |

---

## 8. 확장 로드맵

### Phase 1 (즉시, 이 제안서 범위)
- [x] `nexus/scripts/weekly_audit.hexa` 작성
- [x] `reports/templates/weekly-audit-template.md` + `changelog-template.md`
- [x] `reports/audits/auto-audit-system-proposal.md` (본 문서)

### Phase 2 (단기 — 1주 이내)
- [ ] 첫 실행 (`hexa nexus/scripts/weekly_audit.hexa --dry-run`) 검증
- [ ] cron 또는 launchd 등록
- [ ] 기존 세션 리포트 패턴과 정합성 확인

### Phase 3 (중기 — 1개월)
- [ ] atlas 승격 상세 추적 ([7]→[10*] 개별 항목 기록)
- [ ] papers/ 축 편수 변화 추가
- [ ] NEXUS-6 스캔 anomaly 통합 (nexus scan --full 결과)
- [ ] 월간 감사 (`monthly_audit.hexa`) 추가

### Phase 4 (장기)
- [ ] 감사 결과를 `n6shared/convergence/n6-architecture.json` 자동 업데이트 (stable→ossified 전이 감지)
- [ ] 다른 프로젝트 (TECS-L, anima, sedi, papers) 크로스 감사 (nexus 허브 경유)

---

## 9. 리스크 및 완화

| 리스크 | 영향 | 완화 |
|-------|------|------|
| `atlas.n6` grep 근사 오차 | 등급별 카운트 ±소수 | 정확한 섹션 파서는 Phase 3에서 도입 |
| `cargo test` 180s 초과 | 감사 멈춤 | `--no-cargo` 옵션 + 타임아웃 래핑 |
| `discovery_graph.json` 구조 변경 | 노드 카운트 부정확 | Phase 3에서 정규 JSON 파서로 교체 |
| 첫 실행 델타 = 현재 절대값 | 초기값 노이즈 | 문서에 명시 + 2회차 이후 정상 |
| `n6shared/logs/weekly_audit_state.json` 손상 | 델타 0 | `--force` 재실행 + 수동 복구 문서화 |

---

## 10. 결론

본 제안서는 `nexus/scripts/weekly_audit.hexa` 라는 **단일 HEXA 스크립트**로 n6-architecture 프로젝트의 주간 감사를 자동화한다. 6개 데이터 소스(git/growth_bus/discovery_graph/atlas.n6/cargo/convergence)를 **멱등 안전**하게 집계하여 한글 리포트 + 체인지로그 템플릿 + 스냅샷 JSON 3종을 생성한다.

실행 방식은 **수동 > cron > launchd > Claude hook** 순으로 점진 도입이 가능하며, 전 단계에서 `--dry-run` 으로 안전 검증할 수 있다.

R1(HEXA-FIRST), R14(shared SSOT), R19(SILENT EXIT 금지), R21(블로킹 금지), 한글 필수 등 모든 절대 규칙을 준수한다.

---

*본 제안서는 `reports/audits` 축에 속하며, 승인 후 Phase 2(실행 검증) 로 진행 가능하다.*
