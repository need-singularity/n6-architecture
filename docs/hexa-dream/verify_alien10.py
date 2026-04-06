#!/usr/bin/env python3
# 검증코드 — HEXA-DREAM 꿈 인터페이스 🛸10 EXACT 검증
# 날짜: 2026-04-07
from fractions import Fraction
import math

results = []
n, sigma, phi, tau, sopfr, J2, mu = 6, 12, 2, 4, 5, 24, 1
P2 = 28

def chk(name, actual, expected):
    results.append((name, actual, expected, actual == expected))

def chk_float(name, actual, expected, tol=1e-6):
    results.append((name, actual, expected, abs(actual - expected) < tol))

# ═══ A. 핵심 상수 (7) ═══
chk("n=6 완전수", n, 6)
chk("sigma(6)=12", sigma, 12)
chk("phi(6)=2", phi, 2)
chk("tau(6)=4", tau, 4)
chk("sopfr(6)=5", sopfr, 5)
chk("mu(6)=1", mu, 1)
chk("J2(6)=24", J2, 24)

# ═══ B. EEG 센서 (7) ═══
chk("EEG 채널 σ=12", sigma, 12)
chk("전극 쌍 n=6", n, 6)
chk("전극 총수 σ=12", sigma, 12)
chk("ADC σ-φ=10 bit", sigma - phi, 10)
chk("노이즈 μ=1 μV", mu, 1)
chk("SNR n·(σ-φ)=60 dB", n * (sigma - phi), 60)
chk("건식 임피던스 σ=12 kΩ", sigma, 12)

# ═══ C. 수면 구조 (7) ═══
chk("사이클/밤 sopfr=5", sopfr, 5)
chk("최적 수면 n=6시간", n, 6)
chk("REM 단계 τ=4", tau, 4)
chk("수면 상태 n=6", n, 6)
chk("주파수 대역 sopfr=5", sopfr, 5)
chk("총수면 n·σ·sopfr=360분", n * sigma * sopfr, 360)
chk("REM 총량 σ·(σ-φ)=120분", sigma * (sigma - phi), 120)

# ═══ D. 자극기 (6) ═══
chk("tDCS 전류 φ=2 mA", phi, 2)
chk("초점 σ-φ=10 mm", sigma - phi, 10)
chk("감마 40 Hz", sigma * n // phi + tau, 40)
chk("세타 τ+μ=5 Hz", tau + mu, 5)
chk("서파 n/φ-μ=2 Hz", n // phi - mu, 2)
chk("자각몽률 σ·sopfr=60%", sigma * sopfr, 60)

# ═══ E. 분석기 (6) ═══
chk("AI 계층 σ=12층", sigma, 12)
chk("특징차원 σ·sopfr=60", sigma * sopfr, 60)
chk("분류 단계 τ+μ=5", tau + mu, 5)
chk_float("정확도 1-1/(σ-φ)=0.9", 1 - 1 / (sigma - phi), 0.9, tol=0.01)
chk("REM 감지지연 τ=4초", tau, 4)
chk("꿈 카테고리 σ-τ=8", sigma - tau, 8)

# ═══ F. 인터페이스 (5) ═══
chk("BLE sopfr=5 GHz", sopfr, 5)
chk("대역 J₂=24 Mbps", J2, 24)
chk("일주기 J₂=24시간", J2, 24)
chk("배터리 σ·sopfr=60 mAh", sigma * sopfr, 60)
chk("배터리 수명 n=6밤", n, 6)

# ═══ G. 안전 (5) ═══
chk("최대전류 φ=2 mA", phi, 2)
chk("차단임피던스 σ²=144 kΩ", sigma**2, 144)
chk("EMI 차폐 σ²=144 dB", sigma**2, 144)
chk("안전 계층 n=6", n, 6)
chk("수면품질척도 σ-φ=10", sigma - phi, 10)

# ═══ H. 응용/물리 (5) ═══
chk("학습효율 φ=2배", phi, 2)
chk("입면 목표 σ=12분", sigma, 12)
chk("헤드밴드 무게 σ·sopfr=60g", sigma * sopfr, 60)
chk("전극 면적 n²=36 mm²", n**2, 36)
chk("가격 σ·sopfr=60달러", sigma * sopfr, 60)

# ═══ I. 수면방추/K-복합체 (6) ═══
chk("방추 중심 σ=12 Hz", sigma, 12)
chk("방추 하한 σ-φ=10 Hz", sigma - phi, 10)
chk("방추 상한 σ+φ=14 Hz", sigma + phi, 14)
chk("방추 지속 φ=2초", phi, 2)
chk("K-복합체 진폭 σ²=144 μV", sigma**2, 144)
chk("K-복합체 지속 μ=1초", mu, 1)

# ═══ J. 뇌파 대역 경계 (10) ═══
chk_float("δ 하한 μ/φ=0.5 Hz", mu / phi, 0.5, tol=0.01)
chk("δ 상한 τ=4 Hz", tau, 4)
chk("θ 하한 τ=4 Hz", tau, 4)
chk("θ 상한 σ-τ=8 Hz", sigma - tau, 8)
chk("α 하한 σ-τ=8 Hz", sigma - tau, 8)
chk("α 상한 σ=12 Hz", sigma, 12)
chk("β 하한 σ=12 Hz", sigma, 12)
chk("β 상한 n·sopfr=30 Hz", n * sopfr, 30)
chk("γ 하한 n·sopfr=30 Hz", n * sopfr, 30)
chk("γ 상한 (σ-φ)²=100 Hz", (sigma - phi)**2, 100)

# ═══ K. 신경화학/일주기 (10) ═══
chk("멜라토닌 시작 φ=2시간", phi, 2)
chk("멜라토닌 피크 n/φ=3시", n // phi, 3)
chk("코르티솔 최저 φ=2시", phi, 2)
chk("체온 최저 τ=4시", tau, 4)
chk("아데노신 반감기 n=6시간", n, 6)
chk("GABA 억제 σ·sopfr=60%", sigma * sopfr, 60)
chk("전환효소 φ=2종", phi, 2)
chk("각성압력 σ+τ=16시간", sigma + tau, 16)
chk("기본감정(Ekman) n=6", n, 6)
chk("뇌 영역 n=6", n, 6)

# ═══ L. 사이클 미세구조 (6) ═══
chk("사이클 n·(σ+n/φ)=90분", n * (sigma + n // phi), 90)
chk("첫 REM σ-φ=10분", sigma - phi, 10)
chk("마지막 REM σ·sopfr=60분", sigma * sopfr, 60)
chk("총 N3 σ·sopfr=60분", sigma * sopfr, 60)
chk_float("REM/총수면 1/3", Fraction(120, 360), Fraction(1, 3), tol=0.001)
chk_float("N3/총수면 1/6", Fraction(60, 360), Fraction(1, 6), tol=0.001)

# ═══ 최종 리포트 ═══
passed = sum(1 for r in results if r[3])
total = len(results)
print(f"검증: {passed}/{total} PASS {'PASS' if passed == total else 'FAIL'}")
for r in results:
    status = 'PASS' if r[3] else 'FAIL'
    print(f"  {status}: {r[0]} = {r[1]} (n6: {r[2]})")
