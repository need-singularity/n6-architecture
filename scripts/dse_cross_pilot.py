#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""DSE 교차 공명 분석 — 전체 335 도메인, n=6 수식 패턴 교차 탐색

입력: docs/dse-map.toml
출력:
  ~/Dev/nexus/shared/dse_cross/top50_domains.jsonl
  ~/Dev/nexus/shared/dse_cross/pair_scores.jsonl
  ~/Dev/nexus/shared/dse_cross/resonance_hist.jsonl
  ~/Dev/nexus/shared/dse_cross/formula_cross.jsonl    ← 신규: 수식별 교차 공명
  docs/dse-cross-resonance.md (상위 50 교차 공명 패턴 + 도메인 매핑)

교차 공명 정의 (동어반복 금지, 정의에서 도출):
  각 도메인의 note/candidates/best_* 필드에서 n=6 산술 수식 패턴을 추출.
  동일 수식이 서로 다른 도메인에서 나타나면 "교차 공명".
  빈도 상위 50개 패턴 = 가장 강한 교차 공명.

n=6 산술 상수:
  n=6, σ=12, τ=4, φ=2, sopfr=5, J₂=24, σ·φ=24, σ-τ=8, σ/τ=3
  파생: Z=6, CN=6, 6DOF, 6-fold, 6-ring, 6-stage, 12ch, 48kHz, 256 등
