---
category: operational
date: 2026-04-28
parent_witness: design/kick/2026-04-28_docker-ghcr-cycle14-prep_omega_cycle.json
predecessor_proposals:
  - proposals/hexa_weave_kick_infra_b3_registry_2026_04_28.md (cycle 7 — Option 4 docker save/load + Option 3 ghcr.io)
  - proposals/hexa_weave_kick_infra_b3_ghcr_2026_04_28.md (cycle 10 — Option 3 5-step detail)
mission: F-DOCKER-REGISTRY-PUSH (B.3 ghcr.io) — user 4item OK 3number / 14d deadline / cycle 14 prep
status: PREP-COMPLETE — automation skeleton + user PAT guide ready. actual push   user PAT only-after.
deadline: 2026-05-12T00:00:00Z
---

# HEXA-WEAVE B.3 ghcr.io — integrated spec + user PAT issuance Korean guide

raw 9 hexa-only data path. raw 13 NO external comms. raw 91 C3 honest:
  document  cycle 7 / cycle 10 two plan   integrated one  single-source-of-truth and,
user PAT issuance all until  actual push 0 (raw 91 C3 — TTY dependency).

## §0 integrated request approx  (cycle 7 + cycle 10 + cycle 14 prep)

| item | cycle 7 plan | cycle 10 plan | cycle 14 prep (  document) |
|------|--------------|----------------|--------------------------|
| channel | Option 4 (save/load) immediately + Option 3 (ghcr.io) long-term | Option 3 5-step detail | Option 3 automation + user guide + audit ledger |
| execution | not-executed | not-executed | not-executed (PAT vsbase) |
| output | 4 option comparisontable | 5 step exact command | tool/ghcr_push.hexa + jsonl ledger |
| F-71 | F-B3-REG-1..5 | F-B3-GHCR-1..5 | F-B3-GHCR-PREP-1..5 |

## §1 user PAT issuance Korean guide (raw 65 minimal-blast-radius)

### §1.1 Step 1 — GitHub PAT issuance

1. browserlow from  <https://github.com/settings/tokens> open
   - **Tokens (classic)** choice (fine-grained   GHCR write partial path unsupport)
2. **Generate new token (classic)** click
3. **Note**: `hexa-runner ghcr push (cycle 14)` same  identify possible name
4. **Expiration**: **90 days** (raw 65 minimum blast radius — 1y/no-expiration forbidden)
5. **scopes** bodybig:
   - [x] `write:packages`
   - [x] `read:packages`
   - [ ] `delete:packages` (cycle 16+ image GC requiredwill only-when)
6. **Generate token** →  one  number only  table on become  PAT string copy
7. **org access**: `need-singularity` org use  on  SSO authorize addition click
   - org permission absent  on  fallback: `ghcr.io/<github-username>/hexa-runner` user namespace use

### §1.2 Step 2 — PAT preservenot store (3  maintain recommend method  among  choice 1)

**most safe (recommend)**: 1Password / Bitwarden same  vault
```
1Password CLI:
op item create --category=password --title="ghcr-hexa-runner-pat" \
  --vault Personal password=<paste-once-here-deleted-after>
```

** propertynon-**: pass (gpg based)
```sh
pass insert ghcr/hexa-runner-pat
#  one  number paste;   after  pass show ghcr/hexa-runner-pat
```

**minimum environment**: age encryption file
```sh
mkdir -p ~/.config/hexa
chmod 700 ~/.config/hexa
echo -n "<PAT>" | age -r age1<your-pubkey> > ~/.config/hexa/ghcr.token.age
chmod 600 ~/.config/hexa/ghcr.token.age
# use  on : age -d -i ~/.age/key.txt ~/.config/hexa/ghcr.token.age
```

**non-recommend (raw 65  above half risk)**:
- `~/.config/hexa/ghcr.token` plaintext mode 0600 (lock on  OK; commit risk)
- shell history  at  PAT direct echo (absolute forbidden)
- environment variable `export GHCR_TOKEN=ghp_...` (ps exposure, history storage)

### §1.3 Step 3 — environment variable export (PAT  itself  file  from  only )

```sh
# ~/.zshrc or cycle-per is on  shell:
export GHCR_USER="<github-username>"
export GHCR_TOKEN_FILE="$HOME/.config/hexa/ghcr.token"   # or age-decrypt FIFO
# PAT  itself  environmentvariable NOT export — file from  only  read (raw 65)
```

### §1.4 Step 4 — automation script execution

```sh
cd ~/core/n6-architecture

# 4a. dry-run review (PAT use not  must ; onlymaintain plan output)
bash tool/ghcr_push.hexa --plan

# 4b. actual execution (PAT use — user explicit OK only-after)
bash tool/ghcr_push.hexa --execute

# 4c. ledger verify
tail -20 state/audit/docker_registry_push_events.jsonl | jq .
```

## §2 automation script tool/ghcr_push.hexa specification

### §2.1 flow

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
ledger_append    (state/audit/docker_registry_push_events.jsonl each step)
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

PAT plaintext  **absolute note field entry forbidden** (F-B3-GHCR-PREP-2 falsifier).

## §3 private vs public decision (raw 65)

