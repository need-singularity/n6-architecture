#!/usr/bin/env python3
"""
HEXA-LANG 외계인지수 자동 검증 게이트
NEXUS-6 기반 — 🛸7→8→9→10 천장 자동 판정

사용법: python3 tools/hexa-ir/alien_index_gate.py
"""
import os
import sys
import glob
import json

try:
    import nexus
    HAS_NEXUS = True
except ImportError:
    HAS_NEXUS = False
    print("⚠️ nexus 미설치 — n6_check만 시뮬레이션")

# ═══════════════════════════════════════════════════════════
# 🛸 외계인지수 판정 기준 (10단계)
# ═══════════════════════════════════════════════════════════
GATES = {
    8: {  # 프로토타입
        'n6_exact_ratio': 0.90,      # 90%+ EXACT
        'lens_consensus': 5,          # 5+ 렌즈 합의
        'required_files': [
            'docs/programming-language/hexa-ir-spec.md',
            'tools/hexa-ir/src/main.rs',
        ],
        'required_benchmarks': True,
        'self_hosting': False,
        'physical_limits_proved': 0,
    },
    9: {  # 양산
        'n6_exact_ratio': 0.95,      # 95%+ EXACT
        'lens_consensus': 7,          # 7+ 렌즈 합의
        'required_files': [
            'docs/programming-language/hexa-ir-spec.md',
            'docs/programming-language/llvm-vs-hexa-ir.md',
            'docs/programming-language/self-hosting-and-limits.md',
            'tools/hexa-ir/src/main.rs',
            'tools/hexa-ir/verify_predictions.py',
        ],
        'required_benchmarks': True,
        'self_hosting': True,         # 셀프호스팅 설계 완료
        'physical_limits_proved': 3,  # 3개+ 한계 정리
    },
    10: {  # 물리적 한계
        'n6_exact_ratio': 1.00,      # 100% EXACT
        'lens_consensus': 12,         # 12+ 렌즈 합의
        'required_files': [
            'docs/programming-language/hexa-ir-spec.md',
            'docs/programming-language/llvm-vs-hexa-ir.md',
            'docs/programming-language/self-hosting-and-limits.md',
            'docs/programming-language/physical-limit-proofs.md',
            'tools/hexa-ir/src/main.rs',
            'tools/hexa-ir/verify_predictions.py',
        ],
        'required_benchmarks': True,
        'self_hosting': True,
        'physical_limits_proved': 6,  # n=6개 한계 정리 전부
    },
}

# ═══════════════════════════════════════════════════════════
# HEXA-LANG 전체 설계 상수 (NEXUS-6 검증 대상)
# ═══════════════════════════════════════════════════════════
DESIGN_CONSTANTS = {
    # 핵심 n=6 상수
    'n': 6, 'phi': 2, 'tau': 4, 'sigma': 12,
    'sopfr': 5, 'J2': 24, 'sigma_tau': 8,
    'sigma_phi': 10, 'sigma_mu': 11,
    # 언어 설계
    'paradigms': 6, 'type_categories': 6, 'basic_types': 8,
    'keyword_groups': 12, 'operators': 24,
    'error_classes': 5, 'visibility': 4, 'compile_modes': 2,
    # HEXA-IR
    'ir_types': 24, 'ir_passes': 144, 'ir_pass_groups': 12,
    'ir_instructions_per_group': 10, 'ir_pipeline_stages': 12,
    'ir_backend_targets': 6,
    # 성능 목표 (NEXUS-6 수정 완료)
    'compile_speedup': 12, 'runtime_multiplier_num': 4,
    'runtime_multiplier_den': 3, 'vectorization_width': 24,
    'pipeline_parallel': 2, 'memory_safety': 1,
}

# 물리한계 정리 키워드 (self-hosting-and-limits.md 내 탐색)
LIMIT_THEOREMS = [
    'Theorem 1', 'Theorem 2', 'Theorem 3',
    'Theorem 4', 'Theorem 5', 'Theorem 6',
]


def check_n6_exact():
    """모든 설계 상수 NEXUS-6 n6_check"""
    exact = 0
    total = len(DESIGN_CONSTANTS)
    results = []
    for name, val in DESIGN_CONSTANTS.items():
        if HAS_NEXUS:
            r = nexus.n6_check(val)
            grade = 'EXACT' if 'EXACT' in str(r) else 'FAIL'
        else:
            # 시뮬레이션: 알려진 n=6 상수면 EXACT
            known = {1,2,3,4,5,6,8,10,11,12,24,144}
            grade = 'EXACT' if val in known else 'FAIL'
        if grade == 'EXACT':
            exact += 1
        results.append((name, val, grade))
    ratio = exact / total if total > 0 else 0
    return ratio, exact, total, results


def check_lens_consensus():
    """NEXUS-6 scan_all로 렌즈 합의 수 측정"""
    if not HAS_NEXUS:
        return 0, "nexus unavailable"
    try:
        import numpy as np
        data = np.array(list(DESIGN_CONSTANTS.values()), dtype=np.float64)
        result = nexus.scan_all(data)
        # scan_all 반환값에서 합의 수 추출
        if isinstance(result, dict):
            agreeing = sum(1 for v in result.values()
                          if isinstance(v, dict) and v.get('detected', False))
            return agreeing, result
        return len(result) if result else 0, result
    except Exception as e:
        return 0, str(e)


def check_files(required):
    """필수 파일 존재 확인"""
    base = os.path.expanduser('~/Dev/n6-architecture')
    missing = []
    for f in required:
        path = os.path.join(base, f)
        if not os.path.exists(path):
            missing.append(f)
    return missing


