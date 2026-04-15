#!/usr/bin/env python3
# v3 E1: Pari-GP wrapper for elliptic curve Selmer computations
# Sage 대체 — Pari-GP 2.17.3 Mac ARM brew 설치 확인 (2026-04-16 loop 18)
#
# Scope: Cremona curve 에 대한 Pari 의 ellrank() / elltors() / ellL1() 호출
# Sage 수준의 selmer_group(n) 은 Pari 단독으로 부분적 (2-descent 만 native)

import json
import subprocess
import sys
from pathlib import Path

PARI = "/opt/homebrew/bin/gp"


def pari_run(script: str, timeout: int = 30) -> str:
    """Pari-GP 스크립트 실행 → stdout 반환"""
    result = subprocess.run(
        [PARI, "-q", "--test"],
        input=script,
        capture_output=True,
        text=True,
        timeout=timeout,
    )
    return result.stdout.strip()


def ellrank_bound(a_invariants: list[int]) -> dict:
    """Cremona [a1,a2,a3,a4,a6] → Pari ellrank() 2-descent 결과"""
    a_str = ",".join(str(a) for a in a_invariants)
    script = f"""
E = ellinit([{a_str}]);
N = ellglobalred(E)[1];
r = ellrank(E);
print(N);
print(r[1]);
print(r[2]);
quit;
"""
    out = pari_run(script).split("\n")
    if len(out) < 3:
        return {"error": "pari output malformed", "raw": out}
    return {
        "a_invariants": a_invariants,
        "conductor": int(out[0]),
        "rank_lower": int(out[1]),
        "rank_upper": int(out[2]),
    }


def torsion_structure(a_invariants: list[int]) -> dict:
    """E(Q)_tors 의 Pari 직접 계산"""
    a_str = ",".join(str(a) for a in a_invariants)
    script = f"""
E = ellinit([{a_str}]);
T = elltors(E);
print(T[1]);
print(T[2]);
quit;
"""
    out = pari_run(script).split("\n")
    return {
        "torsion_order": int(out[0]) if out else 0,
        "structure_raw": out[1] if len(out) > 1 else "",
    }


def demo():
    """데모: Cremona 37a1 + 11a1 + 17a1 small curves"""
    curves = {
        "37a1": [0, 0, 1, -1, 0],   # rank 1
        "11a1": [0, -1, 1, -10, -20],  # rank 0, torsion 5
        "17a1": [1, -1, 1, -1, -14],  # rank 0
    }
    print("=== Pari-GP E.ellrank() demo (2-descent) ===")
    for name, a in curves.items():
        r = ellrank_bound(a)
        t = torsion_structure(a)
        print(f"  {name}: N={r['conductor']}, rank ∈ [{r['rank_lower']}, {r['rank_upper']}], torsion order={t['torsion_order']}")
    print()
    print("=== E1 산출 — 정직 선언 ===")
    print("  - Pari-GP 2.17.3 Mac ARM brew install 성공 (E1 MANDATORY 달성)")
    print("  - ellrank() 2-descent 은 |Sel_2| upper bound 만 제공")
    print("  - |Sel_3|, |Sel_6| 정밀 값은 Sage 필요 — E2/E3 여전히 DEFERRED")
    print("  - Sage Mac ARM 빌드는 ~1 hour + 의존성 이슈 — v3 이내 비실현")


if __name__ == "__main__":
    demo()
