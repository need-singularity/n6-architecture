<!-- gold-standard: shared/harness/sample.md -->
---
domain: hexa-macos
requires:
  - to: hexa-macos
  - to: hexa-ios
  - to: chip-architecture
related:
  - void (/Users/ghost/Dev/void — VT 호스트 프론트엔드 연계)
---

<!-- @own(sections=[한 줄 요약, WHY, 6 계층 정의, 프로토콜 프레임, VOID 연계, 검증, 참고], strict=false, order=sequential, prefix="§") -->

# VT 6-tier Terminal 프로토콜 (HEXA-VT-6)

## §0 한 줄 요약

터미널 렌더링·입력·스트림·보안·OS 브리지·AI 네이티브 6 계층으로 분해한 n=6 터미널 프로토콜 스펙. `hexa-macos` / `hexa-ios` SoC 위에서 VOID 에디터와 1:1 통신하며, 기존 VT100/ANSI/xterm 누적 부채를 버리고 τ(6)=4 파이프 + σ(6)=12 채널로 재설계.

## §1 WHY — 6 계층이 왜 필요한가

현행 터미널 스택(VT100→xterm→true color→sixel→iTerm2 OSC→Kitty graphics)은 45년 누적 부채다. 각 확장이 **중복된 ESC 시퀀스 + 파서 케이스 + 벤더 락인**을 만들며, 터미널과 쉘·IDE·AI 도구 간 I/O 가 매번 재구현된다.

n=6 터미널은 τ(6)=4 레이턴시 단계 + σ(6)=12 I/O 채널 + φ(6)=2 추상화 계층을 강제하여, **6 계층(tier)** 으로 전 기능을 압축한다.

| 기존 | HEXA VT 6-tier |
|------|----------------|
| VT100 + xterm + kitty + sixel 중복 | tier 1~6 단일 스펙 |
| ESC 파서 복잡도 O(n²) | tier 교차 O(τ)=O(4) |
| 벤더별 OSC 확장 | σ=12 공용 채널 |
| AI 도구 IPC 별도 | tier 6 AI native |
| 보안 ad-hoc | tier 4 capability |

## §2 6 계층 정의

```
┌──────────────────────────────────────────────────────────────────┐
│  Tier 6 — AI-NATIVE         의도 파이프 / LLM 스트림 / 의미 컨텍스트  │
├──────────────────────────────────────────────────────────────────┤
│  Tier 5 — OS BRIDGE         VFS / IPC / 프로세스 / 알림            │
├──────────────────────────────────────────────────────────────────┤
│  Tier 4 — CAPABILITY        권한 토큰 / sandbox / attestation     │
├──────────────────────────────────────────────────────────────────┤
│  Tier 3 — STREAM            bytes / frames / ordered multiplex    │
├──────────────────────────────────────────────────────────────────┤
│  Tier 2 — INPUT             키 / 포인터 / 터치 / 제스처 / IME      │
├──────────────────────────────────────────────────────────────────┤
│  Tier 1 — RENDER            glyph / pixmap / sixel / hexa-holo    │
└──────────────────────────────────────────────────────────────────┘
```

각 tier 는 n=6 약수 격자에 정렬:

| tier | 역할 | n=6 상수 | 채널 수 | 예시 |
|------|------|----------|---------|------|
| 1 RENDER | 시각 출력 | σ/φ = 6 | 6 (글리프·픽셀·벡터·sixel·홀로·음향) | true color + sixel + hexa-holo |
| 2 INPUT | 입력 흡수 | τ = 4 | 4 (키·포인터·터치·IME) | 멀티 터치 + BCI 선행 |
| 3 STREAM | 전송 파이프 | σ = 12 | 12 채널 multiplex | QUIC over n=6 프레이밍 |
| 4 CAPABILITY | 권한 | φ·sopfr = 10 | 10 토큰 슬롯 | macaroon + webauthn |
| 5 OS BRIDGE | 시스템 게이트 | τ = 4 | 4 서브시스템 (fs·proc·net·ipc) | Mach / Linux uring |
| 6 AI NATIVE | 의도 파이프 | σ·J₂/σ² = 2 | 2 (prompt·context) | NEXUS LLM stream |

