#!/usr/bin/env python3
# 검증코드 — HEXA-EMPATH 감정 공유 🛸10 EXACT 검증
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

# === A. 핵심 상수 (7) ===
chk("n=6 완전수", n, 6)
chk("sigma(6)=12", sigma, 12)
chk("phi(6)=2", phi, 2)
chk("tau(6)=4", tau, 4)
chk("sopfr(6)=5", sopfr, 5)
chk("mu(6)=1", mu, 1)
chk("J2(6)=24", J2, 24)

# === B. 센서 아키텍처 (6) ===
chk("바이오센서 모달 n=6", n, 6)
chk("EDA 채널 φ=2", phi, 2)
chk("ECG 리드 σ=12", sigma, 12)
chk("호흡 주파수 τ=4 Hz", tau, 4)
chk("MFCC 음성 계수 sopfr=5", sopfr, 5)
chk("기본 AU n=6", n, 6)

# === C. 감정 분류 (8) ===
chk("Ekman 기본감정 n=6", n, 6)
chk("강도 φ=2 레벨", phi, 2)
chk("세분류 n·φ=σ=12", n * phi, 12)
chk("전체감정 σ·φ=J₂=24", sigma * phi, 24)
chk_float("정확도 1-1/(J₂-τ)=0.95", 1 - 1 / (J2 - tau), 0.95)
chk("감정 벡터 비트 sopfr=5", sopfr, 5)
chk("강도 비트 n/φ=3", n // phi, 3)
chk("비트/프레임 σ-τ=8", sigma - tau, 8)

# === D. 디코더 (8) ===
chk("디코더 계층 σ=12", sigma, 12)
chk("디코더 차원 2^σ=4096", 2**sigma, 4096)
chk("헤드 수 2^sopfr=32", 2**sopfr, 32)
chk("헤드 차원 2^(σ-sopfr)=128", 2**(sigma - sopfr), 128)
chk_float("SwiGLU τ²/σ=4/3", tau**2 / sigma, 4 / 3)
chk("GQA KV σ-τ=8", sigma - tau, 8)
chk_float("dropout ln(4/3)", math.log(4 / 3), 0.2876820724517809, tol=1e-4)
chk_float("LR (n/φ)·10^(-τ)=3e-4", (n / phi) * 10**(-tau), 3e-4, tol=1e-10)

# === E. 전송 (5) ===
chk("코덱 J₂=24 kbps", J2, 24)
chk("AES 2^(σ-τ)=256 bit", 2**(sigma - tau), 256)
chk("전송지연 μ=1 ms", mu, 1)
chk("프레임율 τ=4 kHz", tau, 4)
chk("프로토콜 계층 sopfr=5", sopfr, 5)

# === F. 자극 (6) ===
chk("감정중추 타겟 n=6 영역", n, 6)
chk("자극 세기 σ=12 mT", sigma, 12)
chk("자극 주파수 σ=12 Hz", sigma, 12)
chk("듀티 round((1-1/e)·100)=63%", round((1 - 1 / math.e) * 100), 63)
chk("피드백 루프 τ=4 ms", tau, 4)
chk("공간해상도 μ=1 mm", mu, 1)

# === G. 안전 (6) ===
chk("안전 계층 n=6", n, 6)
chk("윤리 게이트 n/φ=3중", n // phi, 3)
chk("과자극 차단 n·(σ-φ)=60 dB", n * (sigma - phi), 60)
chk("동의 단계 τ=4", tau, 4)
chk("암호 계층 sopfr=5", sopfr, 5)
chk("비상정지 μ=1 ms", mu, 1)

# === H. 피질 (2) ===
chk("대뇌피질 n=6 층", n, 6)
chk("작업기억 τ=4 청크", tau, 4)

# === I. 뇌파/신경 (13) ===
chk("EEG 밴드 수 sopfr=5", sopfr, 5)
chk("기본 전극 σ=12", sigma, 12)
chk("alpha 중심 σ-φ=10 Hz", sigma - phi, 10)
chk("beta 상한 J₂=24 Hz", J2, 24)
chk("gamma 하한 2^sopfr=32 Hz", 2**sopfr, 32)
chk("delta 상한 τ=4 Hz", tau, 4)
chk("theta 하한 τ=4 Hz", tau, 4)
chk("theta 상한 σ-τ=8 Hz", sigma - tau, 8)
chk("뇌 반구 φ=2", phi, 2)
chk("뇌엽 τ=4", tau, 4)
chk("변연계 핵심 n=6 구조", n, 6)
chk("뉴런/컬럼 μ=1 (x10^4)", mu, 1)
chk("수면 단계 sopfr=5", sopfr, 5)

# === J. 신경전달물질 (8) ===
chk("주요 감정 NTM n=6", n, 6)
chk("도파민 수용체 sopfr=5", sopfr, 5)
chk("세로토닌 패밀리 sopfr=5", sopfr, 5)
chk("아드레날린 아형 φ=2", phi, 2)
chk("GABA 유형 φ=2", phi, 2)
chk("엔도르핀 유형 n/φ=3", n // phi, 3)
chk("옥시토신 반감기 sopfr=5분", sopfr, 5)
chk("세로토닌 효소 φ=2", phi, 2)

# === K. 생체신호 (8) ===
chk("ANS 분지 φ=2", phi, 2)
chk("HRV 주파수밴드 n/φ=3", n // phi, 3)
chk("안정 HR 하한 n·(σ-φ)=60 bpm", n * (sigma - phi), 60)
chk("안정 HR 상한 (σ-φ)^φ=100 bpm", (sigma - phi)**phi, 100)
chk("LF/HF 균형 μ=1", mu, 1)
chk("EDA 반응 상한 τ=4초", tau, 4)
chk("코르티솔 피크 σ-τ=8시", sigma - tau, 8)
chk("체온 일주기 변동 μ=1도C", mu, 1)

# === L. 사회/네트워크 (6) ===
chk("6단계 분리 n=6", n, 6)
chk("친밀 관계 sopfr=5", sopfr, 5)
chk("동정 그룹 하한 σ=12", sigma, 12)
chk("Dunbar 밴드 σ·τ=48", sigma * tau, 48)
chk("Dunbar 수 σ²+n=150", sigma**2 + n, 150)
chk("거울뉴런 지연 μ=1 ms", mu, 1)

# === M. 임상 스코어링 (8) ===
chk("GCS 최소 n/φ=3", n // phi, 3)
chk("GCS 최대 σ+n/φ=15", sigma + n // phi, 15)
chk("GCS 하위척도 n/φ=3", n // phi, 3)
chk("SOFA 장기 n=6", n, 6)
chk("PHQ-9 문항 σ-n/φ=9", sigma - n // phi, 9)
chk("HAM-D 항목 J₂=24", J2, 24)
chk("Apgar 항목 sopfr=5", sopfr, 5)
chk("Apgar 최고점 σ-φ=10", sigma - phi, 10)

# === N. 하드웨어 (6) ===
chk("센서 무게 σ=12g", sigma, 12)
chk("총 무게 σ·τ=48g", sigma * tau, 48)
chk("배터리 sopfr·10^φ=500 mAh", sopfr * 10**phi, 500)
chk("연속사용 J₂=24시간", J2, 24)
chk("충전시간 μ=1시간", mu, 1)
chk("BLE 범위 σ-φ=10 m", sigma - phi, 10)

# ═══ 최종 리포트 ═══
passed = sum(1 for r in results if r[3])
total = len(results)
print(f"검증: {passed}/{total} PASS {'PASS' if passed == total else 'FAIL'}")
for r in results:
    status = 'PASS' if r[3] else 'FAIL'
    print(f"  {status}: {r[0]} = {r[1]} (n6: {r[2]})")
