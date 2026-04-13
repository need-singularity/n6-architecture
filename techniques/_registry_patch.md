# _registry.json 패치 제안 (직접 수정 대신 diff 문서)

> 축: techniques
> 규칙: R14 (SSOT), R18 (미니멀), R25 (공용 설정 직접 수정 금지)
> 대상: `techniques/_registry.json`
> 상위: `CLAUDE.md`

## 목적

`techniques/_registry.json` 에 신규 `sota` 서브축을 추가하여 SOTA 3종
(Mamba-2 확장 · Hyena · RWKV v7)을 공식 레지스트리에 편입.
`_total` 은 66 → 69 로 갱신. 실제 파일 수정은 사용자 승인 후 적용.

## 패치 diff (unified)

```diff
--- a/techniques/_registry.json
+++ b/techniques/_registry.json
@@ -1,7 +1,7 @@
 {
   "_doc": "AI 기법 66종 레지스트리. 각 항목은 techniques/<sub>/<name>.hexa 로 실행",
-  "_version": "1.0.0",
-  "_total": 66,
+  "_version": "1.1.0",
+  "_total": 69,

   "attention": [
     "alibi_attention", "dedekind_head", "egyptian_attention",
@@ -36,6 +36,15 @@
     "complete_llm_n6", "constitutional_ai", "context_window_ladder",
     "detr_queries", "fpn_pyramid", "griffin_rglru", "mamba2_ssm",
     "phi6simple", "rectified_flow", "sd3_mmdit", "simclr_temperature",
     "vit_patch_n6", "whisper_ladder", "yolo_nms", "zetaln2_activation"
-  ]
+  ],
+  "sota": [
+    "mamba2", "hyena", "rwkv"
+  ],
+
+  "_sota_doc": "최신 SOTA 3종 — 각 항목은 techniques/sota/<name>.md 설계서 + techniques/sota/<name>.hexa 스텁 쌍. 세부 N61 설계는 해당 .md 참고.",
+  "_bench_plan": "techniques/_bench_plan.md",
+  "_chip_mapping": "techniques/_chip_mapping.md"
 }
```

## 적용 전 체크리스트

- [ ] `techniques/sota/` 디렉토리 존재 확인 (mkdir 완료)
- [ ] 6개 파일 존재 확인
  - [x] `techniques/sota/CLAUDE.md`
  - [x] `techniques/sota/mamba2.md`
  - [x] `techniques/sota/mamba2.hexa`
  - [x] `techniques/sota/hyena.md`
  - [x] `techniques/sota/hyena.hexa`
  - [x] `techniques/sota/rwkv.md`
  - [x] `techniques/sota/rwkv.hexa`
- [ ] `_bench_plan.md` + `_chip_mapping.md` 링크 유효
- [ ] 사용자 명시 승인 (R25 — `_registry.json` 은 공용 SSOT)
- [ ] 적용 후 `INDEX.json` 의 `techniques.subs` 배열에도 `"sota"` 추가

## 적용 명령 (승인 후)

```sh
# 1) _registry.json 패치
hexa run nexus/origins/scripts/patch_registry.hexa \
    --file techniques/_registry.json \
    --patch techniques/_registry_patch.md

# 2) INDEX.json subs 확장
hexa run nexus/origins/scripts/patch_index.hexa \
    --axis techniques --add-sub sota

# 3) 커밋
git add techniques/sota techniques/_*.md techniques/_registry.json INDEX.json
git commit -m "feat(techniques): SOTA 3종 (Mamba2/Hyena/RWKV) + 16 벤치 계획 + 칩맵"
```

## 규칙 게이트

- R14: 규칙 본문은 `n6shared/rules/common.json` 유지, 이 .md 는 참조만
- R18: 미니멀 — `sota` 키 1개 + `_total` 수정 + 힌트 3개만
- R25: `_registry.json` 직접 수정 금지, 승인 후 스크립트로 적용
- R28: 재측정 결과는 `n6shared/n6/atlas.n6` 직접 흡수 (JSON 신규 발견 저장소 금지)

## 관련 링크
- 벤치: `_bench_plan.md`
- 칩맵: `_chip_mapping.md`
- SOTA: `sota/`
- 상위: `CLAUDE.md` + `../INDEX.json`