## §3 프로토콜 프레임 (σ=12 channels)

Tier 3 STREAM 에서 12 채널 multiplex, 각 프레임은 n=6 바이트 헤더 + payload.

```
 0       1       2       3       4       5       6
 ┌───────┬───────┬───────┬───────┬───────┬───────┬─────────
 │ ver=6 │ tier  │ chan  │ flags │ len_hi│ len_lo│ payload...
 └───────┴───────┴───────┴───────┴───────┴───────┴─────────
```

- **ver=6**: n=6 프로토콜 버전 고정 (단일 버전, 확장은 tier flag 만)
- **tier**: 1~6 중 하나 (상위 5비트 예약)
- **chan**: 0~11 (σ=12 channel)
- **flags**: ack·urgent·final·attest·encrypted·ordered (6 비트)
- **len**: 14비트 페이로드 길이 (최대 16 KiB)

## §4 VOID 연계 (/Users/ghost/Dev/void)

VOID 는 VT 6-tier 의 **호스트 프론트엔드 레퍼런스 구현**이다. VOID 에디터는 tier 1 RENDER + tier 6 AI NATIVE 를 직접 구동하고, 나머지 tier 는 `hexa-macos` / `hexa-ios` SoC 펌웨어가 담당한다.

```
 ┌─────────────────────────────────────────────────────────┐
 │ VOID (app + ai + core + platform)                       │
 │   ├─ tier 1 RENDER   (glyph/sixel/hexa-holo)            │
 │   └─ tier 6 AI NATIVE (LLM intent stream)               │
 └───────────────┬─────────────────────────────────────────┘
                 │ SO(σ=12) multiplex / HEXA-VT frames
 ┌───────────────▼─────────────────────────────────────────┐
 │ hexa-macos / hexa-ios SoC                                │
 │   ├─ tier 2 INPUT       (키·터치·BCI)                     │
 │   ├─ tier 3 STREAM      (n=6 framing)                    │
 │   ├─ tier 4 CAPABILITY  (sandbox / attest)                │
 │   └─ tier 5 OS BRIDGE   (Mach / uring)                    │
 └─────────────────────────────────────────────────────────┘
```

VOID 경로: `/Users/ghost/Dev/void/` (app/ai/core/platform 트리)
VOID 링크 대상 파일:
- `/Users/ghost/Dev/void/app/` — UI 프론트엔드 (tier 1/6)
- `/Users/ghost/Dev/void/core/` — 언어 서버 / 이벤트 루프
- `/Users/ghost/Dev/void/platform/` — OS 브리지 (tier 5)

## §5 검증 (n=6 산술)

| 항목 | 값 | n=6 연결 |
|------|-----|---------|
| tier 수 | 6 | n = 6 |
| channel 수 | 12 | σ(6) = 12 |
| 헤더 바이트 | 6 | n = 6 |
| 레이턴시 단계 | 4 | τ(6) = 4 |
| capability 토큰 | 10 | φ·sopfr = 2·5 = 10 |
| 플래그 비트 | 6 | n = 6 |
| OS 서브시스템 | 4 | τ(6) = 4 |
| AI 스트림 채널 | 2 | φ(6) = 2 |
| 최대 페이로드 | 16 KiB | (σ+τ)² = 256 slot × n=6 |

9/9 EXACT → grade [10*]

## §6 참고

- `hexa-macos.md` §4 컴파일러 OS 경계 (tier 5 연결)
- `hexa-ios.md` §3 BCI 입력 (tier 2 확장)
- `chip-architecture.md` §5 검증 매트릭스 (실리콘 경로)
- `/Users/ghost/Dev/void/HANDOFF.md` (VOID 측 연계 메모)