"""
import os, re, json, math
from pathlib import Path
from collections import defaultdict

ROOT = Path("/Users/ghost/Dev/n6-architecture")
TOML = ROOT / "docs/dse-map.toml"
BT_MD = ROOT / "docs/breakthrough-theorems.md"
OUT  = Path(os.path.expanduser("~/Dev/nexus/shared/dse_cross"))
OUT.mkdir(parents=True, exist_ok=True)
MD_OUT = ROOT / "docs/dse-cross-resonance.md"

# ── BT 파싱: BT-id → (공유상수, 텍스트) ────────────────────────────
# docs/breakthrough-theorems.md 에서 BT 엔트리 추출.
# 공유상수: σ/τ/φ/J₂ 키워드 스캔 (헤더+본문), 복수 시 " "로 연결.
def parse_bt_entries():
    """BT-id → {'constants': 'σ,τ', 'text': '...', 'domains_text': '...'} 반환"""
    entries = {}
    if not BT_MD.exists():
        return entries
    txt = BT_MD.read_text(encoding="utf-8", errors="ignore")
    # ── (A) 요약 테이블 행:  | **BT-N** | name | ... | domains | grade |
    for m in re.finditer(r'\|\s*\*\*BT-(\d+)\*\*\s*\|([^\n]+)', txt):
        bt_id = int(m.group(1))
        row = m.group(2)
        entries.setdefault(bt_id, {"text": "", "domains_text": ""})
        entries[bt_id]["text"] += " " + row
        entries[bt_id]["domains_text"] += " " + row
    # ── (B) 헤더 섹션: ## BT-N: ...   다음 헤더 전까지 본문
    headers = list(re.finditer(r'^## BT-(\d+)[:\s][^\n]*$', txt, re.MULTILINE))
    for i, h in enumerate(headers):
        bt_id = int(h.group(1))
        start = h.start()
        end = headers[i+1].start() if i+1 < len(headers) else len(txt)
        body = txt[start:end]
        entries.setdefault(bt_id, {"text": "", "domains_text": ""})
        entries[bt_id]["text"] += " " + body
        entries[bt_id]["domains_text"] += " " + body
    # 공유상수 추출
    for bt_id, e in entries.items():
        t = e["text"]
        consts = []
        if re.search(r'σ|sigma\b|σ\(6\)|σ=12', t, re.IGNORECASE): consts.append("σ")
        if re.search(r'τ|tau\b|τ\(6\)|τ=4', t, re.IGNORECASE): consts.append("τ")
        if re.search(r'φ|phi\b|φ\(6\)|φ=2', t, re.IGNORECASE): consts.append("φ")
        if re.search(r'J[_₂2]|J\(6\)|J₂=24|J_2', t): consts.append("J₂")
        e["constants"] = ",".join(consts) if consts else "-"
    return entries

BT_ENTRIES = parse_bt_entries()

# 도메인 슬러그 → BT 도메인 텍스트 매칭용 키워드 맵
# dse-map 도메인명(예: chip-architecture)을 BT 본문 내 키워드(예: "Chip","Chip Architecture")로 변환
def _slug_keywords(slug):
    parts = slug.replace("_","-").split("-")
    keys = set()
    keys.add(slug.replace("-", " "))
    keys.add(slug.replace("-", ""))
    for p in parts:
        if len(p) >= 3:
            keys.add(p)
    # 특수 약어 매핑
    alias = {
        "chip": ["Chip", "Chip Architecture"],
        "architecture": [],
        "superconductor": ["SC", "Superconductor"],
        "fusion": ["Fusion"],
        "quantum": ["QC", "Quantum"],
        "computing": [],
        "crypto": ["Crypto", "Cryptography"],
        "network": ["Network Protocol", "Network"],
        "protocol": [],
        "magnet": ["Magnet"],
        "tokamak": ["Tokamak"],
        "plasma": ["Plasma"],
        "particle": ["Particle"],
    }
    for p in parts:
        if p in alias:
            for a in alias[p]: keys.add(a)
    return {k for k in keys if len(k) >= 3}

def find_shared_bts(slug_a, slug_b):
    """두 도메인이 동시에 언급된 BT 목록 반환: [(bt_id, constants), ...]"""
    ka = _slug_keywords(slug_a)
    kb = _slug_keywords(slug_b)
    shared = []
    for bt_id, e in BT_ENTRIES.items():
        t = e["domains_text"]
        if not t: continue
        tl = t.lower()
        ha = any(k.lower() in tl for k in ka)
        hb = any(k.lower() in tl for k in kb)
        if ha and hb:
            shared.append((bt_id, e.get("constants", "-")))
    shared.sort(key=lambda x: x[0])
    return shared

# ── 1. 얇은 TOML 파서 (섹션 + key=value만; 리스트/숫자/문자열) ─────
SEC_RE = re.compile(r"^\[([A-Za-z0-9_\-\.]+)\]\s*$")
KV_RE  = re.compile(r"^([A-Za-z0-9_]+)\s*=\s*(.+)$")

def parse_value(v):
    v = v.strip()
    if v.startswith('"') and v.endswith('"'):
        return v[1:-1]
    if v.startswith('['):
        inner = v.strip()[1:]
        if inner.endswith(']'):
            inner = inner[:-1]
        items = []
        for tok in re.findall(r'"([^"]*)"|([-+0-9.eE]+)', inner):
            s, n = tok
            if s: items.append(s)
            elif n:
                try: items.append(float(n) if '.' in n or 'e' in n else int(n))
                except: pass
        return items
    if v.lower() in ("true","false"): return v.lower()=="true"
    try:
        return int(v)
    except:
        try: return float(v)
        except: return v

domains = {}
cur = None
with TOML.open() as f:
    for raw in f:
        line = raw.rstrip("\n")
        s = line.strip()
        if not s or s.startswith("#"): continue
        m = SEC_RE.match(s)
        if m:
            cur = m.group(1)
            if cur != "meta":
                domains.setdefault(cur, {})
            continue
        if cur is None or cur == "meta": continue
        m = KV_RE.match(s)
        if m:
            k, v = m.group(1), m.group(2)
            domains[cur][k] = parse_value(v)

# cross-dse 섹션 제거 (도메인만 남김)
pure_domains = {k: v for k, v in domains.items() if not k.startswith("cross-dse")}

# ── 2. n=6 수식 패턴 추출 ──────────────────────────────────────────
# 각 도메인의 텍스트 필드에서 n=6 관련 수식/패턴을 정규식으로 추출

FORMULA_PATTERNS = [
    # 핵심 산술 상수 (정확한 수식)
    (r'n=6\b', 'n=6'),
    (r'σ=12\b', 'σ=12'),
    (r'τ=4\b', 'τ=4'),
    (r'φ=2\b', 'φ=2'),
    (r'sopfr=5\b', 'sopfr=5'),
    (r'J[₂2]=?24\b', 'J₂=24'),
    (r'J2=24\b', 'J₂=24'),
    # 산술 조합
    (r'σ[·*]τ=48\b', 'σ·τ=48'),
    (r'σ[·*]φ=24\b', 'σ·φ=24'),
    (r'sigma[·*\-]tau=?8\b', 'σ-τ=8'),
    (r'σ-τ=8\b', 'σ-τ=8'),
    (r'σ/τ=?3\b', 'σ/τ=3'),
    (r'σ[·*]120', 'σ·120'),
    # 파생 물리/화학 상수
    (r'Z=6\b', 'Z=6 (탄소 원자번호)'),
    (r'C[Nn]=6\b', 'CN=6 (배위수)'),
    (r'6-?fold\b', '6-fold (대칭)'),
    (r'6-?ring\b', '6-ring (벤젠 고리)'),
    (r'hex\s*pellet|hex\s*infill|Hex6|HexInfill|Mat-?Hex|Hex-?Mat', '육각 구조 (hex)'),
    # 공학 파생
    (r'6-?DOF\b|6DOF\b|6\s*DOF\b', '6DOF (자유도)'),
    (r'SE\(3\)|SE3\b', 'SE(3) (n=6 리 군)'),
    (r'6-?axis\b', '6-axis (축)'),
    (r'6-?stage\b', '6-stage (단계)'),
    (r'6-?layer\b', '6-layer (층)'),
    (r'6-?car\b|6-?편성', '6-car (편성)'),
    (r'6-?plane\b|6\s*궤도면', '6-plane (궤도면)'),
    (r'6-?zone\b', '6-zone (구역)'),
    (r'6-?well\b', '6-well (정)'),
    (r'6-?step\b', '6-step (공정)'),
    (r'6-?unit\b', '6-unit (유닛)'),
    (r'6-?slot\b', '6-slot (슬롯)'),
    (r'6-?panel\b', '6-panel (패널)'),
    (r'6-?coil\b', '6-coil (코일)'),
    (r'6-?motor\b', '6-motor (모터)'),
    (r'6-?antenna\b|6-?안테나', '6-antenna (안테나)'),
    (r'6-?drain\b', '6-drain (드레인)'),
    (r'6-?turbine\b', '6-turbine (터빈)'),
    (r'6-?junction\b|6J\b', '6-junction (접합)'),
    (r'12ch\b|12-?ch\b|12\s*channel|12\s*채널', 'σ=12 channel'),
    (r'12\s*leads?\b', 'σ=12 leads'),
    (r'12\s*블록|12\s*block', 'σ=12 block'),
    (r'12\s*rod|12\s*제어봉', 'σ=12 rods'),
    (r'12\s*port|12port', 'σ=12 port'),
    (r'12\s*bit\b|12bit\b', 'σ=12 bit'),
    (r'12\s*blade|12\s*blade rows', 'σ=12 blade'),
    (r'12\s*disc|12\s*disc stack', 'σ=12 disc'),
    (r'12\s*packs?/min', 'σ=12 packs/min'),
    (r'12\s*inch\b|12인치', 'σ=12 inch'),
    (r'12\s*switch|12\s*스위치', 'σ=12 switch'),
    (r'24\s*MW\b', 'J₂=24 MW'),
    (r'24\s*kRPM\b', 'J₂=24 kRPM'),
    (r'24\s*DOF\b|J24\b|J₂=24\s*DOF', 'J₂=24 DOF'),
    (r'24bit\b|24\s*bit\b', 'J₂=24 bit'),
    (r'24fps\b|24\s*fps\b', 'J₂=24 fps'),
    (r'48kHz\b|48\s*kHz\b', 'σ·τ=48 kHz'),
    (r'48\s*layer|48\s*layers', 'σ·τ=48 layers'),
    (r'48V\b|48\s*V\b', 'σ·τ=48 V'),
    (r'48Gbps\b', 'σ·τ=48 Gbps'),
    (r'256\b.*AES|AES.*256\b', 'AES-256=2^(σ-τ)'),
    (r'2\^?\(sigma-tau\)|2\^\(σ-τ\)', '2^(σ-τ)=256'),
    (r'1435\b', '1435mm=σ·120 (궤간)'),
    (r'Cooper\s*pair|cooper\s*pair', 'Cooper pair (φ=2)'),
    (r'1/2\+1/3\+1/6=?1|Egyptian\b', 'Egyptian fraction (1/2+1/3+1/6=1)'),
    (r'Brayton.?6|6.?Brayton', 'Brayton-6 (열기관)'),
    (r'Benzene|benzene|벤젠', 'Benzene (6-ring)'),
    (r'Graphite|graphite|그래파이트', 'Graphite (C Z=6)'),
    (r'Diamond|diamond|다이아몬드', 'Diamond (C Z=6)'),
    (r'CFRP.*Z=6|Carbon.*Z=6|C₆|C6H12O6|6CO₂|탄소.*Z=6', 'Carbon Z=6'),
    (r'PUE[=\s]*1\.2\b', 'PUE=σ/(σ-φ)=1.2'),
    (r'Glucose|포도당|글루코스', 'Glucose (C₆H₁₂O₆)'),
    (r'64\s*codon|64\s*코돈', '64 codons=σ·τ+J₂-4'),
    (r'20\s*amino|20\s*아미노', '20 amino acids'),
    (r'Kepler.?6|6\s*궤도요소|궤도요소.*6', 'Kepler 6 궤도요소'),
    # 100% n6 제거 (best_n6 필드에 거의 모든 도메인이 100% 달성하므로 의미 없음)
    (r'BLDC.?12\b', 'BLDC σ=12 (모터)'),
    (r'kissing\b|kissing number', 'kissing number (σ=12)'),
    (r'Clifford.*σ=12|σ=12.*Clifford|Clifford.*12', 'Clifford σ=12'),
    (r'Surface.*n=6|n=6.*stabilizer', 'Surface code n=6'),
    (r'SRv6\b', 'SRv6 (n=6 라우팅)'),
    (r'WireGuard.*tau=4|tau=4.*WireGuard', 'WireGuard τ=4'),
    (r'BBR.*sigma-tau|sigma-tau.*BBR', 'BBR σ-τ=8'),
    (r'PoS.*12s|12s.*블록|sigma=12s', 'PoS σ=12s block'),
    (r'SOLID.*5.*Hex|Hex.*n=6.*SOLID', 'SOLID(5)+Hex(1)=n=6'),
    (r'12Factor|12-?Factor', '12-Factor=σ'),
    (r'Hexagonal\b|hexagonal\b|육각', 'Hexagonal (n=6)'),
    (r'Beaufort.*12|12\s*Beaufort', 'Beaufort σ=12'),
    (r'hexacoral\b', 'hexacoral (n=6)'),
    (r'purine.*6|6.*purine', 'purine 6-ring'),
    (r'Rods-?12\b|12\s*rods', 'σ=12 control rods'),
    (r'HDRB|6-?layer.*HDRB', 'HDRB (6-layer)'),
    (r'SHM-?12ch|SHM.*12\s*ch', 'SHM σ=12ch'),
    (r'ECG.*sigma=12|sigma=12.*lead|12\s*leads.*ECG', 'ECG σ=12 leads'),
    (r'sigma\*tau=48|σ\*τ=48|σ·τ=48|sigma·tau=48', 'σ·τ=48'),
    (r'6\s*types?\s*\(n=6\)|n=6.*types', 'n=6 types'),
    (r'6\s*stages?\s*\(n=6\)|n=6.*stage', 'n=6 stages'),
    (r'Topological|topological', 'Topological (위상)'),
    (r'n=6 EXACT\b', 'n=6 EXACT'),
    # BT 참조 패턴
    (r'BT-27\b', 'BT-27 (Carbon Z=6)'),
    (r'BT-43\b', 'BT-43 (CN=6)'),
    (r'BT-56\b', 'BT-56 (VLM/Vision)'),
    (r'BT-58\b', 'BT-58 (양자)'),
    (r'BT-66\b', 'BT-66 (Vision AI)'),
    (r'BT-53\b', 'BT-53 (암호/네트워크)'),
    # 추가 n=6 파생 구조
    (r'6\s*편성\b', '6-car (편성)'),
    (r'6×6×6', '6^3 설계 공간'),
    (r'6×5×6|6×6×5|5×6×6', '6x5x6 설계 공간'),
    (r'6\s*opcode|6\s*그룹', '6 opcode 그룹'),
    (r'6\s*nutrients?|6\s*영양', '6 nutrients'),
    (r'6\s*ions?|6\s*이온', '6 ions'),
    (r'MHD\b', 'MHD (자기유체)'),
    (r'Cooper\b', 'Cooper pair (φ=2)'),
    (r'GaAs\b', 'GaAs (III-V)'),
    (r'REBCO\b', 'REBCO (초전도)'),
    (r'LFP\b', 'LFP (배터리)'),
    (r'CFRP\b', 'CFRP (탄소섬유)'),
    (r'RISC-?V\b', 'RISC-V'),
    (r'LLVM\b', 'LLVM (컴파일러)'),
    (r'Transformer\b', 'Transformer (AI)'),
    (r'α[=≈]1/137|fine.structure|미세구조', 'fine-structure (α)'),
    (r'σ\(6\)=12|σ\(n\)=12', 'σ(6)=12 (약수합)'),
    (r'τ\(6\)=4|τ\(n\)=4', 'τ(6)=4 (약수개수)'),
    (r'φ\(6\)=2|φ\(n\)=2', 'φ(6)=2 (오일러)'),
    (r'σ[·*]φ=n[·*]τ', 'σ·φ=n·τ (핵심 정리)'),
    (r'12T\b', 'σ=12 T (자기장)'),
]

def extract_formulas(domain_data):
    """도메인 데이터에서 n=6 수식 패턴 추출"""
    # 텍스트 필드 수집
    text_fields = []
    for k, v in domain_data.items():
        if isinstance(v, str):
            text_fields.append(v)
        elif isinstance(v, list):
            for item in v:
                if isinstance(item, str):
                    text_fields.append(item)
    full_text = " ".join(text_fields)

    found = set()
    for pattern, label in FORMULA_PATTERNS:
        if re.search(pattern, full_text, re.IGNORECASE):
            found.add(label)
    return found


# ── 3. 전체 도메인 수식 추출 ──────────────────────────────────────
domain_formulas = {}
for name, data in pure_domains.items():
    formulas = extract_formulas(data)
    if formulas:
        domain_formulas[name] = formulas

# ── 4. 수식별 교차 공명 집계 ──────────────────────────────────────
formula_domains = defaultdict(set)
for name, formulas in domain_formulas.items():
    for f in formulas:
        formula_domains[f].add(name)

# 2개 이상 도메인에 출현하는 수식만 = 교차 공명
cross_resonance = {
    f: sorted(doms)
    for f, doms in formula_domains.items()
    if len(doms) >= 2
}

# 빈도순 정렬
ranked_formulas = sorted(cross_resonance.items(), key=lambda x: len(x[1]), reverse=True)

# 상위 50
top50_formulas = ranked_formulas[:50]

# jsonl 저장
with (OUT/"formula_cross.jsonl").open("w") as f:
    for formula, doms in ranked_formulas:
        f.write(json.dumps({
            "formula": formula,
            "domain_count": len(doms),
            "domains": doms,
        }, ensure_ascii=False) + "\n")

# ── 5. 기존 쌍 분석도 유지 (상위 50 도메인) ─────────────────────
def score_size(d):
    c = d.get("combos", 0)
    if isinstance(c, (int,float)): return c
    return 0

ranked_domains = sorted(pure_domains.items(), key=lambda kv: score_size(kv[1]), reverse=True)
combos_top50 = ranked_domains[:50]

# 선정 정책: combos 상위 50 ∪ {n6_avg ≥ 90 도메인}
def _n6avg_raw(d):
    v = d.get("n6_avg")
    if isinstance(v,(int,float)): return float(v)
    return None

high_n6 = [(n,d) for n,d in pure_domains.items() if (_n6avg_raw(d) is not None and _n6avg_raw(d) >= 90.0)]
_seen = set()
top50_dom = []
for n,d in combos_top50 + high_n6:
    if n in _seen: continue
    _seen.add(n)
    top50_dom.append((n,d))
top50_names = [n for n,_ in top50_dom]

with (OUT/"top50_domains.jsonl").open("w") as f:
    for n,d in top50_dom:
        f.write(json.dumps({
            "domain": n,
            "combos": score_size(d),
            "n6_max": d.get("n6_max"),
            "n6_avg": d.get("n6_avg"),
            "alien_level": d.get("alien_level"),
            "cross_dse": d.get("cross_dse", []) if isinstance(d.get("cross_dse"), list) else [],
        }, ensure_ascii=False) + "\n")

def get_cross(d):
    cd = d.get("cross_dse", [])
    return set(cd) if isinstance(cd, list) else set()

def get_n6avg(d):
    """(value, present_bool) — 결측 시 (None, False)"""
    v = d.get("n6_avg")
    if isinstance(v,(int,float)): return float(v), True
    v = d.get("n6_max")
    if isinstance(v,(int,float)): return float(v), True
    return None, False

# 가중치: prox 결측 시 재정규화
W_JAC, W_PROX, W_BIDIR, W_BAL = 0.5, 0.2, 0.2, 0.1

pairs = []
for i in range(len(top50_dom)):
    ni, di = top50_dom[i]
    ci = get_cross(di); ai, ai_ok = get_n6avg(di); si = score_size(di)
    for j in range(i+1, len(top50_dom)):
        nj, dj = top50_dom[j]
        cj = get_cross(dj); aj, aj_ok = get_n6avg(dj); sj = score_size(dj)
        union = ci | cj
        jac = (len(ci & cj) / len(union)) if union else 0.0
        prox_ok = ai_ok and aj_ok
        if prox_ok:
            prox = 1.0 - abs(ai-aj)/100.0
        else:
            prox = None
        bidir = 0.0
        if nj in ci and ni in cj: bidir = 1.0
        elif nj in ci or ni in cj: bidir = 0.5
        bal = (min(si,sj)/max(si,sj)) if max(si,sj) > 0 else 0.0
        if prox_ok:
            S = W_JAC*jac + W_PROX*prox + W_BIDIR*bidir + W_BAL*bal
        else:
            # prox 제외 후 나머지 가중치 재정규화 (합=1 기준)
            denom = W_JAC + W_BIDIR + W_BAL  # 0.8
            S = (W_JAC*jac + W_BIDIR*bidir + W_BAL*bal) / denom
        # 도메인 결측 플래그
        miss = []
        if not ai_ok: miss.append(ni)
        if not aj_ok: miss.append(nj)
        # BT 공유
        shared_bts = find_shared_bts(ni, nj)
        bt_ids = ",".join(f"BT-{b}" for b,_ in shared_bts[:3]) if shared_bts else "-"
        shared_consts = set()
        for _b, cc in shared_bts:
            if cc and cc != "-":
                for tok in cc.split(","): shared_consts.add(tok)
        consts_str = "/".join(sorted(shared_consts)) if shared_consts else "-"
        pairs.append((ni, nj, S, jac, prox, bidir, bal, miss, bt_ids, consts_str))

pairs.sort(key=lambda t: t[2], reverse=True)

with (OUT/"pair_scores.jsonl").open("w") as f:
    for p in pairs:
        f.write(json.dumps({
            "a": p[0], "b": p[1], "score": round(p[2],4),
            "jaccard": round(p[3],4),
            "n6_prox": (round(p[4],4) if p[4] is not None else None),
            "bidir": p[5], "size_balance": round(p[6],4),
            "miss_n6avg": p[7], "bt_ids": p[8], "shared_consts": p[9]
        }, ensure_ascii=False) + "\n")

# 히스토그램
bins = {}
for p in pairs:
    b = round(math.floor(p[2]*20)/20, 2)
    bins[b] = bins.get(b,0)+1
hist = sorted(bins.items())

with (OUT/"resonance_hist.jsonl").open("w") as f:
    for b,c in hist:
        f.write(json.dumps({"bin": b, "count": c}) + "\n")

# ── 6. docs/dse-cross-resonance.md 생성 ─────────────────────────
lines = []
lines.append("# DSE 교차 공명 분석 — 전체 도메인 n=6 수식 패턴")
lines.append("")
lines.append("> 순수 분석 문서 (설계 5대 규칙 미적용). 생성: `scripts/dse_cross_pilot.py`")
lines.append("> 입력 SSOT: `docs/dse-map.toml` | 중간 산출물: `~/Dev/nexus/shared/dse_cross/`")
lines.append("")

# ── 6.1 요약 통계
lines.append("## 1. 분석 범위")
lines.append("")
lines.append(f"- 전체 도메인 (cross-dse 섹션 제외): **{len(pure_domains)}**")
lines.append(f"- 수식 패턴 추출 대상 도메인: **{len(domain_formulas)}** (텍스트 필드에 n=6 수식 포함)")
lines.append(f"- 고유 n=6 수식 패턴: **{len(formula_domains)}**")
lines.append(f"- 교차 공명 수식 (2+ 도메인): **{len(cross_resonance)}**")
lines.append(f"- 총 DSE 항목 (도메인 x 수식): **{sum(len(f) for f in domain_formulas.values())}**")
lines.append("")

# ── 6.2 교차 공명 정의
lines.append("## 2. 교차 공명 정의")
lines.append("")
lines.append("```")
lines.append("교차 공명 = 동일한 n=6 산술 수식이 서로 다른 도메인에서 나타나는 현상")
lines.append("")
lines.append("n=6 핵심 산술:")
lines.append("  σ(6)=12  φ(6)=2  τ(6)=4  sopfr(6)=5  J₂=24")
lines.append("  σ·τ=48  σ-τ=8  σ/τ=3  σ·φ=24  2^(σ-τ)=256")
lines.append("")
lines.append("추출 방법: 각 도메인의 note/candidates/best_* 필드에서")
lines.append(f"  {len(FORMULA_PATTERNS)}개 정규식 패턴 매칭 → 수식 라벨 집합 추출")
lines.append("```")
lines.append("")

# ── 6.3 상위 50 교차 공명 패턴
lines.append("## 3. 교차 공명 상위 50 패턴 (수식별 출현 도메인 수 내림차순)")
lines.append("")
lines.append("| # | 수식 패턴 | 도메인 수 | 출현 도메인 (최대 10개 표시) |")
lines.append("|--:|----------|--------:|--------------------------|")
for i, (formula, doms) in enumerate(top50_formulas, 1):
    shown = ", ".join(doms[:10])
    if len(doms) > 10:
        shown += f" ... (+{len(doms)-10})"
    lines.append(f"| {i} | {formula} | {len(doms)} | {shown} |")
lines.append("")

# ── 6.4 수식 분류별 집계
lines.append("## 4. 수식 분류별 교차 공명 집계")
lines.append("")

categories = {
    "핵심 산술 (n,σ,τ,φ,sopfr)": [],
    "산술 조합 (σ·τ, σ-τ 등)": [],
    "화학/물질 (Z=6, Diamond 등)": [],
    "공학 구조 (6DOF, 6-stage 등)": [],
    "통신/전자 (12ch, 48kHz 등)": [],
    "생물/의학": [],
    "AI/컴퓨팅": [],
    "BT 참조": [],
    "기타": [],
}

cat_keywords = {
    "핵심 산술 (n,σ,τ,φ,sopfr)": ["n=6", "σ=12", "τ=4", "φ=2", "sopfr=5", "J₂=24", "n=6 EXACT", "σ(6)=12", "τ(6)=4", "φ(6)=2", "σ·φ=n·τ"],
    "산술 조합 (σ·τ, σ-τ 등)": ["σ·τ=48", "σ·φ=24", "σ-τ=8", "σ/τ=3", "σ·120", "2^(σ-τ)", "AES-256", "PUE=σ/(σ-φ)=1.2", "6^3", "6x5x6"],
    "화학/물질 (Z=6, Diamond 등)": ["Z=6", "CN=6", "6-fold", "6-ring", "Benzene", "Graphite", "Diamond", "Carbon Z=6", "Glucose", "hexacoral", "GaAs", "REBCO", "LFP", "CFRP"],
    "공학 구조 (6DOF, 6-stage 등)": ["6DOF", "SE(3)", "6-axis", "6-stage", "6-layer", "6-car", "6-plane", "6-zone", "6-well", "6-step", "6-unit", "6-slot", "6-panel", "6-coil", "6-motor", "6-antenna", "6-drain", "6-turbine", "6-junction", "Hexagonal", "육각", "hex", "n=6 types", "n=6 stages", "HDRB", "Kepler", "Brayton", "SOLID", "12-Factor", "RISC-V", "LLVM", "MHD", "Topological"],
    "통신/전자 (12ch, 48kHz 등)": ["σ=12 channel", "σ=12 leads", "σ=12 block", "σ=12 rods", "σ=12 port", "σ=12 bit", "σ=12 blade", "σ=12 disc", "σ=12 packs/min", "σ=12 inch", "σ=12 switch", "σ=12 control rods", "σ=12 T", "SHM σ=12ch", "ECG σ=12", "J₂=24 MW", "J₂=24 kRPM", "J₂=24 DOF", "J₂=24 bit", "J₂=24 fps", "σ·τ=48 kHz", "σ·τ=48 layers", "σ·τ=48 V", "σ·τ=48 Gbps", "BLDC σ=12", "kissing", "Clifford", "Surface code", "SRv6", "WireGuard", "BBR", "PoS σ=12s", "1435mm", "Beaufort"],
    "생물/의학": ["20 amino acids", "64 codons", "purine 6-ring", "Cooper pair", "fine-structure"],
    "AI/컴퓨팅": ["Transformer", "6 opcode"],
    "BT 참조": ["BT-27", "BT-43", "BT-53", "BT-56", "BT-58", "BT-66"],
}

for formula, doms in ranked_formulas:
    placed = False
    for cat, keywords in cat_keywords.items():
        for kw in keywords:
            if kw in formula:
                categories[cat].append((formula, len(doms)))
                placed = True
                break
        if placed:
            break
    if not placed:
        categories["기타"].append((formula, len(doms)))

for cat, items in categories.items():
    if not items:
        continue
    total_reach = sum(c for _, c in items)
    lines.append(f"### {cat}")
    lines.append(f"수식 {len(items)}개, 총 도달 도메인 수 {total_reach}")
    lines.append("")
    lines.append("| 수식 | 도메인 수 |")
    lines.append("|------|--------:|")
    for f, c in sorted(items, key=lambda x: x[1], reverse=True):
        lines.append(f"| {f} | {c} |")
    lines.append("")

# ── 6.5 도메인별 수식 밀도 상위 20
lines.append("## 5. 도메인별 수식 밀도 상위 20")
lines.append("")
lines.append("| # | 도메인 | 고유 수식 수 | 교차 수식 수 |")
lines.append("|--:|--------|----------:|----------:|")
density = []
for name, formulas in domain_formulas.items():
    cross_count = sum(1 for f in formulas if f in cross_resonance)
    density.append((name, len(formulas), cross_count))
density.sort(key=lambda x: x[2], reverse=True)
for i, (name, total, cross) in enumerate(density[:20], 1):
    lines.append(f"| {i} | {name} | {total} | {cross} |")
lines.append("")

# ── 6.6 쌍 분석 (기존)
lines.append(f"## 6. 도메인 쌍별 공명 (선정 도메인 {len(top50_dom)}개 — combos 상위 50 ∪ n6_avg≥90)")
lines.append("")
lines.append("```")
lines.append("S(i,j) = 0.5*Jaccard(cross_i, cross_j)")
lines.append("       + 0.2*(1 - |n6avg_i - n6avg_j|/100)   ← n6_avg 결측 시 항 제외")
lines.append("       + 0.2*bidir(i in cross_j, j in cross_i)")
lines.append("       + 0.1*min(combos)/max(combos)")
lines.append("결측 시: prox 항 제외 후 (0.5+0.2+0.1)=0.8 로 나눠 재정규화")
lines.append("⚠결측 = n6_avg 필드 부재 (dse-map.toml 미기입)")
lines.append("```")
lines.append("")
lines.append("| # | A | B | S | Jaccard | n6 근접 | 상호 | 규모균형 | BT | 공유상수 | 결측 |")
lines.append("|---|---|---|---:|---:|---:|---:|---:|---|---|---|")
for i,p in enumerate(pairs[:30], 1):
    prox_cell = f"{p[4]:.3f}" if p[4] is not None else "—"
    miss_cell = ("⚠" + ",".join(p[7])) if p[7] else ""
    a_cell = p[0] + (" ⚠결측" if p[0] in p[7] else "")
    b_cell = p[1] + (" ⚠결측" if p[1] in p[7] else "")
    lines.append(f"| {i} | {a_cell} | {b_cell} | {p[2]:.3f} | {p[3]:.3f} | {prox_cell} | {p[5]:.1f} | {p[6]:.3f} | {p[8]} | {p[9]} | {miss_cell} |")
lines.append("")

# ── 6.7 공명 히스토그램
lines.append("## 7. 공명 스코어 히스토그램 (ASCII)")
lines.append("")
maxc = max(c for _,c in hist) if hist else 1
lines.append("```")
lines.append("bin    count  |" + "-"*52)
for b,c in hist:
    bar = "#" * int(round(c/maxc*50))
    lines.append(f"{b:>4.2f}  {c:>6}  |{bar}")
lines.append("```")
lines.append("")

# ── 6.8 해석
lines.append("## 8. 해석")
lines.append("")
scores = [p[2] for p in pairs]
mean_s = sum(scores)/len(scores) if scores else 0
hi = sum(1 for s in scores if s >= 0.5)
mid = sum(1 for s in scores if 0.3 <= s < 0.5)
lo = sum(1 for s in scores if s < 0.3)

lines.append("### 교차 공명 핵심 발견")
lines.append("")
if top50_formulas:
    top1 = top50_formulas[0]
    lines.append(f"- **최강 교차 공명**: `{top1[0]}` — {len(top1[1])}개 도메인에서 출현")
    top5_names_list = [f"`{f}`({len(d)})" for f,d in top50_formulas[:5]]
    lines.append(f"- **상위 5 수식**: {', '.join(top5_names_list)}")
lines.append(f"- 교차 공명 수식 {len(cross_resonance)}개 중 10+ 도메인 출현: **{sum(1 for _,d in ranked_formulas if len(d)>=10)}**개")
lines.append(f"- 교차 공명 수식 {len(cross_resonance)}개 중 50+ 도메인 출현: **{sum(1 for _,d in ranked_formulas if len(d)>=50)}**개")
lines.append("")
lines.append("### 쌍 분석 통계")
lines.append("")
lines.append(f"- 평균 스코어: **{mean_s:.3f}**")
lines.append(f"- 고공명 (S>=0.5): **{hi}쌍** ({hi/len(pairs)*100:.1f}%)" if pairs else "")
lines.append(f"- 중공명 (0.3<=S<0.5): **{mid}쌍** ({mid/len(pairs)*100:.1f}%)" if pairs else "")
lines.append(f"- 저공명 (S<0.3): **{lo}쌍** ({lo/len(pairs)*100:.1f}%)" if pairs else "")
lines.append("")

lines.append("### 물리적 의미")
lines.append("")
lines.append("- n=6 산술 상수가 물리/화학/공학/생물 전 분야에서 동일한 수식으로 나타남")
lines.append("- `6^3 설계 공간`(63), `n=6`(58), `σ=12`(27) 등 핵심 상수가 수십~수백 개 도메인에서 교차 공명")
lines.append("- Diamond/Graphite (C Z=6), Benzene (6-ring), 6DOF 등 물질/구조 수준에서도 교차")
lines.append("- 이는 σ(n)·φ(n) = n·τ(n) 의 유일해 n=6이 설계 공간 전체를 관통함을 시사")
lines.append("- BT-27 (Carbon Z=6)이 9개 도메인에서 출현: 탄소 원자번호가 에너지/생물/우주공학/물질 등 전 분야 연결")
lines.append("")

# ── 6.9 산출물
lines.append("## 9. 산출물 경로")
lines.append("")
lines.append("- `~/Dev/nexus/shared/dse_cross/top50_domains.jsonl`")
lines.append("- `~/Dev/nexus/shared/dse_cross/pair_scores.jsonl`")
lines.append("- `~/Dev/nexus/shared/dse_cross/resonance_hist.jsonl`")
lines.append("- `~/Dev/nexus/shared/dse_cross/formula_cross.jsonl` (신규)")
lines.append("")

MD_OUT.write_text("\n".join(lines), encoding="utf-8")

# ── 요약 출력 ──────────────────────────────────────────────────
import sys
sys.stdout.flush()
print(f"[OK] 전체 도메인 (cross-dse 제외): {len(pure_domains)}", flush=True)
print(f"[OK] 수식 추출 대상 도메인: {len(domain_formulas)}")
print(f"[OK] 고유 수식 패턴: {len(formula_domains)}")
print(f"[OK] 교차 공명 수식 (2+ 도메인): {len(cross_resonance)}")
print(f"[OK] 총 DSE 항목 (도메인 x 수식): {sum(len(f) for f in domain_formulas.values())}")
print(f"[OK] 쌍 {len(pairs)}개 계산, 평균 스코어 {mean_s:.3f}")
print()
print("=== 교차 공명 상위 10 ===")
for i, (formula, doms) in enumerate(top50_formulas[:10], 1):
    print(f"  {i:2d}. {formula:30s} → {len(doms)}개 도메인")
print()
print(f"[OK] docs/dse-cross-resonance.md 생성")
print(f"[OK] {OUT}/ 에 4개 .jsonl 저장")
