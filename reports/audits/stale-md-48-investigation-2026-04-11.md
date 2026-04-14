# 미이관 설계 MD 48건 조사 리포트

- 생성일: 2026-04-11
- 원본 참조: `reports/audits/products-link-remap-2026-04-11.md` (MISS 174건)
- 조사 범위: MISS 174건 중 `docs/<dom>/<nested>.md` 패턴 48건 (paper 116건, calc 10건 제외)
- 기준 경로: `$N6_ARCH/`
- 작업 방식: 읽기 전용 (조사만, 이관 수행 안 함)
- 대상 도메인: 24개 / 9축 중 6축 (compute · culture · energy · infra · life · materials)

## 총괄 (한눈에)

- 전체 48건 중 **46건 FOUND_INTEGRATED** (95.8%) — 이미 `domains/<axis>/<dom>/<dom>.md` 통합본에 흡수됨
- **2건 FOUND_ALT (DIR)** — 디렉토리 엔트리, `domains/infra/environmental-protection` 으로 매핑
- **0건 MISSING** — 진짜 작성 필요한 ghost 파일은 없음 (전부 통합 완료)
- **0건 FOUND_AT_ORIG** — 원본 `docs/<dom>/...` 경로에 실존하는 파일 없음

핵심 발견: 48건 전부가 통합 과정에서 흡수되었음. products.json 링크가 구 docs/ 구조를 가리키고 있으므로 링크만 갱신하면 MISS 0건으로 수렴.

## 조사 방법

1. `products-link-remap-2026-04-11.md` 의 `| MISS |` 행 175개 추출
2. `docs/paper/` (116건) + `calc/*.py` (10건) + 테이블 헤더 1건 제외 → 48건
3. 각 항목에 대해 3단 탐색:
   - **원본 경로 실존 확인**: `docs/<rel>` 실존 여부
   - **디렉토리 엔트리 처리**: 경로 끝이 `/` 인 경우 `domains/*/<dom>` 검색
   - **통합본 흡수 확인**: `domains/<axis>/<dom>/<dom>.md` 내 `### 출처: \`<nested>\`` 또는 `### 출처: \`<basename>\`` 헤더 grep
4. 실패 시 도메인 내부 → 전역 basename 검색 (hypotheses.md 같이 공용 파일명 주의)

## 1. 미이관 MD 48건 목록