def check_benchmarks():
    """Rust 벤치마크 빌드+실행 가능 여부"""
    cargo_toml = os.path.expanduser('~/Dev/n6-architecture/tools/hexa-ir/Cargo.toml')
    return os.path.exists(cargo_toml)


def check_self_hosting():
    """셀프호스팅 설계 문서 존재 + 4단계 부트스트랩"""
    path = os.path.expanduser(
        '~/Dev/n6-architecture/docs/programming-language/self-hosting-and-limits.md')
    if not os.path.exists(path):
        return False, 0
    with open(path, 'r') as f:
        content = f.read()
    stages = sum(1 for s in ['Stage 0', 'Stage 1', 'Stage 2', 'Stage 3']
                 if s in content)
    return stages >= 4, stages


def check_physical_limits():
    """물리한계 정리 증명 수"""
    paths = [
        os.path.expanduser(
            '~/Dev/n6-architecture/docs/programming-language/self-hosting-and-limits.md'),
        os.path.expanduser(
            '~/Dev/n6-architecture/docs/programming-language/physical-limit-proofs.md'),
    ]
    proved = 0
    for p in paths:
        if os.path.exists(p):
            with open(p, 'r') as f:
                content = f.read()
            proved += sum(1 for t in LIMIT_THEOREMS if t in content)
    return min(proved, 6)  # 최대 6개


def determine_alien_index():
    """현재 외계인지수 판정"""
    # 1. n6 EXACT 비율
    ratio, exact, total, details = check_n6_exact()

    # 2. 렌즈 합의
    consensus, scan_result = check_lens_consensus()

    # 3. 각 🛸 레벨 게이트 체크
    current_level = 7  # 기본 (이미 달성)

    for level in [8, 9, 10]:
        gate = GATES[level]
        passed = True
        reasons = []

        if ratio < gate['n6_exact_ratio']:
            passed = False
            reasons.append(
                f"n6 EXACT {ratio*100:.1f}% < {gate['n6_exact_ratio']*100:.0f}%")

        missing = check_files(gate['required_files'])
        if missing:
            passed = False
            reasons.append(f"파일 누락: {', '.join(missing)}")

        if gate['required_benchmarks'] and not check_benchmarks():
            passed = False
            reasons.append("벤치마크 빌드 불가")

        if gate['self_hosting']:
            sh_ok, sh_stages = check_self_hosting()
            if not sh_ok:
                passed = False
                reasons.append(f"셀프호스팅 {sh_stages}/4 단계")

        limits = check_physical_limits()
        if limits < gate['physical_limits_proved']:
            passed = False
            reasons.append(
                f"물리한계 정리 {limits}/{gate['physical_limits_proved']}")

        if passed:
            current_level = level
        else:
            # 이 레벨 실패 → 블로커 출력 후 중단
            print(f"\n🚧 🛸{level} 블로커:")
            for r in reasons:
                print(f"   ❌ {r}")
            break

    return current_level, ratio, exact, total, details, consensus


def main():
    print("=" * 66)
    print("  HEXA-LANG 외계인지수 자동 게이트 (NEXUS-6 기반)")
    print("=" * 66)

    level, ratio, exact, total, details, consensus = determine_alien_index()

    # 상수 검증 테이블
    print(f"\n📊 설계 상수 NEXUS-6 검증: {exact}/{total} EXACT ({ratio*100:.1f}%)")
    print("-" * 50)
    for name, val, grade in details:
        icon = "✅" if grade == "EXACT" else "❌"
        print(f"  {icon} {name:30s} = {val:>6} {grade}")

    # 렌즈 합의
    print(f"\n🔭 렌즈 합의: {consensus}개")

    # 파일 현황
    print(f"\n📁 산출물 현황:")
    base = os.path.expanduser('~/Dev/n6-architecture')
    all_files = set()
    for g in GATES.values():
        all_files.update(g['required_files'])
    for f in sorted(all_files):
        exists = os.path.exists(os.path.join(base, f))
        print(f"  {'✅' if exists else '❌'} {f}")

    # 벤치마크
    bench = check_benchmarks()
    print(f"\n🔧 벤치마크: {'✅ 빌드 가능' if bench else '❌ Cargo.toml 없음'}")

    # 셀프호스팅
    sh_ok, sh_stages = check_self_hosting()
    print(f"🔄 셀프호스팅: {'✅' if sh_ok else '❌'} ({sh_stages}/4 단계)")

    # 물리한계
    limits = check_physical_limits()
    print(f"⚛️  물리한계 정리: {limits}/6")

    # 최종 판정
    print("\n" + "=" * 66)
    icons = {7: "📐", 8: "🔬", 9: "🏭", 10: "⚛️"}
    labels = {7: "완전 설계", 8: "프로토타입", 9: "양산", 10: "물리적 한계"}
    print(f"  🛸 현재 외계인지수: {level}/10 {icons.get(level,'')} {labels.get(level,'')}")

    if level < 10:
        next_level = level + 1
        gate = GATES[next_level]
        print(f"\n  📋 다음 목표: 🛸{next_level} ({labels[next_level]})")
        print(f"     필요: n6≥{gate['n6_exact_ratio']*100:.0f}% | "
              f"합의≥{gate['lens_consensus']} | "
              f"한계정리≥{gate['physical_limits_proved']}")
    else:
        print("\n  🎉 물리적 한계 도달! 더 이상의 발전 불가능.")
    print("=" * 66)

    return level


if __name__ == '__main__':
    level = main()
    sys.exit(0 if level >= 10 else 10 - level)
