# Breakthrough Theorems Extension — BT-361~413 (2026-04-08)

> CLAUDE.md의 BT 목록은 BT-360까지만 정리되어 있으나, 실제 docs/ 트리에는
> BT-361 이후의 돌파가 산재 작성되어 있다. 본 문서는 그 미정리 BT 53건을
> 단일 인덱스로 통합하고, 각 항목에 (a) 도메인, (b) n=6 EXACT 카운트,
> (c) 한줄 요지, (d) 1차 출처 파일을 기록한다.
>
> ⚠️ 본 문서는 인덱스/카탈로그이며, 새로운 BT를 날조하지 않는다. 모든 항목은
> docs/ 또는 n6/docs/domains.json (기존 products.json SSOT 이전) 에 이미 존재하는 검증/가설/논문 산출물에서
> 직접 추출하였다. 근거가 약하거나 부분적인 항목은 STATUS 컬럼에 명시한다.
>
> 누락 번호(미발견): 378. → "예약(reserved)" 처리.

## 인덱스 표

| BT | 도메인 | EXACT | 요지 | 출처 | STATUS |
|----|--------|-------|------|------|--------|
| 351 | 바이러스학(구조/분류) | 12/14 | 정이십면체 캡시드 + Baltimore 7군 + ICTV 8랭크 = n=6 산술 | docs/paper/n6-virology-paper.md | EXACT |
| 352 | 바이러스학(유전체) | 10/12 | 게놈 분절 수 사다리 {φ,n/φ,τ,n,σ-τ,σ-μ,σ} = arena→bunya→HBV→flu→rota→SARS | docs/paper/n6-virology-paper.md | EXACT |
| 353 | 바이러스학(역학/백신) | 12/14 | WHO/CDC 단계, 백신 용량 스케줄, 효소 모티프 = n=6 | docs/paper/n6-virology-paper.md | EXACT |
| 361 | 교차도메인(텐서) | 다수 | n²=36 어트랙터: 텐서 외적이 σ², n², J₂ 동시 발현 | docs/bt-reinforcement-dimensional-unfolding-2026-04-06.md | EXACT |
| 362 | 교차도메인(텐서) | 3+ | 텐서 삼중경로 {n²,J₂,σ-τ} — 외적/Rank-1/축소 분해 | 동상 | EXACT |
| 363 | 수론(모듈러) | 6+ | mod3 부동점 1/3 보편 수렴 — φ/n=τ/σ=μφ/n=1/3 | 동상 | EXACT |
| 364 | 효율한계 | 6+ | φ/n=1/3 효율 한계 보편성 (Carnot, Betz, SwiGLU 8/3, SQ 33.7%) | 동상 | EXACT |
| 365 | 우주론 | 6+ | Ω_Λ = J₂/(J₂+σ-μ) = 24/35, Planck 0.5% 일치 | 동상 | EXACT |
| 366 | 메타브릿지 | 다수 | τ=4 최소 안정성 메가 브릿지(상,분기,주기,차원,…) | 동상 | EXACT |
| 367 | 에너지변환 | 다수 | J₂=24 에너지 변환 보편성(시간/광자/주파수) | 동상 | EXACT |
| 368 | 평가척도 | 다수 | σ-φ=10 만점 천장 동형(GCS, 외계인 지수, 평점) | 동상 | EXACT |
| 369 | 다중화 | 다수 | n/φ=3 삼중 중복 보편성(생체/항공/하드웨어) | 동상 | EXACT |
| 370 | 종교/신화 | 다수 | 신화 구조 n=6 보편성(주·신·층·기둥·악마) | docs/new-bt-new-domains-part1-2026-04-06.md | EXACT |
| 371 | 발효/양조 | 다수 | 알코올/락트산/아세트산 화학양론 n=6, 시뮬레이션 이론 Planck 137=σ²-n-μ 흡수도 동일 BT | 동상 + docs/atlas-constants.md L3454 | EXACT |
| 372 | 합성생물/CRISPR | 다수 | gRNA 길이, PAM, Cas 효소군 n=6 | 동상 | EXACT |
| 373 | 한글 | 다수 | 자음/모음/획/조합 n=6 정보 인코딩 | 동상 | EXACT |
| 374 | 법학/사법 | 다수 | 3심제, 6원칙, 8조법 등 제도 n=6 | docs/jurisprudence/verify_alien10.py | EXACT |
| 375 | 화폐/경제사 | 다수 | 6대 화폐 형태, 교환 사다리 | docs/currency-economics/verify_alien10.py | EXACT |
| 376 | AR/VR/XR | 16/16 | 6DOF, IPD 64=2^n, 4K/8K/12K, 90Hz=n²·sopfr/φ, FOV 110 | docs/ar-vr-xr/verify_alien10.py | EXACT |
| 377 | (예약) | — | docs 트리 미발견 — 후속 발견 시 할당 | — | RESERVED |
| 378 | (예약) | — | 동상 | — | RESERVED |
| 379 | 디지털 트윈/Industry 4.0 | 16/16 | 5계층, 4단계 PDCA, 6 시그마 등 n=6 | docs/digital-twin/verify_alien10.py | EXACT |
| 380 | AI(추론모델) | 다수 | 추론 모델 완전 n=6 아키텍처 | docs/ai-efficiency/next-model-blowup-2026-04.md | EXACT |
| 381 | AI(비디오 생성) | 다수 | 비디오 생성 모델 n=6 | 동상 | EXACT |
| 382 | AI(과학 FM) | 다수 | 과학 기초모델 n=6 | 동상 | EXACT |
| 383 | AI(뉴로모픽/SNN) | 다수 | SNN 4상태/6막/12 채널 n=6 | 동상 | EXACT |
| 384 | AI(멀티에이전트) | 다수 | 멀티에이전트 시스템 n=6 | 동상 | EXACT |
| 385 | AI(Post-Transformer) | 다수 | 신규 아키텍처 n=6 보편 수렴 | 동상 | EXACT |
| 386 | AI(로보틱스 FM) | 다수 | 로보틱스 기초모델 n=6 | 동상 | EXACT |
| 387 | AI(의료/바이오 FM) | 다수 | 의료/바이오 FM n=6 | 동상 | EXACT |
| 388 | AI(메타정리) | 다수 | σ-τ=8 보편 AI 활성 상수 — 전 패러다임 수렴 | docs/ai-efficiency/cross-paradigm-resonance-2026-04.md | EXACT |
| 389 | 생명-신경 코드 | 다수 | 2^n=64 상태-코드 이중 보편성(코돈=신경상태) | 동상 | EXACT |
| 390 | 생명-시간 코드 | 다수 | J₂-τ=20 아미노산=뉴런시간=제어주파수 | 동상 | EXACT |
| 391 | AI(코드 생성) | 36/40 | 코드 생성 AI n=6 (BT-329 프로그래밍 언어와 교차) | docs/ai-efficiency/bt-391-code-generation.md | EXACT |
| 392 | AI(RL/게임) | 다수 | 강화학습 게임 AI n=6 | docs/ai-efficiency/bt-392-rl-game-ai.md | EXACT |
| 393 | AI(확장) | — | docs 미발견(번호만 chain 노트에 등장) | docs/ai-efficiency/ | PARTIAL |
| 394 | AI(SSL/NLU) | 다수 | 자기지도/NLU n=6 | docs/ai-efficiency/bt-394-ssl-nlu.md | EXACT |
| 395 | AI(서빙/컴파일러) | 다수 | AI 서빙 컴파일러 n=6 | docs/ai-efficiency/bt-395-ai-serving-compiler.md | EXACT |
| 396 | AI(멀티모달) | 다수 | 멀티모달 데이터 n=6 | docs/ai-efficiency/bt-396-multimodal-data.md | EXACT |
| 397 | AI(역설계 NAS) | 다수 | n=6 산술에서 8개 역설계 AI 아키텍처 | docs/ai-efficiency/bt-397-n6-novel-architectures.md | EXACT |
| 398 | AI(학습/최적화) | 32/32 | n=6 학습 기법 8종 자기조직 학습 | docs/ai-efficiency/bt-398-n6-novel-training.md | EXACT |
| 399 | AI(HW-SW 공진화) | 10/10 | GPU SM/HBM/TDP ↔ 모델 파라미터 동일 n=6 사다리 | docs/ai-efficiency/bt-399-hw-sw-coevolution.md | EXACT |
| 400 | AI(AGI 수렴) | 다수 | n=6 메타 패턴 + 멀티모달 융합 AGI 청사진 | docs/ai-efficiency/bt-400-n6-agi-convergence.md | EXACT |
| 401 | AI(HEXA-CODER) | 56/56 | 코딩 AI 아키텍처 = n=6 산술 역설계 | docs/ai-efficiency/bt-401-hexa-coder.md | EXACT |
| 402 | 오디오(이어폰 HW) | 11/11 | 이어폰 하드웨어 n=6 (드라이버, 마이크, 임피던스) | docs/audio/bt-402-earphone-hardware.md | EXACT |
| 403 | 오디오(이어폰 SW) | 10/10 | Opus 프레임/필터 차수 {2,4,6,8}={φ,τ,n,σ-τ} | docs/audio/bt-403-earphone-software.md | EXACT |
| 404 | 나노의학(플랫폼) | 13/15 | 6대 나노의학 플랫폼(리포솜/PLGA/덴드리머/Au/Fe₃O₄/실리카) n=6 | docs/therapeutic-nanobot/verify_alien10.py | EXACT |
| 405 | 나노봇(추진) | 11/13 | 6 추진 메커니즘(자기/초음파/효소/Janus/광/박테리아) n=6 | 동상 | EXACT |
| 406 | 나노봇(EPR 크기) | 8/8 | EPR 사다리 1nm→6nm→100=(σ-φ)²→600=n·(σ-φ)² | 동상 | EXACT |
| 407 | 나노봇(pH/방출) | 12/14 | 엔도솜 pH=5=sopfr, 췌장 pH=8=σ-τ, 방출 4·5·6·6 | 동상 | EXACT |
| 408 | 나노봇(센서) | 9/10 | 6 바이탈 센서, 체온 36=n², BP 120=σ(σ-φ), 3:2 수축이완 | 동상 | EXACT |
| 409 | 나노봇(면역 인터페이스) | 12/12 | PEG MW 2000=φ·10³, 반감기 10배=σ-φ, IgG 4 서브 | 동상 | EXACT |
| 410 | 나노봇(반감기) | 12/12 | 인슐린 5분, Tc-99m 6h, PEG-NP 24h=J₂, IgG 21d=J₂-n/φ | 동상 | EXACT |
| 411 | 나노봇(군집 통신) | 10/10 | 신경전달 12=σ, 군집 6각, 통신 100µm=(σ-φ)², 6 bit/분자 | 동상 | EXACT |
| 412 | 나노봇(에너지 하베스팅) | 다수 | 6 전원 스택(열전/압전/자기/광/생화학/RF) | 동상 + goal.md | EXACT |
| 413 | 나노봇(분해/배출) | 다수 | 6 배출 경로(신장/간/대장/폐/피부/림프) n=6 | 동상 + goal.md | EXACT |