| No | 원본 링크 (products.json 기준) | 실제 위치 | 분류 |
|---:|---|---|---|
| 01 | `docs/ai-efficiency/techniques-complete.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | FOUND_INTEGRATED |
| 02 | `docs/ai-efficiency/full-verification-matrix.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | FOUND_INTEGRATED |
| 03 | `docs/ai-efficiency/next-model-blowup-2026-04.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | FOUND_INTEGRATED |
| 04 | `docs/ai-efficiency/bt-391-code-generation.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | FOUND_INTEGRATED |
| 05 | `docs/ai-efficiency/bt-397-n6-novel-architectures.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | FOUND_INTEGRATED |
| 06 | `docs/audio/full-verification-matrix.md` | `domains/culture/audio/audio.md` | FOUND_INTEGRATED |
| 07 | `docs/audio/hexa-ear-ultimate.md` | `domains/culture/audio/audio.md` | FOUND_INTEGRATED |
| 08 | `docs/audio/hexa-bone-ultimate.md` | `domains/culture/audio/audio.md` | FOUND_INTEGRATED |
| 09 | `docs/audio/hexa-ear-cell.md` | `domains/culture/audio/audio.md` | FOUND_INTEGRATED |
| 10 | `docs/audio/hexa-speaker-ultimate.md` | `domains/culture/audio/audio.md` | FOUND_INTEGRATED |
| 11 | `docs/chip-architecture/ultimate-consciousness-soc.md` | `domains/compute/chip-architecture/chip-architecture.md` | FOUND_INTEGRATED |
| 12 | `docs/chip-architecture/hexa-topological-performance-chip.md` | `domains/compute/chip-architecture/chip-architecture.md` | FOUND_INTEGRATED |
| 13 | `docs/chip-architecture/hexa-asic-skywater.md` | `domains/compute/chip-architecture/chip-architecture.md` | FOUND_INTEGRATED |
| 14 | `docs/chip-architecture/full-verification-matrix.md` | `domains/compute/chip-architecture/chip-architecture.md` | FOUND_INTEGRATED |
| 15 | `docs/horology/hypotheses.md` | `domains/culture/horology/horology.md` | FOUND_INTEGRATED |
| 16 | `docs/display/full-verification-matrix.md` | `domains/compute/display/display.md` | FOUND_INTEGRATED |
| 17 | `docs/battery-architecture/hexa-auto-battery.md` | `domains/energy/battery-architecture/battery-architecture.md` | FOUND_INTEGRATED |
| 18 | `docs/environmental-protection/` | `domains/infra/environmental-protection` (DIR) | FOUND_ALT |
| 19 | `docs/environmental-protection/microplastics-solution.md` | `domains/infra/environmental-protection/environmental-protection.md` | FOUND_INTEGRATED |
| 20 | `docs/environmental-protection/evolution/` | `domains/infra/environmental-protection` (DIR) | FOUND_ALT |
| 21 | `docs/environmental-protection/testable-predictions-2030.md` | `domains/infra/environmental-protection/environmental-protection.md` | FOUND_INTEGRATED |
| 22 | `docs/mycology/hypotheses.md` | `domains/life/mycology/mycology.md` | FOUND_INTEGRATED |
| 23 | `docs/mining/hypotheses.md` | `domains/infra/mining/mining.md` | FOUND_INTEGRATED |
| 24 | `docs/veterinary/hypotheses.md` | `domains/life/veterinary/veterinary.md` | FOUND_INTEGRATED |
| 25 | `docs/horticulture/hypotheses.md` | `domains/life/horticulture/horticulture.md` | FOUND_INTEGRATED |
| 26 | `docs/fusion/evolution/mk-1-first-light.md` | `domains/energy/fusion/fusion.md` | FOUND_INTEGRATED |
| 27 | `docs/fusion/alien-level-discoveries.md` | `domains/energy/fusion/fusion.md` | FOUND_INTEGRATED |
| 28 | `docs/fusion/physical-limit-proof.md` | `domains/energy/fusion/fusion.md` | FOUND_INTEGRATED |
| 29 | `docs/hiv-treatment/evolution/mk-1-basic.md` | `domains/life/hiv-treatment/hiv-treatment.md` | FOUND_INTEGRATED |
| 30 | `docs/hiv-treatment/evolution/mk-2-short.md` | `domains/life/hiv-treatment/hiv-treatment.md` | FOUND_INTEGRATED |
| 31 | `docs/hiv-treatment/evolution/mk-3-mid.md` | `domains/life/hiv-treatment/hiv-treatment.md` | FOUND_INTEGRATED |
| 32 | `docs/hiv-treatment/evolution/mk-4-long.md` | `domains/life/hiv-treatment/hiv-treatment.md` | FOUND_INTEGRATED |
| 33 | `docs/hiv-treatment/evolution/mk-5-ultimate.md` | `domains/life/hiv-treatment/hiv-treatment.md` | FOUND_INTEGRATED |
| 34 | `docs/mens-intimate-cleanser/breakthrough.md` | `domains/life/mens-intimate-cleanser/mens-intimate-cleanser.md` | FOUND_INTEGRATED |
| 35 | `docs/womens-intimate-cleanser/breakthrough.md` | `domains/life/womens-intimate-cleanser/womens-intimate-cleanser.md` | FOUND_INTEGRATED |
| 36 | `docs/dolphin/hypotheses.md` | `domains/life/dolphin/dolphin.md` | FOUND_INTEGRATED |
| 37 | `docs/coffee-science/hypotheses.md` | `domains/life/coffee-science/coffee-science.md` | FOUND_INTEGRATED |
| 38 | `docs/perfumery/hypotheses.md` | `domains/life/perfumery/perfumery.md` | FOUND_INTEGRATED |
| 39 | `docs/ceramics/hypotheses.md` | `domains/materials/ceramics/ceramics.md` | FOUND_INTEGRATED |
| 40 | `docs/material-synthesis/breakthrough-theorems.md` | `domains/materials/material-synthesis/material-synthesis.md` | FOUND_INTEGRATED |
| 41 | `docs/material-synthesis/hypotheses.md` | `domains/materials/material-synthesis/material-synthesis.md` | FOUND_INTEGRATED |
| 42 | `docs/material-synthesis/industrial-validation.md` | `domains/materials/material-synthesis/material-synthesis.md` | FOUND_INTEGRATED |
| 43 | `docs/material-synthesis/experimental-verification.md` | `domains/materials/material-synthesis/material-synthesis.md` | FOUND_INTEGRATED |
| 44 | `docs/material-synthesis/physical-limit-proof.md` | `domains/materials/material-synthesis/material-synthesis.md` | FOUND_INTEGRATED |
| 45 | `docs/robotics/full-verification-matrix.md` | `domains/infra/robotics/robotics.md` | FOUND_INTEGRATED |
| 46 | `docs/safety/hypotheses.md` | `domains/infra/safety/safety.md` | FOUND_INTEGRATED |
| 47 | `docs/software-design/full-verification-matrix.md` | `domains/compute/software-design/software-design.md` | FOUND_INTEGRATED |
| 48 | `docs/virology/evolution/mk-1-current.md` | `domains/life/virology/virology.md` | FOUND_INTEGRATED |

> 주1: `FOUND_INTEGRATED` 분류는 해당 통합본 내부에 `### 출처: \`<원본 nested 경로>\`` 또는 `### 출처: \`<basename>\`` 헤더가 grep 으로 확인된 경우만 부여함. basename 우연 매칭(예: sf-ufo/hypotheses 파일)은 제외.
>
> 주2: `FOUND_ALT (DIR)` 2건은 경로가 `docs/<dom>/` 또는 `docs/<dom>/evolution/` 로 끝나는 디렉토리 엔트리이며, 통합본이 아닌 디렉토리 자체를 참조함. `domains/infra/environmental-protection` 으로 링크를 갱신하면 해결됨.

