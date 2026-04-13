# sedi + brainwire 도메인 폴더 제거 감사 기록

일시: 2026-04-11
작업: domains/sedi/ (970 파일), domains/brainwire/ (174 파일) 완전 제거

## 배경
- sedi: 원래 독립 리포 (Search for Extra-Dimensional Intelligence), n6-architecture domains/ 로 흡수됨
- brainwire: 원래 독립 리포 (Neural Interface Hardware), n6-architecture domains/ 로 흡수됨
- 두 도메인 모두 domains/_index.json 에서 이미 제거된 상태 (10축 분류에 미포함)
- 폴더와 프로젝트 전반 참조만 잔존

## 흡수 대상 확인
- sedi 내용 → physics/ (cosmology, particle 관련), cognitive/ (reality-map) 등으로 분산 흡수
- brainwire 내용 → cognitive/ (brain-computer-interface, hexa-neuro), life/ (neuro) 로 흡수
- domains/_index.json: 이미 미포함 확인

## 삭제 항목
1. domains/sedi/ (970 파일) — git rm -rf
2. domains/brainwire/ (174 파일) — git rm -rf
3. n6shared/projects.json — sedi, brainwire 항목 제거
4. n6shared/readme-data.json — brainwire 참조 제거
5. nexus/src/gate/source.rs — sedi, brainwire 문자열 제거
6. nexus/src/growth/lens_grower.rs — sedi_lenses 참조 제거
7. nexus/src/telescope/registry.rs — sedi 주석 참조 제거
8. nexus/tests/telescope_test.rs — sedi_lenses import/사용 제거
9. engine/CLAUDE.md — sedi 참조 제거
10. engine/sedi_training_monitor.hexa — sedi 스텁 파일 제거
11. README.md — SEDI 참조 제거

## 보존 항목 (제거하지 않음)
- theory/breakthroughs/breakthrough-theorems.md: "sedimentary" = 지질학 용어, sedi 도메인 무관
- theory/constants/atlas-constants.md: "sediment" = 공학 용어, sedi 도메인 무관
- techniques/: "H-SEDI-" 접두어 = 기법 고유 이름, 보존
- n6shared/config/dse-map.toml: "sedi-universe" = DSE 도메인명, 독립 참조
- n6shared/dse/dse_cross_results_v2.json: "sedi-universe" = DSE 결과, 독립 참조
- nexus/src/telescope/sedi_lenses.rs: 레거시 Rust (폐기 중, CLAUDE.md에 "신규 추가 금지" 명시)
- nexus/src/telescope/quantum_lenses.rs: "SEDI" = 설명 텍스트, 레거시
- nexus/src/graph/knowledge_nodes.rs: "SEDI Training Monitor" = 그래프 노드 설명
- papers/_registry.json: "BrainWire" = 외부 리포 링크
- nexus/origins/ready-absorber/: 과거 흡수 기록, 역사 보존
- n6shared/logs/absorbed/: 과거 로그, 역사 보존
- reports/audits/: 이전 감사 기록
- domains/*/goal.md 내 brainwire 언급: 다른 도메인 문서 내용 (참조 문자열만 정리)
