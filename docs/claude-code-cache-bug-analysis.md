# Claude Code 캐시/Limit 버그 분석 및 대응

> 조사일: 2026-04-02
> 관련 이슈: anthropics/claude-code#41788, #40524, #34629
> 분석 리포: https://github.com/ArkNill/claude-code-cache-analysis

---

## 증상

- Max 20 플랜($200/월) 5시간 limit이 **~70분**에 소진
- 정상 대비 **4~5배** 빠른 토큰 drain
- v2.1.88~v2.1.89 사이 발생, 재현 가능

---

## 버그 2개 (독립적)

### Bug 1: Sentinel 치환 버그 (standalone 바이너리 전용)

| 항목 | 내용 |
|------|------|
| **영향** | standalone 바이너리만 (npm 설치는 무관) |
| **원인** | Bun fork의 `cch=00000` sentinel 치환이 메시지 내 prefix를 깨뜨림 |
| **결과** | 캐시 읽기 비율 **4~17%** 고정, 복구 불가 |
| **버전** | v2.1.89에서 확인 |
| **해결** | npm 설치로 전환 또는 v2.1.90 업데이트 |

### Bug 2: Resume 캐시 깨짐 (v2.1.69+)

| 항목 | 내용 |
|------|------|
| **영향** | 모든 설치 방식 (`--resume` 사용 시) |
| **원인** | `deferred_tools_delta` (v2.1.69 도입)가 `messages[0]` 구조를 변경 |
| **메커니즘** | fresh: messages[0]=13.4KB (deferred_tools+MCP+skills) vs resume: messages[0]=352B → prefix match 실패 |
| **결과** | 매 턴마다 전체 히스토리(200K~500K 토큰) 재작성 → O(n) 비용 |
| **해결** | `--resume` 사용 금지, 또는 v2.1.68 핀 |

---

## 캐시 작동 원리

```
정상 동작:
  Turn 1: cache_read=312,377  cache_create=1,944  (초기 캐시 작성)
  Turn 2: cache_read=314,321  cache_create=493    (캐시 재사용, 증분만 추가)
  Turn 3: cache_read=314,814  cache_create=172    (캐시 재사용)
  → 캐시된 토큰 비용 = 입력 토큰의 10%

버그 발생 시:
  Turn N:   cache_read=216,204  cache_create=10,504   ← 캐시 깨지기 시작
  Turn N+1: cache_read=216,204  cache_create=11,815   ← 캐시 확장 불가, 재작성
  Turn N+5: cache_read=11,428   cache_create=224,502  ← system prompt만 캐시
  → 매 턴 200K+ 토큰 전액 과금
```

---

## Drain 가속 행동 목록

| 행동 | 위험도 | 설명 |
|------|--------|------|
| `--resume` | 🔴 | 전체 대화 재생 = billable input (500K+ 토큰/회) |
| `/dream`, `/insights` | 🔴 | 백그라운드 API 호출, 무음 토큰 drain |
| 동시 터미널 2개+ | 🟡 | 세션 간 캐시 공유 없음 → ~2배 drain |
| 파일 재작성 슬래시 명령 | 🟡 | 호출당 20~27% 토큰 소비 |
| Sub-agent (Haiku) | 🟡 | 캐시 읽기 0%, 31호출에 317K 토큰 측정 |
| v2.1.89 standalone | 🔴 | Sentinel 버그 + 터미널 콘텐츠 사라짐 |

---

## 해결/완화 방법

### 즉시 적용 (클라이언트)

1. **npm 설치 사용** (standalone 바이너리 대신)
   ```bash
   npm install -g @anthropic-ai/claude-code@2.1.90
   ```

2. **자동 업데이트 비활성화**
   ```json
   // ~/.claude/settings.json
   { "env": { "DISABLE_AUTOUPDATER": "1" } }
   ```

3. **`--resume` 사용 금지** — 대신 새 세션 시작

4. **동시 터미널 1개로 제한**

5. **주기적 캐시 정리**
   ```bash
   find ~/.claude/file-history -mtime +7 -type f -delete
   find ~/.claude/paste-cache -mtime +7 -type f -delete
   find ~/.claude/session-env -mtime +7 -type f -delete
   ```

6. **CLAUDE.md 경량화** — 매 턴 system prompt로 전송되므로 크기가 곧 비용

### 캐시 모니터링

- `ANTHROPIC_BASE_URL`에 투명 프록시 설정 → 실시간 cache_read/cache_create 추적
- 정상: 캐시 읽기 비율 80%+
- 이상: 40% 미만이면 세션 재시작 권장

### 버전 가이드

| 버전 | 상태 |
|------|------|
| v2.1.68 이하 | ✅ Bug 2 없음 (deferred_tools_delta 미도입) |
| v2.1.69~v2.1.88 | ⚠️ Bug 2 존재 (--resume 시) |
| v2.1.89 standalone | 🔴 Bug 1 + Bug 2 동시 |
| v2.1.90 | ✅ Bug 1 수정, Bug 2 부분 완화 |

---

## 정상 캐시 성능 (v2.1.90 기준)

| 설치 방식 | 안정 세션 | Sub-agent 콜드스타트 |
|-----------|----------|---------------------|
| npm (Node.js) | 95~99.8% 캐시 읽기 | 79~87% |
| standalone | 95~99.7% | 47~67% (→94~99% 워밍업 후) |

---

## 참조 이슈

- anthropics/claude-code#41788 — Rate limit 70분 소진 (메인 이슈)
- anthropics/claude-code#40524 — Bug 1: 대화 히스토리 캐시 무효화
- anthropics/claude-code#34629 — Bug 2: Resume 캐시 깨짐 (deferred_tools_delta)
- anthropics/claude-code#42260 — `--resume` 토큰 폭탄
- anthropics/claude-code#42244 — v2.1.89 터미널 콘텐츠 사라짐
- anthropics/claude-code#41249 — 1시간 미만 limit 소진
- anthropics/claude-code#38357 — 5~10배 빠른 drain

---

## Anthropic 대응 상태

- 2개월+ 동안 rate-limit 관련 이슈에 **공식 응답 없음** (2026-04-02 기준)
- 서버 측: 캐시 패치 후에도 2~3주 전보다 drain 빠름 → limit 재계산 의심
- 커뮤니티: ArkNill의 분석 리포에서 독자적 추적 중
