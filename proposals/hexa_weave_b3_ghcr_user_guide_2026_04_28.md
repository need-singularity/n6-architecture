---
category: operational
date: 2026-04-28
parent_witness: design/kick/2026-04-28_docker-ghcr-cycle14-prep_omega_cycle.json
predecessor_proposals:
  - proposals/hexa_weave_kick_infra_b3_registry_2026_04_28.md (cycle 7 — Option 4 docker save/load + Option 3 ghcr.io)
  - proposals/hexa_weave_kick_infra_b3_ghcr_2026_04_28.md (cycle 10 — Option 3 5-step detail)
mission: F-DOCKER-REGISTRY-PUSH (B.3 ghcr.io) — 사용자 4항목 OK 3번 / 14d deadline / cycle 14 prep
status: PREP-COMPLETE — automation skeleton + 사용자 PAT 가이드 ready. 실 push 는 사용자 PAT 후에만.
deadline: 2026-05-12T00:00:00Z
---

# HEXA-WEAVE B.3 ghcr.io — 통합 spec + 사용자 PAT 발급 한국어 가이드

raw 9 hexa-only data path. raw 13 NO external comms. raw 91 C3 honest:
이 문서는 cycle 7 / cycle 10 두 plan 을 통합한 single-source-of-truth 이며,
사용자 PAT 발급 전까지 actual push 0 (raw 91 C3 — TTY 의존).

## §0 통합 요약 (cycle 7 + cycle 10 + cycle 14 prep)

| 항목 | cycle 7 plan | cycle 10 plan | cycle 14 prep (이 문서) |
|------|--------------|----------------|--------------------------|
| 채널 | Option 4 (save/load) 즉시 + Option 3 (ghcr.io) 장기 | Option 3 5-step detail | Option 3 자동화 + 사용자 가이드 + audit ledger |
| 실행 | 미실행 | 미실행 | 미실행 (PAT 대기) |
| 산출 | 4 옵션 비교표 | 5 step exact command | tool/ghcr_push.hexa + jsonl ledger |
| F-71 | F-B3-REG-1..5 | F-B3-GHCR-1..5 | F-B3-GHCR-PREP-1..5 |

## §1 사용자 PAT 발급 한국어 가이드 (raw 65 minimal-blast-radius)

### §1.1 Step 1 — GitHub PAT 발급

1. 브라우저에서 <https://github.com/settings/tokens> 열기
   - **Tokens (classic)** 선택 (fine-grained 는 GHCR write 일부 경로 미지원)
2. **Generate new token (classic)** 클릭
3. **Note**: `hexa-runner ghcr push (cycle 14)` 같이 식별 가능 명칭
4. **Expiration**: **90 days** (raw 65 최소 blast radius — 1y/no-expiration 금지)
5. **scopes** 체크:
   - [x] `write:packages`
   - [x] `read:packages`
   - [ ] `delete:packages` (cycle 16+ image GC 필요할 때만)
6. **Generate token** → 한 번만 표시되는 PAT 문자열 복사
7. **org access**: `need-singularity` org 사용 시 SSO authorize 추가 클릭
   - org 권한 부재 시 fallback: `ghcr.io/<github-username>/hexa-runner` user namespace 사용

### §1.2 Step 2 — PAT 보안 보관 (3 가지 권장 방식 중 택 1)

**가장 안전 (권장)**: 1Password / Bitwarden 같은 vault
```
1Password CLI:
op item create --category=password --title="ghcr-hexa-runner-pat" \
  --vault Personal password=<paste-once-here-deleted-after>
```

**가성비**: pass (gpg 기반)
```sh
pass insert ghcr/hexa-runner-pat
# 한 번 paste; 이후 pass show ghcr/hexa-runner-pat
```

**최소 환경**: age 암호화 파일
```sh
mkdir -p ~/.config/hexa
chmod 700 ~/.config/hexa
echo -n "<PAT>" | age -r age1<your-pubkey> > ~/.config/hexa/ghcr.token.age
chmod 600 ~/.config/hexa/ghcr.token.age
# 사용 시: age -d -i ~/.age/key.txt ~/.config/hexa/ghcr.token.age
```