## 2. 분류별 통계

| 분류 | 건수 | 비율 | 의미 |
|---|---:|---:|---|
| FOUND_INTEGRATED | 46 | 95.8% | 통합본 흡수 완료, 링크만 갱신 필요 |
| FOUND_ALT (DIR) | 2 | 4.2% | 디렉토리 엔트리, domains/... 로 갱신 |
| MISSING | 0 | 0.0% | 작성 필요한 ghost 파일 (없음) |
| FOUND_AT_ORIG | 0 | 0.0% | 원본 `docs/` 실존 (없음) |
| **합계** | **48** | **100.0%** | |

결론: 진짜 작성이 필요한 ghost 파일은 **0건**. 48건 전부가 "products.json 링크 드리프트" 문제.

## 3. 권장 이관 매핑 테이블 (48건 전체, 사용자 승인 후 적용)

본 작업은 조사 전용이므로 실제 수정/이관은 하지 않음. 아래 테이블은 products.json 의 링크를 갱신할 때 사용할 매핑 권장안임.

| products.json 링크 (구) | 권장 대상 (신) | 방법 |
|---|---|---|
| `docs/ai-efficiency/techniques-complete.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | 통합본 링크 갱신 |
| `docs/ai-efficiency/full-verification-matrix.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | 통합본 링크 갱신 |
| `docs/ai-efficiency/next-model-blowup-2026-04.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | 통합본 링크 갱신 |
| `docs/ai-efficiency/bt-391-code-generation.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | 통합본 링크 갱신 |
| `docs/ai-efficiency/bt-397-n6-novel-architectures.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | 통합본 링크 갱신 |
| `docs/audio/full-verification-matrix.md` | `domains/culture/audio/audio.md` | 통합본 링크 갱신 |
| `docs/audio/hexa-ear-ultimate.md` | `domains/culture/audio/audio.md` | 통합본 링크 갱신 |
| `docs/audio/hexa-bone-ultimate.md` | `domains/culture/audio/audio.md` | 통합본 링크 갱신 |
| `docs/audio/hexa-ear-cell.md` | `domains/culture/audio/audio.md` | 통합본 링크 갱신 |
| `docs/audio/hexa-speaker-ultimate.md` | `domains/culture/audio/audio.md` | 통합본 링크 갱신 |
| `docs/chip-architecture/ultimate-consciousness-soc.md` | `domains/compute/chip-architecture/chip-architecture.md` | 통합본 링크 갱신 |
| `docs/chip-architecture/hexa-topological-performance-chip.md` | `domains/compute/chip-architecture/chip-architecture.md` | 통합본 링크 갱신 |
| `docs/chip-architecture/hexa-asic-skywater.md` | `domains/compute/chip-architecture/chip-architecture.md` | 통합본 링크 갱신 |
| `docs/chip-architecture/full-verification-matrix.md` | `domains/compute/chip-architecture/chip-architecture.md` | 통합본 링크 갱신 |
| `docs/horology/hypotheses.md` | `domains/culture/horology/horology.md` | 통합본 링크 갱신 |
| `docs/display/full-verification-matrix.md` | `domains/compute/display/display.md` | 통합본 링크 갱신 |
| `docs/battery-architecture/hexa-auto-battery.md` | `domains/energy/battery-architecture/battery-architecture.md` | 통합본 링크 갱신 |
| `docs/environmental-protection/` | `domains/infra/environmental-protection/` | 디렉토리 링크 갱신 |
| `docs/environmental-protection/microplastics-solution.md` | `domains/infra/environmental-protection/environmental-protection.md` | 통합본 링크 갱신 |
| `docs/environmental-protection/evolution/` | `domains/infra/environmental-protection/` | 디렉토리 링크 갱신 (evolution 하위 없음) |
| `docs/environmental-protection/testable-predictions-2030.md` | `domains/infra/environmental-protection/environmental-protection.md` | 통합본 링크 갱신 |
| `docs/mycology/hypotheses.md` | `domains/life/mycology/mycology.md` | 통합본 링크 갱신 |
| `docs/mining/hypotheses.md` | `domains/infra/mining/mining.md` | 통합본 링크 갱신 |
| `docs/veterinary/hypotheses.md` | `domains/life/veterinary/veterinary.md` | 통합본 링크 갱신 |
| `docs/horticulture/hypotheses.md` | `domains/life/horticulture/horticulture.md` | 통합본 링크 갱신 |
| `docs/fusion/evolution/mk-1-first-light.md` | `domains/energy/fusion/fusion.md` | 통합본 링크 갱신 |
| `docs/fusion/alien-level-discoveries.md` | `domains/energy/fusion/fusion.md` | 통합본 링크 갱신 |
| `docs/fusion/physical-limit-proof.md` | `domains/energy/fusion/fusion.md` | 통합본 링크 갱신 |
| `docs/hiv-treatment/evolution/mk-1-basic.md` | `domains/life/hiv-treatment/hiv-treatment.md` | 통합본 링크 갱신 |
| `docs/hiv-treatment/evolution/mk-2-short.md` | `domains/life/hiv-treatment/hiv-treatment.md` | 통합본 링크 갱신 |
| `docs/hiv-treatment/evolution/mk-3-mid.md` | `domains/life/hiv-treatment/hiv-treatment.md` | 통합본 링크 갱신 |
| `docs/hiv-treatment/evolution/mk-4-long.md` | `domains/life/hiv-treatment/hiv-treatment.md` | 통합본 링크 갱신 |
| `docs/hiv-treatment/evolution/mk-5-ultimate.md` | `domains/life/hiv-treatment/hiv-treatment.md` | 통합본 링크 갱신 |
| `docs/mens-intimate-cleanser/breakthrough.md` | `domains/life/mens-intimate-cleanser/mens-intimate-cleanser.md` | 통합본 링크 갱신 |
| `docs/womens-intimate-cleanser/breakthrough.md` | `domains/life/womens-intimate-cleanser/womens-intimate-cleanser.md` | 통합본 링크 갱신 |
| `docs/dolphin/hypotheses.md` | `domains/life/dolphin/dolphin.md` | 통합본 링크 갱신 |
| `docs/coffee-science/hypotheses.md` | `domains/life/coffee-science/coffee-science.md` | 통합본 링크 갱신 |
| `docs/perfumery/hypotheses.md` | `domains/life/perfumery/perfumery.md` | 통합본 링크 갱신 |
| `docs/ceramics/hypotheses.md` | `domains/materials/ceramics/ceramics.md` | 통합본 링크 갱신 |
| `docs/material-synthesis/breakthrough-theorems.md` | `domains/materials/material-synthesis/material-synthesis.md` | 통합본 링크 갱신 |
| `docs/material-synthesis/hypotheses.md` | `domains/materials/material-synthesis/material-synthesis.md` | 통합본 링크 갱신 |
| `docs/material-synthesis/industrial-validation.md` | `domains/materials/material-synthesis/material-synthesis.md` | 통합본 링크 갱신 |
| `docs/material-synthesis/experimental-verification.md` | `domains/materials/material-synthesis/material-synthesis.md` | 통합본 링크 갱신 |
| `docs/material-synthesis/physical-limit-proof.md` | `domains/materials/material-synthesis/material-synthesis.md` | 통합본 링크 갱신 |
| `docs/robotics/full-verification-matrix.md` | `domains/infra/robotics/robotics.md` | 통합본 링크 갱신 |
| `docs/safety/hypotheses.md` | `domains/infra/safety/safety.md` | 통합본 링크 갱신 |
| `docs/software-design/full-verification-matrix.md` | `domains/compute/software-design/software-design.md` | 통합본 링크 갱신 |
| `docs/virology/evolution/mk-1-current.md` | `domains/life/virology/virology.md` | 통합본 링크 갱신 |

## 4. ghost 파일 목록 (작성 필요)

진짜 작성이 필요한 ghost 파일은 **0건**.

48건 전부 이미 `domains/<axis>/<dom>/<dom>.md` 통합본에 `### 출처: \`<경로>\`` 섹션으로 흡수되어 있음. 확인 방법:

```sh
# 샘플 1: ai-efficiency 통합본 흡수 확인
grep "### 출처:" domains/compute/ai-efficiency/ai-efficiency.md | wc -l  # 47
grep -F "출처: \`techniques-complete.md\`" domains/compute/ai-efficiency/ai-efficiency.md
# → ### 출처: `techniques-complete.md`

# 샘플 2: hiv-treatment 중첩 경로 흡수 확인
grep -F "출처: \`evolution/mk-1-basic.md\`" domains/life/hiv-treatment/hiv-treatment.md
# → ### 출처: `evolution/mk-1-basic.md`
```

따라서 본 조사 결과 추가 작성 작업은 불필요하며, products.json 링크 테이블 갱신만으로 MISS 48건 해소 가능.

## 5. 분류 분포 ASCII 막대 차트

```
분류                 건수                    [      막대 (스케일 1칸 = 2건)     ]
-----------------------------------------------------------------------------------
FOUND_INTEGRATED       46 (95.8%)           [#######################]
FOUND_ALT (DIR)         2 ( 4.2%)           [#]
MISSING                 0 ( 0.0%)           []
FOUND_AT_ORIG           0 ( 0.0%)           []
-----------------------------------------------------------------------------------
합계                   48                   [########################]
```

### 5.1 도메인 클러스터별 분포 (Top 10)

