# papers — 논문 159편

목적: 도메인당 1편. Abstract+Foundation+Domain+Limitations+TestablePredictions+검증코드(.hexa) 필수
축: papers
상위: ../CLAUDE.md

## 하위
- n6-&lt;domain&gt;-paper.md  논문 본문
- README.md               작성 가이드

## SSOT
- _registry.json  논문 SSOT
- _dag.json       도메인 의존성 DAG (n6.dag.v1) — `hexa run scripts/build_dag.hexa` 로 frontmatter 집계
  schema: nodes[{id,path,alien_current,alien_target}] / edges[{from,to,alien_min,alien_cur,blocker,reason}] / _meta.cycles[]
  frontmatter 키: domain / alien_index_current / alien_index_target / requires:[{to,alien_min,reason}]

## 진입 명령
- nexus analyze sync-papers
- 발행: $PAPERS/ → Zenodo/OSF

## 절대규칙
- 한글 필수 (.md/주석/커밋)
- HEXA-FIRST (.py 금지)
- 검증코드 없는 논문 = 미완성

## 관련 링크
- 루트: ../CLAUDE.md + INDEX.json