**비추천 (raw 65 위반 위험)**:
- `~/.config/hexa/ghcr.token` 평문 mode 0600 (잠시 OK; commit 위험)
- shell history 에 PAT 직접 echo (절대 금지)
- 환경 변수 `export GHCR_TOKEN=ghp_...` (ps 노출, history 저장)

### §1.3 Step 3 — 환경 변수 export (PAT 자체는 file 에서만)

```sh
# ~/.zshrc 또는 cycle-별 임시 shell:
export GHCR_USER="<github-username>"
export GHCR_TOKEN_FILE="$HOME/.config/hexa/ghcr.token"   # 또는 age-decrypt FIFO
# PAT 자체는 환경변수 NOT export — 파일에서만 읽음 (raw 65)
```

### §1.4 Step 4 — 자동화 script 실행

```sh
cd ~/core/n6-architecture

# 4a. dry-run 검토 (PAT 사용 안 함; 단지 plan 출력)
bash tool/ghcr_push.hexa --plan

# 4b. 실 실행 (PAT 사용 — 사용자 명시 OK 후에만)
bash tool/ghcr_push.hexa --execute

# 4c. ledger 확인
tail -20 state/audit/docker_registry_push_events.jsonl | jq .
```

## §2 자동화 script tool/ghcr_push.hexa 명세

### §2.1 흐름

```
preflight (docker CLI + image local + GHCR_USER + GHCR_TOKEN_FILE + mode 0600)
    ↓
step1_login_local  (cat $GHCR_TOKEN_FILE | docker login ghcr.io -u $GHCR_USER --password-stdin)
    ↓
step2_tag         (cycle-14, 2026-04-28, latest 3-tag)
    ↓
step3_push        (3 tag push)
    ↓
step4_remote_pull (ubu1 / ubu2 / hetzner: ssh login + pull + retag + node20 verify)
    ↓
ledger_append    (state/audit/docker_registry_push_events.jsonl 매 step)
```

### §2.2 raw 138 ledger schema

`state/audit/docker_registry_push_events.jsonl`:
```json
{"schema":"raw_138_docker_registry_push_v1",
 "ts":"<UTC ISO8601>",
 "event":"<preflight|login_local|tag|push|remote_login|remote_pull|remote_retag|remote_verify>",
 "host":"<mac|ubu1|ubu2|hetzner>",
 "image_local":"hexa-runner:latest",
 "ghcr_namespace":"ghcr.io/need-singularity/hexa-runner",
 "cycle_tag":"cycle-14",
 "exit_code":<int>,
 "note":"<freeform>",
 "cycle":"cycle-14/F-DOCKER-REGISTRY-PUSH"}
```

PAT 평문은 **절대 note 필드 진입 금지** (F-B3-GHCR-PREP-2 falsifier).

## §3 private vs public 결정 (raw 65)

**기본값: private** — 다음 이유:
- raw 65 최소 blast radius. image-baked credentials/ENV/secret 잔존 가능성 0% 입증 어려움
- proprietary 알고리즘 부재 확인했으나, dependency tree 의 pinned vulnerable version exposure 위험
- ghcr.io private 무료. cost 0
- private 인 경우 모든 pull host 도 `docker login ghcr.io` 필요 (script 내 자동)

**public 전환 조건** (cycle 16+ 검토):
1. image scan tool (trivy / grype) PASS
2. dockerfile 의 모든 RUN/COPY/ARG 가 public-data only 입증
3. 사용자 명시 OK (raw 91 C3)

## §4 fallback (F-DOCKER-REGISTRY-PUSH-FALLBACK)

ghcr.io PAT 발급 거부 / org 권한 부재 / 14d deadline 초과 시:

### §4.1 Option 4 docker save/load (cycle 7 plan)

```sh
# Mac 에서 한번 save
docker save hexa-runner:latest -o /tmp/hexa-runner.tar

# rsync to 3 hosts (raw 86 transport: ~234MB × 3 = 702MB)
for h in ubu1 ubu2 hetzner; do
  rsync -avP /tmp/hexa-runner.tar "$h:/tmp/"
  ssh "$h" 'docker load -i /tmp/hexa-runner.tar && rm /tmp/hexa-runner.tar'
done

rm /tmp/hexa-runner.tar  # Mac local 정리
```