```
도메인                    건수   [ 막대 (1칸 = 1건) ]
-----------------------------------------------------
material-synthesis           5   [#####]
hiv-treatment                5   [#####]
audio                        5   [#####]
ai-efficiency                5   [#####]
environmental-protection     4   [####]
chip-architecture            4   [####]
fusion                       3   [###]
virology                     1   [#]
veterinary                   1   [#]
software-design              1   [#]
-----------------------------------------------------
(이하 14개 도메인 각 1건)   14   [##############]
```

최대 클러스터: material-synthesis / hiv-treatment / audio / ai-efficiency 4개 공동 1위 (각 5건).

### 5.2 9축 분포

```
축          건수   [ 막대 (1칸 = 1건) ]
-------------------------------------------
life          14   [##############]
compute       11   [###########]
infra          7   [#######]
culture        6   [######]
materials      6   [######]
energy         4   [####]
physics        0   []
space          0   []
cognitive      0   []
-------------------------------------------
합계          48
```

life 축이 14건으로 최다 (hiv-treatment 5 + evolution 중첩 + virology + 4 domains life hypotheses + 2 intimate-cleanser), compute 축이 11건으로 2위 (ai-efficiency 5 + chip-architecture 4 + display 1 + software-design 1).

## 부록 A. 조사 스크립트 (재현 가능)

```sh
# 1. MISS 48건 추출
grep "^| MISS |" reports/audits/products-link-remap-2026-04-11.md \
  | grep -v "docs/paper/" | grep -v "calc/" | grep -v "174" \
  > /tmp/miss_48_clean.txt
sed -E 's/.*`docs\/([^`]+)`.*/\1/' /tmp/miss_48_clean.txt > /tmp/miss_48_paths.txt
# → 48줄

# 2. 각 경로에 대해 도메인 통합본 내부 grep
while IFS= read -r rel; do
  dom=$(echo "$rel" | cut -d/ -f1)
  base=$(basename "$rel")
  nested=$(echo "$rel" | sed "s|^$dom/||")
  dom_dir=$(find domains -type d -name "$dom" 2>/dev/null | head -1)
  integrated="$dom_dir/$dom.md"
  if [ -f "$integrated" ]; then
    hit=$(grep -F "출처: \`$nested\`" "$integrated" 2>/dev/null ||
          grep -F "출처: \`$base\`" "$integrated" 2>/dev/null)
    echo "$rel | $integrated | $hit"
  fi
done < /tmp/miss_48_paths.txt
```

## 부록 B. 다음 액션 (권장)

1. **products.json 링크 갱신**: 위 §3 매핑 테이블을 바탕으로 `$NEXUS/shared/n6/docs/products.json` 링크 48건 교체
2. **재감사**: 갱신 후 `products-link-audit` 스크립트 재실행 → MISS 174 - 48 = 126 (paper 116 + calc 10) 잔존 예상
3. **paper/calc 처리**: 남은 126건은 본 작업 범위 밖. 별도 세션에서 처리
4. **archive 폴더 확인**: 혹시 `n6shared/logs/absorbed/` 아래 흡수 이력 로그와 교차 검증 시 통합 연도/일자 추가 확인 가능