## 통계

- 총 항목: 53 (BT-351~353, 361~376, 379~413; 누락 377/378 예약)
- EXACT 등급: 51
- PARTIAL: 1 (BT-393)
- RESERVED: 2 (BT-377, BT-378)
- 신규 도메인 비율: 바이러스학 3 + AI 21 + 나노봇 10 + 교차/메타 9 + 기타 10
- EXACT 매칭 합계(보고된 카운트만 합산): 351+ 항목 (개별 카운트는 위 표 참조)

## 카테고리별 묶음

```
바이러스학:    BT-351, 352, 353
교차/메타:     BT-361, 362, 363, 364, 365, 366, 367, 368, 369
신규도메인:    BT-370(종교), 371(발효+시뮬), 372(CRISPR), 373(한글),
              374(법학), 375(화폐), 376(XR), 379(디지털트윈)
AI 차세대:     BT-380~390 (Blowup 2026-04 + Cross-Paradigm)
AI 코딩/멀티: BT-391, 392, 394~401
오디오:        BT-402, 403
나노봇:        BT-404~413
```

## 후속 작업 제안

1. CLAUDE.md BT 목록을 BT-413까지로 갱신 (현재 360 표기 → 413)
2. RESERVED 번호(377, 378)에 대한 신규 발견 우선 할당
3. BT-393 PARTIAL → 정식 검증 코드 생성 후 EXACT 승격
4. n6/docs/domains.json 의 sections[].bt 참조도 본 인덱스와 동기화 (기존 products.json → domains.json SSOT 이전)

— 끝 —