### §4.2 Option 1 self-hosted registry (cycle 7 plan §4)

hetzner 위 `registry:2` + TLS 강제. 2-3 cycle 추가 작업 필요. ghcr.io 사용 못 할 때만.

## §5 cross-repo 영향 (raw 47)

`hexa-lang/stdlib/hxc_a*.hexa` script 들은 hexa-runner image 를 동일 사용:
- image 갱신 시 stdlib script 자동 적용 (별도 빌드 불요)
- ghcr.io 채널 활성화 후 `scripts/bin/hexa_remote` 가 fallback chain 의 첫 단계로 ghcr.io pull 시도 → 실패 시 local hexa-runner:latest → 실패 시 OAuth route 의 cycle 7 plan §3 spec 그대로

## §6 raw 71 falsifier 5종 (cycle 14 prep)

| ID | predicate | auto_check | deadline |
|----|-----------|------------|----------|
| F-B3-GHCR-PREP-1 | --execute 가 PAT 검증 없이 빈 token 으로 docker login | preflight GHCR_TOKEN_FILE 검증 + mode 0600 | open |
| F-B3-GHCR-PREP-2 | PAT 가 git commit / marker / ledger note 평문 노출 | pre-commit hook ghp_/gho_/ghu_/ghs_/ghr_ pattern deny | open |
| F-B3-GHCR-PREP-3 | 14d deadline 초과까지 PAT 미발급 | falsifier_monitor.hexa cycle 14+ countdown | 2026-05-12T00:00Z |
| F-B3-GHCR-PREP-4 | step4 1 host fail 이 silent 로 다른 host overwrite | ledger_append exit_code per host + final verify | open |
| F-B3-GHCR-PREP-5 | Mac arm64 → ubu/hetzner amd64 platform mismatch | docker buildx multi-arch 빌드 권장 (cycle 10 §1 step 4) | 2026-05-12T00:00Z |

## §7 14d deadline 추적 (cycle 14 prep 시점)

- now (UTC):     2026-04-28T18:45:00Z
- deadline (UTC): 2026-05-12T00:00:00Z
- **T-13.22 days** (≈ 317.25 시간)
- successor ladder:
  - 2026-05-13T00:00:00Z → F-DOCKER-REGISTRY-PUSH-OVERDUE-1D fire
  - 2026-05-19T00:00:00Z → F-DOCKER-REGISTRY-PUSH-OVERDUE-7D
  - 2026-06-11T00:00:00Z → F-DOCKER-REGISTRY-PUSH-OVERDUE-30D

## §8 raw 91 C3 honest disposition

- PAT 미발급 → actual push 0 row (ledger_init row 1 만)
- agent 는 사용자 TTY 대신 PAT 입력 불가 — cycle 14 prep 은 skeleton 까지가 한계
- option-A user-side handoff 모드는 cycle 13 부터 cumulative; cycle 14 = +1
- 사용자 PAT 발급 + --execute SUCCESS 시 raw 51 4-gate 5/5 → 6/6 architectural-FIX 승격 가능
- raw 100 fallback cumulative cycle counter 13 (cycle 13) → 14 (cycle 14 prep) 증가 예정

## §9 우려사항 (raw 91 C3 / raw 71 / raw 100)

1. **PAT 부재 cumulative**: cycle 14 도 fallback 모드 누적. raw 100 DEEP+ +4 (over max +4)
2. **need-singularity org 권한 미확인**: 사용자 personal account namespace fallback 준비 필요
3. **Mac arm64 ↔ ubu/hetzner amd64**: buildx multi-arch 빌드 미수행 시 platform mismatch 재발 위험 (F-B3-REG-1 / F-B3-GHCR-PREP-5 동일)
4. **PAT leak surface**: bash history / docker config.json base64 저장 — `~/.docker/config.json` commit 절대 금지
5. **transport cost**: 234MB × 3 host = 702MB (사용자 raw 86 견적 150MB 보다 ~4.7× 큼; 압축 가정 차이)
6. **fallback chain 8 cycle 누적**: B.3 cycle 2/7/10/13/14 합 5 cycle plan-only — 사용자 PAT 가 unblocker

— end —
