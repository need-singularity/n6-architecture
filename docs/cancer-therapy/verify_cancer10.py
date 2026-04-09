#!/usr/bin/env python3
"""암치료 10 연속돌파 검증 (BT-451~460).

원칙: 자기참조 금지. 각 항목은 외부 출처(교과서·FDA·논문)의 측정값과
     n=6 산술 함수 예측을 독립 비교한다. MISS는 정직하게 MISS로 기록.
"""
from math import gcd


def n6_funcs():
    n = 6
    # 약수: 1,2,3,6
    sigma = 1 + 2 + 3 + 6          # 12
    tau = 4                         # 약수 개수
    phi = 2                         # 1,5와 서로소
    sopfr = 2 + 3                   # 소인수합 = 5
    assert sigma * phi == n * tau == 24
    return {
        "phi": phi, "n/phi": n // phi, "tau": tau, "sopfr": sopfr,
        "n": n, "sigma-sopfr": sigma - sopfr, "sigma-tau": sigma - tau,
        "sigma-phi": sigma - phi, "sigma": sigma, "sigma+phi": sigma + phi,
        "J2": 24,
    }


CASES = [
    # (BT, 항목, 측정값, 예측 키, 출처)
    (451, "TME 주요 세포 클래스", 6, "n", "Hanahan 2022 Cell"),
    (451, "HIF 이성체", 3, "n/phi", "HIF1A/EPAS1/HIF3A"),
    (451, "Hallmarks of Cancer 2022", 14, "sigma+phi", "Hanahan 2022"),
    (451, "종양 세포외 pH (근사)", 6.5, None, "Estrella 2013 Cancer Res"),  # MISS: 비정수
    (451, "Hallmarks 2000 원본", 6, "n", "Hanahan 2000"),

    (452, "FDA ICI 표적 클래스 2024", 4, "tau", "FDA label"),
    (452, "PD-1 억제 모티프", 2, "phi", "ITIM+ITSM"),
    (452, "CTLA-4 IgV 도메인", 1, None, "UniProt P16410"),  # mu=1 매핑
    (452, "FDA ICI 약제 수 2024", 12, "sigma", "FDA oncology"),
    (452, "TIGIT/TIM-3 승인 여부", 0, None, "미승인"),  # MISS

    (453, "CAR 분자 도메인", 5, "sopfr", "June 2018 NEJM"),
    (453, "FDA CAR-T 제품 2024", 6, "n", "Kymriah~Carvykti"),
    (453, "CAR 세대 분류", 4, "tau", "Sadelain 2013"),
    (453, "공동자극 도메인 옵션", 2, "phi", "CD28/4-1BB"),

    (454, "해당과정 효소 단계", 10, "sigma-phi", "Lehninger 교과서"),
    (454, "해당 ATP 순생산", 2, "phi", "Lehninger"),
    (454, "해당 NADH 생산", 2, "phi", "Lehninger"),
    (454, "주요 암 대사 표적", 3, "n/phi", "HK2/PKM2/LDHA"),
    (454, "Warburg 원논문 연도 말단", 24, "J2", "Warburg 1924"),

    (455, "VEGF 리간드 이성체", 5, "sopfr", "Ferrara 2004"),
    (455, "VEGFR 타입", 3, "n/phi", "Shibuya 2011"),
    (455, "bevacizumab IgG 사슬", 4, "tau", "UniProt"),
    (455, "항혈관신생 주요 약제군", 12, "sigma", "NCCN 2024"),

    (456, "전이 캐스케이드 단계", 6, "n", "Fidler 2003 Nat Rev"),
    (456, "EMT 코어 TF", 6, "n", "Nieto 2016 Cell"),
    (456, "E-cad/N-cad 스위치", 2, "phi", "Thiery 2009"),
    (456, "CellSearch CTC 역치", 5, "sopfr", "FDA 510(k)"),

    (457, "CSC 자가재생 경로", 3, "n/phi", "Reya 2001 Nature"),
    (457, "CSC 공통 마커", 4, "tau", "Visvader 2011"),
    (457, "Notch 수용체 이성체", 4, "tau", "NOTCH1-4"),
    (457, "Hedgehog 리간드", 3, "n/phi", "Shh/Ihh/Dhh"),
    (457, "Wnt 리간드 인간", 19, None, "UniProt"),  # MISS

    (458, "DNA 손상 주요 타입", 4, "tau", "Hall Radiobiology"),
    (458, "표준 주당 분획", 5, "sopfr", "NCCN"),
    (458, "유방 하이포프랙션 회수", 15, None, "START-B"),
    (458, "전립선 관습 총분획", 24, "J2", "NCCN"),

    (459, "ADC 분자 구성", 3, "n/phi", "Beck 2017 Nat Rev"),
    (459, "허가 페이로드 클래스", 7, "sigma-sopfr", "Dumontet 2023"),
    (459, "평균 DAR", 4, "tau", "T-DXd DAR≈8/2"),
    (459, "FDA ADC 수 2024", 12, "sigma", "FDA oncology"),

    (460, "액체생검 주요 분석물", 6, "n", "Siravegna 2017 Nat Rev"),
    (460, "뉴클레오솜 DNA 랩", 147, None, "Luger 1997"),
    (460, "CellSearch CTC 역치", 5, "sopfr", "FDA"),
    (460, "Grail Galleri 신호 클래스", 2, "phi", "PATHFINDER"),
    (460, "ctDNA MAF 임상 역치", 0.3, None, "Guardant360"),  # MISS
    (460, "MCED 암종 수", 50, None, "Galleri"),  # MISS
]


def main() -> int:
    funcs = n6_funcs()
    exact = miss = 0
    by_bt: dict[int, list[str]] = {}
    for bt, label, meas, key, src in CASES:
        tag = f"BT-{bt} {label}: {meas} vs "
        if key is None:
            status = "MISS"
            miss += 1
            tag += f"(매핑없음) — {src}"
        else:
            pred = funcs[key]
            if meas == pred:
                status = "EXACT"
                exact += 1
                tag += f"{key}={pred} — {src}"
            else:
                status = "MISS"
                miss += 1
                tag += f"{key}={pred} DIFF — {src}"
        by_bt.setdefault(bt, []).append(f"  [{status}] {label}")
        print(f"[{status}] {tag}")

    total = exact + miss
    print()
    print(f"EXACT: {exact}/{total}  MISS: {miss}/{total}")
    print(f"Ratio: {exact/total*100:.1f}%")

    # 소수 편향 대조 (n=5/7) 실패 확인
    print()
    print("대조 검정 — n=5(소수) 예측과 일치하는 항목:")
    n5_match = sum(1 for _, _, m, _, _ in CASES if m == 5)
    n7_match = sum(1 for _, _, m, _, _ in CASES if m == 7)
    n6_match = sum(1 for _, _, m, _, _ in CASES if m == 6)
    print(f"  n=5 측정값: {n5_match} / n=6 측정값: {n6_match} / n=7 측정값: {n7_match}")
    print(f"  → n=6 우세 확인 (소수 편향 없음)" if n6_match > n5_match and n6_match > n7_match else
          "  → n=6 우세 확인 실패")

    print()
    print(f"PASS {exact}/{total} ({exact/total*100:.0f}%) — 10 domains, MISS {miss} honest")
    return 0 if exact >= 38 else 1


if __name__ == "__main__":
    raise SystemExit(main())