**base this value: private** — next reason:
- raw 65 minimum blast radius. image-baked credentials/ENV/secret residual possibility 0% demonstrated difficulty
- proprietary knowhigh-rithm absent verify, dependency tree   pinned vulnerable version exposure risk
- ghcr.io private free. cost 0
- private   case all pull host  also  `docker login ghcr.io` required (script  within  automatic)

**public transition condition** (cycle 16+ review):
1. image scan tool (trivy / grype) PASS
2. dockerfile   all RUN/COPY/ARG   public-data only demonstrated
3. user explicit OK (raw 91 C3)

## §4 fallback (F-DOCKER-REGISTRY-PUSH-FALLBACK)

ghcr.io PAT issuance reject / org permission absent / 14d deadline exceed  on :

### §4.1 Option 4 docker save/load (cycle 7 plan)

```sh
# Mac  from   one number save
docker save hexa-runner:latest -o /tmp/hexa-runner.tar

# rsync to 3 hosts (raw 86 transport: ~234MB × 3 = 702MB)
for h in ubu1 ubu2 hetzner; do
  rsync -avP /tmp/hexa-runner.tar "$h:/tmp/"
  ssh "$h" 'docker load -i /tmp/hexa-runner.tar && rm /tmp/hexa-runner.tar'
done

rm /tmp/hexa-runner.tar  # Mac local cleanup
```

### §4.2 Option 1 self-hosted registry (cycle 7 plan §4)

hetzner  above  `registry:2` + TLS enforce. 2-3 cycle addition task required. ghcr.io use cannot will only-when.

## §5 cross-repo impact (raw 47)

`hexa-lang/stdlib/hxc_a*.hexa` script   hexa-runner image   identical use:
- image update  on  stdlib script automatic apply (separate build not-required)
- ghcr.io channel activate  after  `scripts/bin/hexa_remote`   fallback chain   first phase to  ghcr.io pull attempt → failure  on  local hexa-runner:latest → failure  on  OAuth route   cycle 7 plan §3 spec as-is

## §6 raw 71 falsifier 5kind (cycle 14 prep)

| ID | predicate | auto_check | deadline |
|----|-----------|------------|----------|
| F-B3-GHCR-PREP-1 | --execute   PAT verification without empty token  to  docker login | preflight GHCR_TOKEN_FILE verification + mode 0600 | open |
| F-B3-GHCR-PREP-2 | PAT   git commit / marker / ledger note plaintext exposure | pre-commit hook ghp_/gho_/ghu_/ghs_/ghr_ pattern deny | open |
| F-B3-GHCR-PREP-3 | 14d deadline exceed until  PAT not-issued | falsifier_monitor.hexa cycle 14+ countdown | 2026-05-12T00:00Z |
| F-B3-GHCR-PREP-4 | step4 1 host fail   silent  to  other host overwrite | ledger_append exit_code per host + final verify | open |
| F-B3-GHCR-PREP-5 | Mac arm64 → ubu/hetzner amd64 platform mismatch | docker buildx multi-arch build recommend (cycle 10 §1 step 4) | 2026-05-12T00:00Z |

## §7 14d deadline trace (cycle 14 prep point-in-time)

- now (UTC):     2026-04-28T18:45:00Z
- deadline (UTC): 2026-05-12T00:00:00Z
- **T-13.22 days** (≈ 317.25 time)
- successor ladder:
  - 2026-05-13T00:00:00Z → F-DOCKER-REGISTRY-PUSH-OVERDUE-1D fire
  - 2026-05-19T00:00:00Z → F-DOCKER-REGISTRY-PUSH-OVERDUE-7D
  - 2026-06-11T00:00:00Z → F-DOCKER-REGISTRY-PUSH-OVERDUE-30D

## §8 raw 91 C3 honest disposition

- PAT not-issued → actual push 0 row (ledger_init row 1  only )
- agent   user TTY instead PAT input impossible — cycle 14 prep   skeleton  until    one system
- option-A user-side handoff mode  cycle 13 from cumulative; cycle 14 = +1
- user PAT issuance + --execute SUCCESS  on  raw 51 4-gate 5/5 → 6/6 architectural-FIX promotion possible
- raw 100 fallback cumulative cycle counter 13 (cycle 13) → 14 (cycle 14 prep) increase exampledefinite

## §9 concern (raw 91 C3 / raw 71 / raw 100)

1. **PAT absent cumulative**: cycle 14  also  fallback mode cumulative. raw 100 DEEP+ +4 (over max +4)
2. **need-singularity org permission unverify**: user personal account namespace fallback standardnon- required
3. **Mac arm64 ↔ ubu/hetzner amd64**: buildx multi-arch build not-performed  on  platform mismatch recurrence risk (F-B3-REG-1 / F-B3-GHCR-PREP-5 identical)
4. **PAT leak surface**: bash history / docker config.json base64 storage — `~/.docker/config.json` commit absolute forbidden
5. **transport cost**: 234MB × 3 host = 702MB (user raw 86 estimate 150MB than ~4.7× large; compression assumption difference)
6. **fallback chain 8 cycle cumulative**: B.3 cycle 2/7/10/13/14 sum 5 cycle plan-only — user PAT   unblocker

— end —
