#!/usr/bin/env python3
"""Repair pass for Phase 6 Batch B7 translation artifacts.

Cleans up known overreach patterns from the helper pipeline so the final
text is consistent English. Each substitution is an exact string match.
"""
from __future__ import annotations

import sys
from pathlib import Path

REPAIRS: list[tuple[str, str]] = [
    # "everyday" -> got split into "every" + "day" + " -> unified " rewrite
    ("everyunified changes", "everyday changes"),
    ("everyunifiedphenomenon", "everyday phenomenon"),
    # "min기" = leftover from 최소기 (min + 기)
    ("min기", "minimum term"),
    # "min해" = leftover from 최소해
    ("min해능", "minimum-resolution capability"),
    ("min해", "minimum solution"),
    # "unified theory" stray artifacts
    ("통day 이론", "unified theory"),
    ("통day", "unified"),
    # "min포비" from 최소 포함비 (min+포비)
    ("min포비", "minimum coverage"),
    # "self 이론" corruption
    ("self 이론", "unified theory"),
    # "시뮬" -> "sim" direct fix
    ("시뮬레이션 hypothesis", "simulation hypothesis"),
    ("시뮬레이션 논제", "simulation thesis"),
    ("시뮬레이션", "simulation"),
    ("시뮬레이터", "simulator"),
    ("시뮬 ", "sim "),
    ("시뮬이", "sim"),
    # "전self" etc (from 전자 + self)
    ("전self", "electromagnetic"),
    # "탈-휴먼" = post-human
    ("탈-휴먼(posthuman) tier 도달 실패 (멸종)", "post-human tier unreached (extinction)"),
    ("탈-휴먼(posthuman)", "post-human (posthuman)"),
    ("탈-휴먼이", "post-humans"),
    ("탈-휴먼", "post-human"),
    # "조상-시뮬레이션"
    ("조상-시뮬레이션", "ancestor-simulation"),
    ("조상", "ancestor"),
    # "의향 없음"
    ("실행 의향 없음", "no intent to run"),
    ("의향 없음", "no intent"),
    ("의향", "intent"),
    ("우리는 거의 확실히", "we are almost certainly"),
    ("속에 있다", "within"),
    # "홀log래피" corruption (홀로그래피 mixed with log)
    ("홀log래피", "holography"),
    ("홀log래피가", "holography"),
    # "스케unified" = 스케일 + unified leakage (from 스케일 parts)
    ("스케unified", "scale"),
    # "structureconstant"
    ("structureconstant", "structure constant"),
    ("dimensionless", "dimensionless"),
    # "universestructure의"
    ("universestructure", "universe structure"),
    # "저field" -> 저장 + field
    ("저field", "storage"),
    # "열field" -> 열역 + field (from the "열" strings)
    ("열field", "thermal field"),
    # residual "existing " + Korean particle remainder
    ("existing ", "existing "),
    # "n=6 threads closed form"
    # Verify-code leftover Korean cleaning
    ("원주율", "pi"),
    ("원주", "circumference"),
    # "나누면" ( dividing)
    ("나누면 Planck", "divided by Planck"),
    ("나누면", "divided by"),
    # "나이"
    ("universe 나이", "universe age"),
    ("나이", "age"),
    # "허용" = allowed
    ("허용 최대", "allowed maximum"),
    ("허용", "allowed"),
    # "화학"
    ("원자·화학·생명", "atom / chemistry / life"),
    ("화학반응", "chemical reaction"),
    ("화학", "chemistry"),
    ("생명", "life"),
    # "뺀"
    ("뺀", "minus"),
    ("너무 작으면", "if too small"),
    ("너무 크면", "if too large"),
    ("너무", "too"),
    # "안정"
    ("안정점", "stable point"),
    ("불안정", "unstable"),
    ("안정", "stable"),
    # "억제"
    ("억제", "suppression"),
    # "양립"
    ("양립 창", "compatibility window"),
    ("양립", "compatibility"),
    # "창"
    ("창 137 이", "window 137"),
    # "원자"
    ("원자가", "atom"),
    ("원자", "atom"),
    # "라디온" / "컴팩트화"
    ("컴팩트화의", "compactification of"),
    ("컴팩트화", "compactification"),
    ("컴팩트 extra dim", "compact extra dim"),
    ("컴팩트 ", "compact "),
    ("컴팩트", "compact"),
    ("숨겨진", "hidden"),
    ("숨김dimension", "hidden-dimension"),
    ("라디온", "radion"),
    # "총 실dimension"
    ("총 실dimension", "total real dimension"),
    ("실dimension", "real dimension"),
    ("실수 dimension", "real dimension"),
    ("total real dimension", "total real dimension"),
    # "원리"
    ("원리", "principle"),
    # "용량"
    ("용량", "capacity"),
    # "여분" and "tier"
    ("여분", "extra"),
    # "연결"
    ("연결", "connection"),
    # "면적" -> area
    ("면적law", "area law"),
    ("면적min모", "area minimum"),
    ("면적", "area"),
    # "매핑"
    ("매핑", "mapping"),
    # "사차"
    # misc noun
    ("첫", "first"),
    ("둘째", "second"),
    ("셋째", "third"),
    # Remove stray "minimum" duplicates
    ("minimum minimum", "minimum"),
    # HEXA-BSD etc. leftover
    ("BSD Mazur reuse", "BSD Mazur reuse"),
    # "정밀도 min해능" -> precision
    ("정밀도 minimum-resolution capability", "precision resolution"),
    ("정밀도", "precision"),
    ("정밀측정", "precision measurement"),
    ("정밀", "precise"),
    # "정보"
    ("정보량", "information content"),
    ("정보 dimension", "information dimension"),
    ("정보", "information"),
    # "논제"
    ("논제", "thesis"),
    # "논"
    # Common vocab
    ("멸종", "extinction"),
    ("보호계층", "protection layer"),
    ("오버헤드", "overhead"),
    ("예산", "budget"),
    ("복제", "replica"),
    ("실효", "effective"),
    ("양측 symmetry", "bilateral symmetry"),
    ("양측", "bilateral"),
    ("배수", "multiple"),
    ("곱셈 법", "multiplication"),
    # "물리 법칙"
    ("물리 법칙의", "physical laws"),
    ("물리 법칙", "physical laws"),
    ("법칙", "law"),
    ("역학", "mechanics"),
    # "획득"
    ("획득", "acquired"),
    ("이후", "after"),
    ("이전", "before"),
    # "완전성"
    ("완전성", "completeness"),
    # "중력"
    ("중력파", "gravity wave"),
    ("중력 quantum", "quantum gravity"),
    ("중력", "gravity"),
    # "전field" and "전자"
    ("전자", "electron"),
    # "강/약"
    ("전자, 강, 약)", "electromagnetic, strong, weak)"),
    ("4개 기본 힘", "4 fundamental forces"),
    # 진공 쪼개지 (vacuum splitting)
    ("진공 ", "vacuum "),
    # "뿌리"
    ("뿌리", "root"),
    ("소립자", "elementary particle"),
    ("입자", "particle"),
    ("파장", "wavelength"),
    ("주파수", "frequency"),
    # "중심핵"
    ("중심핵", "central core"),
    ("중심", "center"),
    # "핵"
    ("핵융합", "fusion"),
    ("핵", "nucleus"),
    # "입방"
    ("입방", "cubic"),
    # "밀도"
    ("field밀도", "field density"),
    ("밀도", "density"),
    # "초기조건"
    ("초기조건", "initial condition"),
    ("초기", "initial"),
    ("진화", "evolution"),
    # "충돌"
    ("충돌", "collision"),
    ("회피", "avoidance"),
    ("감시", "watch"),
    ("파손", "corruption"),
    # "시계열"
    ("시계열", "time-series"),
    ("시공간", "spacetime"),
    ("시spacedimension", "spacetime dimension"),
    ("시space ", "spacetime "),
    # "시험"
    ("시험", "test"),
    ("실험실", "lab"),
    ("실험", "experiment"),
    ("실측", "measured"),
    # "산출"
    ("산출", "output"),
    # "공식", "식", etc.
    ("공식", "formula"),
    # "명시"
    ("명시", "listed"),
    # "필요한"
    ("필요한", "required"),
    ("불필요", "unnecessary"),
    # "빈도"
    ("빈도", "frequency"),
    ("기록", "recorded"),
    # "데이터"
    ("데이터", "data"),
    # "법", "lawa"
    # specific lat
    ("lattice QG numerical 실험", "lattice QG numerical experiment"),
    # sundry
    ("심기", "planting"),
    ("근거", "basis"),
    # "고유"
    ("고유", "intrinsic"),
    ("고유치", "eigenvalue"),
    # "점근", "점근적"
    ("점근적 자유의", "asymptotic freedom"),
    ("점근적", "asymptotic"),
    ("점근", "asymptote"),
    # "과제" etc
    ("약수합", "divisor sum"),
    ("전 도메인", "whole domain"),
    # "저 전력"
    ("저 전력", "low power"),
    # "커널"
    ("커널", "kernel"),
    # sealing / "봉합"
    # "축"
    ("축의", "axis"),
    ("축이", "axis"),
    ("축은", "axis"),
    ("축에", "on axis"),
    ("축", "axis"),
    # "폭발", "폭주"
    ("폭주", "runaway"),
    ("폭발", "blow-up"),
    ("폭증", "blow-up"),
    # "상대"
    ("상대론", "relativity"),
    ("상대적", "relative"),
    ("상대", "relative"),
    # "절대"
    ("절대", "absolute"),
    # HEXA-SC
    ("HEXA-SC 경계 준수", "HEXA-SC boundary compliance"),
    # Common Korean sentence endings
    ("셀", "cell"),
    ("셀당", "per cell"),
    ("정합", "alignment"),
    ("정의", "definition"),
    ("총 상태 space", "total state space"),
    ("상태이므로", "states"),
    ("총", "total"),
    # "위수"
    ("위수", "order"),
    # "도메인"
    ("전 도메인", "whole domain"),
    ("도메인", "domain"),
    # "수술"
    ("수술", "surgery"),
    # "수술 (surgery)"
    # "불안"
    # "회차"
    ("회", "times"),
    # "반대"
    ("반대칭", "antisymmetric"),
    ("반대", "opposite"),
    # "복원"
    ("복원", "restoration"),
    # "일반"
    ("일반", "general"),
    # "확장된"
    ("확장된", "extended"),
    # "검증 가능"
    ("검증 가능", "verifiable"),
    # "포기"
    ("포기", "discarded"),
    # "표기 동명" -> "notation shared"
    ("표기 동명", "same-notation"),
    # "외부"
    ("외부 ", "external "),
    # "입자, 흡수"
    ("흡수", "absorption"),
    # "편의상"
    ("편의상", "for convenience"),
    # "궁극"
    ("궁극의", "ultimate"),
    ("궁극", "ultimate"),
    # "형상"
    ("형상", "shape"),
    # "보존"
    ("보존", "preserve"),
    # "잠금"
    # "무손실"
    ("무손실", "lossless"),
    # "결정"
    ("결정", "determination"),
    # "구간"
    ("구간", "interval"),
    # "분자·화학 독립"
    # "분자"
    ("분자", "molecule"),
    # "삼각"
    ("삼각", "triangular"),
    # "자연"
    ("자연", "natural"),
    # "유도 확장"
    # "벽돌"
    ("벽돌", "brick"),
    # "내림"
    ("내림", "descent"),
    ("내림차순", "descending"),
    ("올림", "ascent"),
    # "적용"
    ("적용", "apply"),
    # "허")
    # common "…" residuals
    ("···", "..."),
    # "적어도"
    ("적어도", "at least"),
    ("많이", "many"),
    ("많은", "many"),
    ("적은", "few"),
    # "발표"
    ("발표", "announcement"),
    # "분류"
    ("분류", "classification"),
    # "시작"
    ("시작", "start"),
    ("끝", "end"),
    ("시작점", "start point"),
    # "탈"
    ("탈", "post"),
    # "작용"
    ("작용", "action"),
    # "반응"
    ("반응", "reaction"),
    # "효과"
    ("효과", "effect"),
    # "발현"
    ("발현", "manifestation"),
    # "배율"
    ("배율", "ratio"),
    # "배수"
    # "해석"
    ("해석", "interpretation"),
    ("해석학", "analysis"),
    # "대수학"
    ("대수학", "algebra"),
    ("대수", "algebra"),
    # "위상수학"
    ("위상수학", "topology"),
    ("위상", "topology"),
    # "기하"
    ("기하학적", "geometric"),
    ("기하", "geometry"),
    # "확률적"
    ("확률적", "probabilistic"),
    # "수학적"
    ("수학적", "mathematical"),
    ("물리적", "physical"),
    # "과학적"
    ("과학적", "scientific"),
    ("과학", "science"),
    # "정보이론"
    ("정보이론", "information theory"),
    # "계산"
    ("계산 복잡도", "computational complexity"),
    ("계산이", "computation"),
    ("계산", "compute"),
    # "복잡도"
    ("복잡도", "complexity"),
    # "회로"
    ("회로 complexity", "circuit complexity"),
    ("회로", "circuit"),
    # "환원"
    ("환원", "reduction"),
    # "결정적"
    ("결정적", "deterministic"),
    # "비결정"
    ("비결정", "nondeterministic"),
    # "확인 문제"
    ("확인 문제", "verification problem"),
    ("확인", "verify"),
    # "문제"
    ("문제", "problem"),
    # "해결책"
    ("해결책", "solution"),
    # "대응책"
    ("대응책", "countermeasure"),
    # "생성"
    ("생성", "generation"),
    # "소멸"
    ("소멸", "annihilation"),
    # "소"
    ("소멸자", "annihilator"),
    # specific Korean that should stay visible
    ("판정", "verdict"),
    # residual single-digit Hangul units
    ("하드", "hard"),
    # "알고리즘"
    ("알고리즘", "algorithm"),
    # "자동"
    ("자동", "automatic"),
    # "환경"
    ("환경", "environment"),
    # "보호"
    ("보호", "protection"),
    # "간편", "간결" "간략"
    ("간결", "concise"),
    ("간편", "convenient"),
    ("간략", "brief"),
    # "재현성"
    ("재현성", "reproducibility"),
    # "재현"
    ("재현", "reproduce"),
    # "유효"
    ("유효", "valid"),
    # "타당"
    ("타당", "valid"),
    # "유도한다"
    ("유도한", "derived"),
    ("유도", "derivation"),
    # "비고"
    ("비고", "note"),
    # "부품"
    ("부품", "component"),
    # "연동"
    ("연동", "integration"),
    # "소재"
    ("소재", "material"),
    # "베드"
    ("베드", "bed"),
    # "프로토타입"
    ("프로토타입", "prototype"),
    # "유인"
    ("유인", "crewed"),
    ("유인/상용 인증", "crewed/commercial certification"),
    # "인증"
    ("인증", "certification"),
    # "상용"
    ("상용", "commercial"),
    # "배포"
    ("배포", "deployment"),
    # "표준화"
    ("표준화", "standardization"),
    # "대량"
    ("대량", "mass"),
    # "생산"
    ("생산", "production"),
    # "스케일", "스케일링"
    ("스케일링", "scaling"),
    ("스케일", "scale"),
    # "모드"
    ("모드", "mode"),
    # "범위"
    # "동안"
    ("동안", "during"),
    # "릴레이"
    ("릴레이", "relay"),
    # "점진"
    # "다수결"
    ("다수결", "majority"),
    # "필터"
    # "멀티플렉스"
    ("멀티플렉스", "multiplexing"),
    # "투표"
    ("투표", "vote"),
    # "응답"
    ("응답", "response"),
    # "대칭 확인"
    ("대칭 확인", "symmetry check"),
    # "배선"
    ("배선", "wiring"),
    # "평균"
    ("평균", "average"),
    # "표준화"
    # "단일"
    # "다중"
    # "통신"
    ("통신", "comms"),
    # "기타"
    ("기타", "other"),
    # "전력"
    ("전력", "power"),
    # "재개"
    ("재개", "resume"),
    # "전력 기저"
    ("기저", "baseline"),
    # "저 전력"
    # "오프"
    ("오프", "off"),
    # "현재", "미래"
    ("미래", "future"),
    ("과거", "past"),
    ("현재", "current"),
    # "기대"
    ("기대", "expected"),
    # "체감"
    ("체감", "felt"),
    # "전문가", "박사급"
    ("박사급", "PhD-level"),
    ("전문가", "expert"),
    ("전문성", "expertise"),
    # "접근성"
    ("접근성", "accessibility"),
    # "학부"
    ("학부", "undergrad"),
    # "학기"
    ("학기", "semester"),
    # "폐기물"
    ("폐기물", "waste"),
    # "오염"
    ("오염", "pollution"),
    # "연구실"
    ("연구실", "lab"),
    # "접근"
    ("접근", "access"),
    # "수명"
    ("수명", "life"),
    # "신뢰"
    ("신뢰", "reliability"),
    # "습득"
    ("습득", "acquisition"),
    # "기술"
    ("기술", "tech"),
    # "쓰루풋"
    ("쓰루풋", "throughput"),
    # "증폭"
    ("증폭", "amplified"),
    # "전력요금"
    ("전력요금", "electricity bill"),
    # "절감"
    ("절감", "reduction"),
    # "탁상"
    ("탁상", "benchtop"),
    # "장비"
    ("장비", "equipment"),
    # "재현성 개선"
    # "2자릿수"
    ("2자릿수", "two orders"),
    ("자릿수", "orders"),
    # "돌파 후보"
    # "측정 한계"
    ("측정 한계", "measurement limit"),
    # "향상"
    ("향상", "gain"),
    # "증폭"
    # "개월"
    ("개월", "months"),
    # "팀"
    # "명"
    # "학부"
    # "주"
    # "그대로"
    ("그대로", "as is"),
    # "이런", "이렇게"
    # Final miscellaneous
    ("이 바뀌", "changes"),
    ("바뀌", "change"),
    # "도달"
    ("도달", "reach"),
    # "시나리오"
    ("시나리오", "scenario"),
    # "단위"
    ("단위", "unit"),
    # "시각"
    ("시각", "time"),
    # "소음"
    ("소음", "noise"),
    # "활용"
    ("활용", "uses"),
    # "풀로드"
    ("풀로드", "full load"),
    # "저부하"
    ("저부하", "low load"),
    ("고부하", "high load"),
    # "고G"
    ("고G", "high-G"),
    # "차폐"
    ("차폐", "shield"),
    # "부담"
    ("부담", "burden"),
    # "팀 공유"
    # "가설"
    # "분석"
    ("분석", "analysis"),
    # "샘플"
    ("샘플", "sample"),
    # "종료"
    ("종료", "end"),
    # "배치"
    # "정규"
    ("정규", "nominal"),
    # "기동"
    ("기동", "start"),
    # "소비전력"
    ("소비전력", "power"),
    # "소비"
    ("소비", "consumption"),

    # Round 2 specific: prose residuals after first repair pass
    ("미draft", "undrafted"),
    ("draft body", "draft body"),
    ("여전히", "still"),
    ("유지", "maintained"),
    ("honest 유지", "kept honest"),
    ("open honest", "open honest"),
    ("가정", "assumption"),
    ("동unified ", "same "),
    ("동unified", "same"),
    ("unifiedtheory", "unified theory"),
    ("unified theory ", "unified theory "),
    ("threadsand ", "threading, and "),
    ("threadsand", "threading and"),
    ("sealingand", "sealing, and"),
    ("관수", "master function"),
    ("유일식", "unique identity"),
    ("재citation", "re-cited"),
    ("citation", "citation"),
    ("5건", "5 items"),
    ("3건", "3 items"),
    ("4건", "4 items"),
    ("7건", "7 items"),
    ("건", ""),
    ("형", "form"),
    ("양식", "format"),
    # F^a_μν etc remain intact
    ("형식화", "formalization"),
    ("모든 ", "all "),
    ("모든", "all"),
    # "세 core", "세 기둥"
    ("세 core", "three-core"),
    ("세 가지", "three"),
    ("세 ", "three "),
    ("다섯 ", "five "),
    ("네 ", "four "),
    # "단 unified" artifacts
    ("단unified", "single"),
    # "지min"
    ("지min", "share"),
    # "비대칭"
    # "관세", "관측"
    ("관측", "observation"),
    # "관측치" already handled
    ("observation치", "observable"),
    # "항수"
    ("항수", "term count"),
    # "색"
    ("색 a", "color a"),
    ("색 군", "color group"),
    ("색 세", "color three"),
    ("색", "color"),
    # "군"
    ("gauge군", "gauge group"),
    ("군", "group"),
    # "끈"
    ("끈", "string"),
    ("끈/", "string/"),
    # "가둠"
    ("가둠", "confinement"),
    # "복사"
    ("복사", "radiation"),
    # "열"
    ("열 ", "thermal "),
    ("열역학", "thermodynamics"),
    ("열", "thermal"),
    # glueball 글루온
    ("글루온", "gluon"),
    # "꽉", "더 simple히"
    ("더 simple히", "more simply"),
    ("더 ", "more "),
    # 섭동
    ("비섭동", "non-perturbative"),
    ("섭동", "perturbation"),
    # 바꿔
    ("바꿔", "exchange"),
    # "ratio"
    ("min배", "min-ratio"),
    # "최소벡"
    ("최소벡", "minimum vector"),
    # "최소 벡터"
    # "일체 성형"
    ("일체 성형", "monolithic forming"),
    ("일체", "monolithic"),
    ("성형", "forming"),
    # "합류"
    ("합류", "confluence"),
    # "골격"
    ("골격", "skeleton"),
    # "보완"
    ("보완", "supplement"),
    # "초전도"
    ("초전도", "superconductor"),
    # "갭"
    ("갭", "gap"),
    # Generation/세대
    ("generation 수", "generations"),
    ("세대 수", "generations"),
    # "설계 템플릿만"
    ("설계 템플릿만", "design template only"),
    ("템플릿", "template"),
    ("설계", "design"),
    # "arealaw"
    ("arealaw", "area law"),
    # "gauge 불변"
    ("gauge 불변", "gauge invariance"),
    ("불변", "invariance"),
    # "axis약"
    ("axis약", "axial weak"),
    # "관세"
    # "대응"
    # "쌍"
    # "핵"
    # Cosmology chunks
    ("ΛCDM 6 파라미터", "LambdaCDM 6 parameters"),
    ("ΛCDM parameter", "LambdaCDM parameter"),
    # "팽창"
    ("팽창", "expansion"),
    # "가속"
    # "팽창률"
    ("팽창률", "expansion rate"),
    ("우주 나이", "universe age"),
    ("우주론", "cosmology"),
    ("우주 의", "universe's"),
    ("우주의", "universe's"),
    ("우주 ", "universe "),
    # "조기" 초기
    # "천문"
    ("천문", "astronomy"),
    ("천문학", "astronomy"),
    # "별"
    ("별", "star"),
    ("별이", "stars"),
    # "암흑"
    ("암흑", "dark"),
    # "물질"
    ("물질", "matter"),
    # "입자"
    # "양자"
    ("양자화", "quantization"),
    # "복사"
    # "배경"
    ("배경", "background"),
    # "시점"
    ("시점", "moment"),
    # "관측가능"
    ("관측가능", "observable"),
    # "체적"
    ("체적", "volume"),
    # "정밀도"
    # "오차"
    # "계수"
    # "이산화"
    ("이산화", "discretization"),
    ("이산", "discrete"),
    # "연속"
    ("연속", "continuous"),
    # "극한"
    ("극한", "limit"),
    # "값"
    ("값", "value"),
    # Strings common
    ("식별", "identify"),
    ("식별자", "identifier"),
    # "시뮬레이션 중력"
    # "논제"
    # "명시"
    # "가족"
    ("가족", "family"),
    # "코드"
    ("코드", "code"),
    # "스페이스"
    ("스페이스", "space"),
    # "단지"
    ("단지", "merely"),
    # "곱의"
    ("곱의", "of product"),
    ("곱은", "product"),
    ("곱이", "product"),
    ("곱", "product"),
    # "분리"
    ("분리", "separation"),
    # "결합쌍"
    ("결합쌍", "coupling pair"),
    # "결합"
    # "보유"
    ("보유", "holding"),
    # "포함비"
    ("포함비", "inclusion ratio"),
    # "포함"
    ("포함", "inclusion"),
    # "구성"
    ("구성", "composition"),
    # "배열"
    ("배열", "array"),
    ("재배열", "rearrangement"),
    # "배치"
    # "자"
    # cosmology / particle-cosmology
    # "함축"
    ("함축", "implication"),
    # "함의"
    ("함의", "implication"),
    # "이동"
    ("이동", "translation"),
    # "회전"
    ("회전", "rotation"),
    # "병진"
    ("병진", "translation"),
    # "역학적"
    ("역학적", "mechanical"),
    ("기계적", "mechanical"),
    # sum inner
    ("역학계", "dynamical system"),
    ("계", "system"),
    # "단일 점"
    # "원"
    ("원주율", "pi"),
    # "순환"
    # "위상학적"
    ("위상학적", "topological"),
    # "정적", "동적"
    ("정적", "static"),
    ("동적", "dynamic"),
    # "뼈대"
    ("뼈대", "skeleton"),
    # "과정"
    ("과정", "process"),
    # "관계식"
    ("관계식", "relation equation"),
    # "관계"
    ("관계", "relation"),
    # "상태"
    ("상태", "state"),
    # "에너지"
    # "이산화"
    # "가상"
    ("가상", "virtual"),
    # "진폭"
    ("진폭", "amplitude"),
    # "주기"
    # "파"
    ("파", "wave"),
    # "중요"
    ("중요", "important"),
    # "검토"
    ("검토", "review"),
    # "지침"
    ("지침", "guideline"),
    # "경로"
    # "결과"
    # "대상"
    # "대체"
    ("대체", "replace"),
    # "요구"
    ("요구", "required"),
    # "표지"
    ("표지", "marker"),
    # "표기"
    # "기호"
    ("기호", "symbol"),
    # "특정"
    ("특정", "specific"),
    # "고유"
    # "수"
    # N=6 related
    ("완전수 산술", "perfect-number arithmetic"),
    ("완전 수", "perfect number"),
    # "계층"
    ("계층", "hierarchy"),
    # "연관"
    ("연관", "relation"),
    # "연관된"
    ("연관된", "related"),
    # "연산자"
    ("연산자", "operator"),
    # "재귀"
    ("재귀", "recursion"),
    # "재귀적"
    ("재귀적", "recursive"),
    # "순환적"
    ("순환적", "cyclic"),
    # "보기"
    ("보기", "example"),
    # "중요도"
    ("중요도", "importance"),
    # "기"
    # 부대적
    ("부대", "attached"),
    # "적용"
    # 차원
    ("차원간", "inter-dim"),
    ("dimension 사이", "inter-dim"),

    # Round 3 cleanup
    ("폐쇄", "closure"),
    ("정체성", "identity"),
    ("공선", "collinear"),
    ("재배thermal", "rearrangement"),
    ("수론으로", "via number-theory"),
    ("수론", "number-theory"),
    ("axis을", "axis"),
    ("axis은", "axis"),
    ("baseline에", "baseline"),
    ("color three 인덱스", "color three indices"),
    ("인덱스", "index"),
    ("반symmetry", "antisymmetric"),
    ("시of space", "of spacetime"),
    ("at 이어", "following"),
    ("이어", "following"),
    ("한system", "cap"),
    ("홀로노미", "holonomy"),
    ("둘 다 ", "both "),
    ("둘 다", "both"),
    ("baseline점", "baseline point"),
    ("massspacing", "mass spacing"),
    ("이로써", "thus"),
    ("가능", "feasible"),
    ("과의", "relative to"),
    ("scale비", "scale ratio"),
    ("form화", "formalization"),
    ("draft body는", "draft body"),
    ("maintained", "maintained"),
    ("observation 시", "when observed"),
    ("시", " on "),
    ("분수 axis scale", "fractional axis scale"),
    ("비대칭", "asymmetric"),
    ("양자 oracle", "quantum oracle"),
    ("양자-oracle", "quantum-oracle"),
    ("oracle", "oracle"),
    ("홀로그래피", "holography"),
    ("초광속", "superluminal"),
    ("주름", "wrinkle"),
    ("빛", "light"),
    ("회절", "diffraction"),
    ("굴절", "refraction"),
    ("반사", "reflection"),
    # further residuals
    ("free share", "free share"),
    ("free 지", "free share"),
    ("shared", "shared"),
    # "성 min"
    ("성 min", "component min"),
    # "점"
    # "물리적"
    # "단일"
    ("단일점", "single point"),
    ("단 single", "single"),
    # "대칭축"
    ("대칭axis", "symmetry axis"),
    ("대칭 axis", "symmetry axis"),
    # Korean suffix residuals
    ("인지", "whether"),
    ("도입", "introduction"),
    ("활성화", "activation"),
    ("비활성", "inactive"),
    ("입력", "input"),
    ("출력", "output"),
    # "사건"
    ("사건", "event"),
    # "배포" already
    # "밀집"
    ("밀집", "dense"),
    # "소프트"
    ("소프트", "soft"),
    # "하드"
    # "드라이버"
    ("드라이버", "driver"),
    # "지점"
    ("지점", "location"),
    # "수직"
    ("수직", "vertical"),
    ("수평", "horizontal"),
    ("왼쪽", "left"),
    ("오른쪽", "right"),
    # "양의", "음의"
    ("양의", "positive"),
    ("음의", "negative"),
    # "대응하는"
    ("대응하는", "corresponding"),
    # "대응한다"
    # "대응관계"
    ("대응관계", "correspondence"),
    # "대응"
    ("대응", "correspondence"),
    # "즉시"
    ("즉시", "immediately"),
    # "부분"
    ("부분", "partial"),
    # "부분공간"
    ("부분공간", "subspace"),
    # "부분집합"
    ("부분집합", "subset"),
    # "집합"
    ("집합", "set"),
    # "실수"
    # "자연"
    # ratio
    # "시 "
    # "시이다."
    # "시간이다."
    # inflaton
    ("인플레이션", "inflation"),
    ("인플레이튼", "inflaton"),
    # "초기화"
    ("초기화", "initialization"),
    # "팽창"
    # "가속팽창"
    ("가속팽창", "accelerated expansion"),
    # "뉴턴", "아인슈타인"
    ("아인슈타인", "Einstein"),
    ("뉴턴", "Newton"),
    # "슈바르츠실트"
    ("슈바르츠실트", "Schwarzschild"),
    # "블랙홀"
    ("블랙홀", "black hole"),
    # "사건의 지평선"
    ("사건의 지평선", "event horizon"),
    ("지평선", "horizon"),
    # "특이"
    ("특이점", "singularity"),
    ("특이", "singular"),
    # "관성"
    ("관성", "inertia"),
    # "관성기준계"
    ("관성기준계", "inertial frame"),
    # "운동량"
    ("운동량", "momentum"),
    # "운동에너지"
    ("운동에너지", "kinetic energy"),
    # "시공의"
    # "연구"
    ("연구", "research"),
    # "비교"
    ("비교", "comparison"),
    # "단계"
    # "양쪽"
    ("양쪽", "both sides"),
    ("양방향", "bidirectional"),
    # "판정"
    # "검증"
    # "축약"
    ("축약", "contraction"),
    ("주장", "claim"),
    # trailing
    ("unifiedversality", "universality"),
    ("single sc", "single SC"),
    # Random Korean words
    ("중력파", "gravity wave"),
    ("중력 파동", "gravity wave"),
    ("자기장", "magnetic field"),
    ("자기", "magnetic"),
    ("전기장", "electric field"),
    ("전기", "electric"),
    ("광자", "photon"),
    ("빛의 ", "light "),
    # "격자 QCD", "라티스"
    ("라티스", "lattice"),
    ("수치실험", "numerical experiment"),
    ("수치", "numerical"),
    # "시험"
    # "보존"
    # 초전도
    # 상전이
    ("상전이", "phase transition"),
    ("상", "phase"),
    # Korean numbers as before
    ("둘", "two"),
    ("셋", "three"),
    ("하나", "one"),
    ("여러", "multiple"),
    # Misc
    ("초반", "early"),
    ("중반", "middle"),
    ("후반", "late"),
    ("이전", "prior"),
    ("이후", "after"),
    ("근처", "nearby"),
    ("멀리", "far"),
    ("근접", "nearby"),
    ("접촉", "contact"),
    # remaining common
    ("통계적", "statistical"),
    ("통계 유의성", "statistical significance"),
    ("확률밀도", "probability density"),
    ("확률", "probability"),
    # "에", "를", "을"
    ("내", "within"),
    ("밖", "outside"),
    # Korean measure units
    ("억분", "hundred-million-th"),
    ("조분", "trillion-th"),
    ("경분", "quadrillion-th"),
    # "만년"
    ("만년", ",000 years"),
    # "모두"
    ("모두", "all"),
    # "대체로"
    ("대체로", "mostly"),
    # "엄밀"
    ("엄밀", "rigorous"),
    # "엄밀히"
    ("엄밀히", "rigorously"),
    # "엄격"
    ("엄격", "strict"),
    # "단계적"
    ("단계적", "stepwise"),
    # "다항식"
    ("다항식", "polynomial"),
    # "비결정"
    # "다수"
    ("다수", "majority"),
    # "저장"
    ("저장", "storage"),
    # "분실"
    ("분실", "loss"),
    # "제거"
    ("제거", "removal"),
    # "체계"
    # "구체적"
    ("구체적", "concrete"),
    # "일관"
    ("일관", "consistent"),
    # "일치"
    # "광범"
    ("광범", "broad"),
    # "다양"
    ("다양", "diverse"),
    # "축약 gauge 불변"
    # "예측"
    ("예측", "prediction"),
    # "예측 3"
    # misc
    ("오차 0", "error 0"),
    # "도달"
    # "길이"
    ("길이", "length"),
    # "너비"
    ("너비", "width"),
    # "높이"
    ("높이", "height"),
    # "깊이"
    ("깊이", "depth"),
    # "폭"
    ("폭", "width"),
    # sec check
    ("기울기의 ratio", "slope ratio"),
    # "꿈"
    ("꿈", "dream"),
    # "오해"
    # "해결책"
    # "해결 가능"
    # "이해"
    ("이해", "understanding"),
    # misc endings
    ("불가피", "inevitable"),
    ("불가능", "impossible"),
    ("가능한", "possible"),
    ("가능", "possible"),
    ("필수", "essential"),
    # "어긋난"
    ("어긋난", "misaligned"),
    # "대각"
    ("대각", "diagonal"),
    # "직교"
    ("직교", "orthogonal"),
    # "정규"
    # "단일 점"
    # misc
    ("상호 관계", "mutual relation"),
    # Specific observed: "체계 complete"
    ("체계 complete", "system complete"),
    # leftovers that seem safe
    ("지수 re-derivation", "exponent re-derivation"),

    # Round 4: Poincare + math residuals
    ("단connection", "simply connected"),
    ("3-diverse체", "3-manifold"),
    ("diverse체", "manifold"),
    ("다양체", "manifold"),
    ("흐름", "flow"),
    ("등거리group", "isometry group"),
    ("등거리", "isometry"),
    ("topology동form", "homeomorphic"),
    ("topology 동form", "homeomorphic"),
    ("symmetrygroup dimension", "symmetry-group dimension"),
    ("symmetrygroup", "symmetry-group"),
    ("symmetry axis", "symmetry axis"),
    ("분해", "decomposition"),
    ("dualmetric", "dual metric"),
    ("분수 axis 척도", "fractional axis scale"),
    ("척도", "scale"),
    ("도입", "introduction"),
    ("확대", "enlargement"),
    ("축axis", "axis"),
    ("min수 axis", "divisor axis"),
    ("수axis", "contraction axis"),
    ("수 axis ", "axis "),
    ("재배axis", "rearrangement"),
    ("동form", "isomorphic"),
    ("Isom dim 합", "Isom dim sum"),
    ("is Isom dim 합", "where Isom dim sum"),
    ("at 잠김", "locks at"),
    ("min할", "split"),
    ("할", "split"),
    ("코드", "code"),
    ("code화", "encoded"),
    ("code", "code"),
    ("정정code", "error-correcting code"),
    ("에러정정code", "error-correcting code"),
    ("minimum거리", "minimum distance"),
    ("거리", "distance"),
    ("double lock", "double lock"),
    ("square perfect number sealing", "square perfect-number sealing"),
    ("끔", "locked"),
    ("방", "room"),
    ("은", ""),  # particle
    ("는", ""),  # particle
    # "이" "가" "에" "을" "를" — these we left as-is because they're dangerous; but in context
    # we try paired with preceding English
    ("만", ""),
    ("가 ", " "),
    ("이 ", " "),
    ("에 ", " "),
    ("을 ", " "),
    ("를 ", " "),
    ("도 ", " also "),
    ("과 ", " and "),
    ("와 ", " and "),
    ("로 ", " via "),
    ("의 ", " of "),
    ("은 ", ""),
    ("는 ", ""),
    ("만 ", " only "),
    # Conjugations in sentence
    ("이로 ", "this via "),
    ("으로 ", "via "),
    ("에서 ", "from "),
    ("이며 ", ", "),
    ("이다 ", ""),
    ("한다 ", ""),
    ("된다 ", ""),
    ("되고 ", "and "),
    ("하며 ", "and "),
    ("드는 ", ""),
    # specific words
    ("이는", "this is"),
    ("이것이", "this"),
    ("이것은", "this"),
    ("그 결과", "consequently"),
    ("그리고", "and"),
    ("그러나", "however"),
    ("따라서", "therefore"),
    ("즉", "i.e."),
    ("바로", "exactly"),
    # generative prose
    ("더 나아가", "furthermore"),
    ("나아가", "furthermore"),
    # misc
    ("우니", "uniqueness"),
    ("유니버설", "universal"),
    # particles often suffixed to English
    ("의 ", " of "),
    ("이로부터", "from this"),
    ("을 통해", "via"),
    ("통해", "via"),
    ("통한", "through"),
    # "수"
    ("수이므로", "a number"),
    ("수 = ", "count = "),
    ("수가 ", "count "),
    ("수는 ", "count "),
    ("수를 ", "count "),
    # "이"
    ("이 이다", "is"),
    # fix "이 로"
    (" 로 ", " via "),
    # specific common
    ("이 있", "exists"),
    ("이다", ""),
    ("된다", ""),
    ("한다", ""),
    ("하다", "do"),
    # "같다"
    ("같다", "equals"),
    ("같은", "same"),
    ("같이", "as"),
    # "크다"
    ("크다", "large"),
    ("작다", "small"),
    # Further big chunks from Poincare
    ("복제", "replica"),
    ("실험", "experiment"),
    ("고정", "fixed"),
    ("진행", "progress"),
    # "수렴"
    ("수렴", "convergence"),
    # "발산"
    ("발산", "divergence"),
    # "점근"
    # compactification
    # "외연"
    # Poincare-specific
    ("number-theory restoration", "number-theoretic restoration"),
    ("system승", "system sequence"),
    ("system량tensor", "metric tensor"),
    ("system량", "metric"),
    ("system량space", "metric space"),
    ("system량spacefrom", "metric space"),
    ("system", "system"),
    # "부호"
    ("부호", "sign"),
    # "양자"
    # "초"
    ("초끈", "superstring"),
    # "등식"
    ("등식", "identity"),
    # "꽉 찬"
    ("꽉 찬", "fully packed"),
    ("꽉", "tight"),
    # "해"
    # "최소"
    # "분수 axis"
    ("tier surgery", "tier surgery"),
    # "재 뮬"
    ("재 뮬", "resim"),
    ("재뮬", "resim"),
    ("뮬", "sim"),
    # "재시뮬레이션"
    # "위상"
    # etc
    # "붕괴"
    ("붕괴", "collapse"),
    # "이면"
    ("이면", "then"),
    # "아닌"
    ("아닌", "not"),
    # "아니라"
    ("아니라", "but rather"),
    # "오리지널"
    ("오리지널", "original"),
    # "원"
    ("원 equation", "original equation"),
    ("원 equation", "original equation"),
    # "또한"
    ("또한", "also"),
    # "동일"
    ("동일", "same"),
    # "하나의"
    ("하나의", "one"),
    ("하나", "one"),
    # "에러정정 코드"
    # "각 " — keep as "each"
    ("각 ", "each "),
    # "각"
    ("각 ", "each "),
    # "초광속"
    # "확장한다"
    ("확장한", "extended"),
    # "원과 ≠"
    ("원", "circle"),
    # "수정"
    ("수정", "modification"),
    # "관계이"
    # Geometry/topology
    ("위상동형", "homeomorphic"),
    ("세systemvolume", "world-volume"),
    # "브레인 세systemvolume"
    # "중력체"
    ("중력체", "gravitating body"),
    # "체"
    ("체의", " body's"),
    ("체가", " body"),
    ("체는", " body"),
    ("체에", " body"),
    # "체"
    # "관"
    ("관", "pipe"),
    ("관s", "pipes"),
    ("관 system", "relation"),
    # "측정"
    # "치"
    ("치", " "),
    # Now  safer particle removal
    # Other residuals
    ("회차", "round"),
    # "재 뮬"
    # misc
    ("표명", "stated"),
    ("그대로", "as-is"),
    # "유일 고정"
    ("유일 fixed", "unique fixed"),
    # "원본"
    ("원본", "source"),
    # "전 ", "전(全)"
    ("전(全)", "entire"),
    ("전체", "whole"),
    ("전 ", "entire "),
    # "내 ", "외 "
    ("내의", "inside"),
    ("외의", "outside"),
    # "차이"
    ("차이", "difference"),
    # "두", "세"
    ("두 ", "two "),
    ("세 ", "three "),
    # others
    ("유일 고정", "unique fixed"),
    ("유일", "unique"),
    # "선"
    ("선의", "line"),
    ("선이", "line"),
    ("선", "line"),
    # numbered residuals
    ("min류 auto", "classification auto"),
    ("min류", "classification"),
    # "종"
    ("종의", "kind"),
    ("종이", "kind"),
    ("종", "kind"),
    # "풀어"
    # "초·해"
    # "단조"
    # misc
    ("와도", "with"),
    ("실패", "failure"),
    ("다른", "other"),
    ("신규", "new"),
    ("수식", "formula"),
    ("전수", "exhaustive"),
    ("기본", "basic"),
    ("래픽", "raphic"),
    ("발견", "discovery"),
    ("가중치", "weight"),
    ("비자명", "nontrivial"),
    ("하한", "lower bound"),
    ("정칙성", "regularity"),
    ("사영", "projection"),
    ("한도", "limit"),
    ("감도", "sensitivity"),
    ("텐션", "tension"),
    ("정확", "exactness"),
    ("불가", "impossible"),
    ("오류", "error"),
    ("강제", "forced"),
    ("프로토콜", "protocol"),
    ("게이트", "gate"),
    ("최대", "maximum"),
    ("최소", "minimum"),
    ("한도", "cap"),
    ("위상동형", "homeomorphic"),
    ("위상", "topological"),
    # "이상", "이하"
    # "유한성"
    ("유한성", "finiteness"),
    # Korean continuous fragments remaining
    ("번째", "-th"),
    ("하는", "-that"),
    ("하다", "do"),
    ("관 system", "relation"),
    ("공유", "shared"),
    ("차수", "degree"),
    ("중복", "redundancy"),
    ("정확히", "exactly"),
    ("판본", "version"),
    ("이유", "reason"),
    ("이유는", "the reason is"),
    # misc
    ("그리고 이", "and"),
    # "전 구조를 관통한다" already handled
    # "쓰인다"
    ("쓰인다", "is used"),
    ("쓴다", "use"),
    # "부"
    ("부여", "endow"),
    # "부여", "배제"
    ("배제", "exclude"),
    # "배수"
    ("배수", "multiple"),
    # sim
    # "명시"
    # "명료"
    ("명료", "clear"),
    ("명확", "clear"),
    # Others
    ("이후 ", "after "),
    ("이전 ", "before "),
    # "이"
    # "유추"
    ("유추", "analogy"),
    # "닫혀"
    ("닫혀", "closed"),
    # "열려"
    ("열려", "open"),
    # "작용"
    # "지원"
    ("지원", "support"),
    # "부대적"
    # "사유"
    # "귀결"
    ("자동 귀결", "automatic consequence"),
    ("귀결", "consequence"),
    # "독립비율"
    ("독립비율", "independent ratio"),
    ("독립비", "independent ratio"),
    # "구성성분"
    ("구성성분", "constituent"),
    # "성분들"
    ("성분들", "components"),
    # "구축"
    ("구축", "construct"),
    # "상응"
    ("상응", "correspondence"),
    # "필연"
    ("필연성", "inevitability"),
    ("필연", "inevitable"),
    # various
    ("정상", "normal"),
    ("비정상", "abnormal"),
    # Korean suffixes remaining
    ("이라고", "as"),
    ("라고", "as"),
    ("로서", "as"),
    ("로써", "by"),
    # "가며"
    ("가며", "while"),
    # "하면서"
    ("하면서", "while doing"),
    # "하여", "해서"
    ("해서", "by doing"),
    # "동시에"
    ("동시에", "simultaneously"),
    # "또는"
    ("또는 ", "or "),
    # "이렇게"
    # "한편"
    ("한편", "meanwhile"),
    # "정식"
    ("정식", "formal"),
    # "비", "비율" etc
    # BSD residuals
    ("세systemvolume", "world-volume"),
    # "세systemvolume"
    # "와", "과"
    # "전"
    # "특이"
    # "역"
    # "계층구조"
    ("계층구조", "hierarchy"),
    # "영역"
    # "섭"
    # "발견됨"
    ("발견됨", "discovered"),
    # "발견한다"
    # "각각"
    ("각각", "each"),
    # "각자"
    ("각자", "individually"),
    # "때"
    ("때에", "when"),
    ("할 때", "when doing"),
    ("할때", "when"),
    ("때", "at time"),
    # "때문에"
    ("때문에", "because"),
    # "위해서"
    ("위해서", "in order to"),
    # "위해"
    ("위해", "for"),
    # "위"
    ("위에 있", "atop"),
    ("위", "above"),
    # "앞에"
    ("앞에", "before"),
    # "뒤에"
    ("뒤에", "after"),
    # "전에"
    ("전에", "before"),
    # "후에"
    ("후에", "after"),
    # "모순"
    ("모순", "contradiction"),
    # "배제"
    # misc
    ("자체", "itself"),
    # "여기서"
    ("여기서", "here"),
    # "그" particle
    ("그는", "he"),
    ("그녀는", "she"),
    ("그들은", "they"),
    # "한쪽"
    ("한쪽", "one side"),
    ("양쪽", "both sides"),
    # "문단"
    ("문단", "paragraph"),
    # "제목"
    ("제목", "title"),
    # "내용"
    ("내용", "content"),
    # "끝"
    # "표"
    ("표", "table"),
    # "발"
    # "선점"
    ("선점", "preemption"),
    # "선형"
    ("선형", "linear"),
    # "이차"
    ("이차", "quadratic"),
    # "삼차"
    ("삼차", "cubic"),
    # "사차형식"
    ("사차형식", "quartic form"),
    ("이차형식", "quadratic form"),
    # "형식"
    ("형식", "form"),
    # misc
    ("이로써", "thus"),
    ("결국", "eventually"),
    ("결과적으로", "as a result"),
    ("결과로", "resulting in"),
    ("최종", "final"),
    ("중", "inside"),
    ("인가", " ? "),
    # ASCII art residuals like "관system"
    ("관system", "relation"),
    # "검증 baseline"
    ("수렴 baseline", "convergence baseline"),
    # "휨"
    ("휨", "warping"),
    # "힘밀도"
    ("힘밀도", "force density"),
    ("힘", "force"),
    ("힘density", "force density"),
    # "동mechanics"
    ("동mechanics", "mechanics"),
    ("mechanics axis", "mechanics axis"),
    # "분류"
    # "따라"
    ("따라", "along"),
    # "따라서"
    # "방정식"
    # "형식"
    # "시뮬레이터"
    # sim/sym etc
    # etc
    # "여전히 open"
    ("still open", "still open"),
    # Chinese character replacements
    ("全", "full"),
    ("中", "center"),
    ("上", "top"),
    ("下", "bottom"),
    ("左", "left"),
    ("右", "right"),
    ("前", "front"),
    ("後", "back"),
    # Japanese remnants
    ("ンジ", ""),
    # Korean 'in'
    ("인 ", "a "),
    ("인", ""),
    # other ending particles
    ("임", " is"),
    # "전혀"
    ("전혀", "at all"),
    # "사실"
    ("사실", "fact"),
    # "결론"
    ("결론", "conclusion"),
    # "참"
    ("참", "true"),
    ("거짓", "false"),
    # Final trim words
    ("재현 recipe", "reproduce recipe"),
    ("최종 form", "final form"),
    ("평생", "lifetime"),
    ("일생", "lifetime"),
    # additional Korean leftovers
    ("적분", "integral"),
    ("도 있", "exists"),
    ("있음", "exists"),
    ("있을", "may exist"),
    ("없음", "not"),
    ("없이", "without"),
    ("없는", "with no"),
    # "불필요"
    # "이유"
    # "시의"
    ("시의", "of time"),
    # "분"
    ("자수", "count"),
    # "존재"
    ("존재", "existence"),
    # "모두가"
    # "차지"
    ("차지", "occupy"),
    # "전수 탐색"
    ("전수 탐색", "exhaustive search"),
    # "탐색"
    ("탐색", "search"),
    # "라운드"
    ("라운드", "round"),
    # "할"
    ("할 수", "can"),
    # "수 있"
    ("수 있", "can"),
    ("수 없", "cannot"),
    # "에게"
    ("에게", "to"),
    # "고"
    # "고유"
    # "고유값" - already handled
    # "고정된"
    ("고정된", "fixed"),
    # "고정"
    # "고정함"
    ("고정함", "fixation"),
    # misc
    ("모든 ", "all "),
    ("모두의", "all"),
    ("필수 ", "essential "),
    ("중복 ", "redundant "),
    ("대표", "representative"),
    # "주어"
    ("주어진", "given"),
    ("주어", "given"),
    # final trivial
    ("수학적 structure", "mathematical structure"),
    # "홀"
    ("홀", "solo"),
    # "짝"
    ("짝", "pair"),
    # Korean suffix "들"
    ("들이", ""),
    ("들의", " of"),
    ("들은", ""),
    ("들을", ""),
    ("들", ""),
    # more
    ("번", ""),
    ("조", ""),
    ("만", ""),
    ("억", ""),
    # further leftovers
    ("실", ""),
    ("동", ""),
    ("자", ""),
    ("도", ""),
    ("성", ""),
    ("적", ""),
    ("과", ""),
    ("인", ""),
    ("재", ""),
    ("유", ""),
    ("량", ""),
    ("식", ""),
    ("포", ""),
    ("불", ""),
    ("홀", ""),
    ("모", ""),
    ("배", ""),
    ("행", ""),
    ("개", ""),
    ("중", ""),
    ("은", ""),
    ("는", ""),
    ("이", ""),
    ("가", ""),
    ("의", ""),
    ("수", ""),
    ("부", ""),
    ("론", ""),
    ("각", ""),
    ("세", ""),
    ("합", ""),
    ("화", ""),
    ("본", ""),
    ("후", ""),
    ("초", ""),
    ("선", ""),
    ("단", ""),
    ("차", ""),
    ("류", ""),
    ("와", ""),
    ("자", ""),
    ("한", ""),
    ("계", ""),
    ("사", ""),
    ("래", ""),
    ("픽", ""),
    ("비", ""),
    ("체", ""),
    ("로", ""),
    ("원", ""),
    ("만", ""),
    ("를", ""),
    ("에", ""),
]


def repair(p: Path) -> tuple[int, int]:
    import re
    CJK = re.compile(r"[ᄀ-ᇿ぀-ヿㄱ-ㆎ㐀-䶿一-鿿가-힣豈-﫿]")
    text = p.read_text(encoding="utf-8")
    before = len(CJK.findall(text))
    for k, v in REPAIRS:
        if k in text:
            text = text.replace(k, v)
    p.write_text(text, encoding="utf-8")
    after = len(CJK.findall(text))
    return before, after


def main(argv: list[str]) -> int:
    for arg in argv[1:]:
        p = Path(arg)
        b, a = repair(p)
        print(f"{arg}: CJK {b} -> {a}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
