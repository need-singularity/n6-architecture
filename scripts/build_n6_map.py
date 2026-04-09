#!/usr/bin/env python3
"""
build_n6_map.py — n6-architecture 전체 5계층 3D 맵 데이터 생성

L0: 원소/상수 (atlas-constants 기본 7 + 파생 비율)
L1: 재료/화학 (DSE 재료 도메인)
L2: 조합/응용 (DSE 전체 도메인 + cross-DSE)
L3: 기술/기법 (techniques/ + 도메인 기술)
L4: 제품 (products.json 섹션 + 제품)

출력: docs/n6-map-data.json
"""
import json, tomllib, re, os, sys
from pathlib import Path

try:
    ROOT = Path(__file__).resolve().parent.parent
except NameError:
    ROOT = Path.cwd()

DOCS = ROOT / "docs"
CONFIG = ROOT / "config"

# ── 재료 키워드 (L1 분류용) ──
MATERIAL_KEYWORDS = {
    "material", "crystal", "graphene", "superconductor", "ceramic",
    "polymer", "composite", "alloy", "metal", "glass", "diamond",
    "silicon", "ferroelectric", "piezoelectric", "thermoelectric",
    "topological", "magnetic", "dielectric", "foam", "refractory",
    "biomaterial", "abrasive", "aramid", "nylon", "concrete",
    "aluminum", "steel", "titanium", "copper", "lithium",
    "photonic-crystal", "liquid-crystal", "pet-film", "tire-cord",
    "crystallography", "metal-organic"
}

# ── 화학/원소 키워드 (L0~L1 경계) ──
CHEMISTRY_KEYWORDS = {
    "chemistry", "synthesis", "element", "periodic", "compound",
    "reaction", "catalyst", "enzyme", "molecular", "atomic",
    "hydrogen", "oxygen", "carbon", "nitrogen", "helium",
    "food-chemistry", "battery-chemistry", "electrochemistry"
}


def is_material_domain(name: str) -> bool:
    return any(kw in name for kw in MATERIAL_KEYWORDS)


def is_chemistry_domain(name: str) -> bool:
    return any(kw in name for kw in CHEMISTRY_KEYWORDS)


def load_constants():
    """atlas-constants.md 에서 기본 7상수 + 파생 비율 파싱"""
    nodes = []
    path = DOCS / "atlas-constants.md"
    if not path.exists():
        return nodes

    text = path.read_text(encoding="utf-8")

    # Base Constants (7)
    base = [
        ("σ=12", "σ(6) = 1+2+3+6 = 12", "sum of divisors"),
        ("τ=4",  "τ(6) = |{1,2,3,6}| = 4", "number of divisors"),
        ("φ=2",  "φ(6) = |{1,5}| = 2", "Euler totient"),
        ("sopfr=5", "2+3 = 5", "sum of prime factors"),
        ("J₂=24", "6²·∏(1-1/p²) = 24", "Jordan function"),
        ("μ=1",  "(-1)² = 1", "Möbius function"),
        ("n=6",  "first perfect number", "the number itself"),
    ]
    for sym, desc, role in base:
        nodes.append({
            "id": f"const_{sym.split('=')[0]}",
            "label": sym,
            "layer": 0,
            "type": "constant",
            "description": f"{role}: {desc}",
        })

    # Derived Ratios — 테이블 행 파싱
    ratio_pattern = re.compile(
        r"\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|"
    )
    in_derived = False
    for line in text.splitlines():
        if "Derived Ratios" in line:
            in_derived = True
            continue
        if in_derived and line.startswith("## "):
            in_derived = False
            continue
        if in_derived:
            m = ratio_pattern.match(line)
            if m:
                expr = m.group(1).strip()
                val = m.group(2).strip()
                app = m.group(3).strip()
                dom = m.group(4).strip()
                if expr.startswith("Expression") or expr.startswith("---"):
                    continue
                cid = re.sub(r"[^a-zA-Z0-9_]", "_", expr)[:40]
                nodes.append({
                    "id": f"ratio_{cid}",
                    "label": f"{expr} = {val}",
                    "layer": 0,
                    "type": "ratio",
                    "description": app,
                    "domain": dom,
                })
    return nodes


def load_dse_domains():
    """dse-map.toml 에서 도메인 + cross_dse 엣지"""
    nodes, edges = [], []
    path = DOCS / "dse-map.toml"
    if not path.exists():
        return nodes, edges

    with open(path, "rb") as f:
        data = tomllib.load(f)

    for key, val in data.items():
        if key == "meta" or not isinstance(val, dict):
            continue

        # 계층 분류
        if is_chemistry_domain(key):
            layer = 0  # 원소/화학
        elif is_material_domain(key):
            layer = 1  # 재료
        else:
            layer = 2  # 조합/응용

        combos = val.get("combos", 0)
        alien = val.get("alien_level", 0)
        levels = val.get("levels", [])
        n6_max = val.get("n6_max", 0)

        nodes.append({
            "id": f"dse_{key}",
            "label": key,
            "layer": layer,
            "type": "dse_domain",
            "combos": combos,
            "alien_level": alien,
            "levels": levels,
            "n6_max": n6_max,
            "note": val.get("note", ""),
        })

        # cross-DSE edges
        for target in val.get("cross_dse", []):
            edges.append({
                "source": f"dse_{key}",
                "target": f"dse_{target}",
                "type": "cross_dse",
            })

    return nodes, edges


def load_techniques():
    """techniques/ 디렉토리 파이썬 파일 → L3 노드"""
    nodes = []
    tech_dir = ROOT / "techniques"
    if not tech_dir.exists():
        return nodes

    for py in sorted(tech_dir.glob("*.py")):
        name = py.stem
        # 첫 docstring 또는 주석 추출
        desc = ""
        try:
            text = py.read_text(encoding="utf-8")
            m = re.search(r'"""(.+?)"""', text, re.DOTALL)
            if m:
                desc = m.group(1).strip().split("\n")[0]
            elif text.startswith("#"):
                desc = text.split("\n")[0].lstrip("# ").strip()
        except Exception:
            pass

        nodes.append({
            "id": f"tech_{name}",
            "label": name.replace("_", " "),
            "layer": 3,
            "type": "technique",
            "description": desc,
            "file": f"techniques/{py.name}",
        })
    return nodes


def load_products():
    """products.json → L4 섹션 + 제품, L3 도메인/도구 연결"""
    nodes, edges = [], []
    path = CONFIG / "products.json"
    if not path.exists():
        return nodes, edges

    data = json.loads(path.read_text(encoding="utf-8"))

    for sec in data.get("sections", []):
        sid = sec["id"]
        # 섹션 노드 (L4)
        nodes.append({
            "id": f"sec_{sid}",
            "label": f"{sec.get('icon','')} {sec.get('title', sid)}",
            "layer": 4,
            "type": "section",
            "alien_index": sec.get("alien_index", 0),
            "bt_count": sec.get("bt_count", 0),
            "bt_exact_pct": sec.get("bt_exact_pct", 0),
            "closure_grade": sec.get("closure_grade", 0),
        })

        # 제품 노드 (L4)
        for prod in sec.get("products", []):
            pid = re.sub(r"[^a-zA-Z0-9가-힣_]", "_", prod["name"])[:40]
            full_id = f"prod_{sid}_{pid}"
            nodes.append({
                "id": full_id,
                "label": prod["name"],
                "layer": 4,
                "type": "product",
                "ufo": prod.get("ufo", 0),
                "description": prod.get("description", ""),
                "section": sid,
            })
            edges.append({
                "source": f"sec_{sid}",
                "target": full_id,
                "type": "section_product",
            })

        # 섹션→DSE 도메인 연결
        for dom in sec.get("domains", []):
            edges.append({
                "source": f"sec_{sid}",
                "target": f"dse_{dom}",
                "type": "section_domain",
            })

        # 섹션→도구 연결 (참고)
        for tool in sec.get("tools", []):
            edges.append({
                "source": f"sec_{sid}",
                "target": f"tool_{tool}",
                "type": "section_tool",
            })

    return nodes, edges


def load_breakthroughs():
    """breakthrough-theorems.md → BT 노드 + cross-link 엣지 (L3)"""
    nodes, edges = [], []
    path = DOCS / "breakthrough-theorems.md"
    if not path.exists():
        return nodes, edges
    text = path.read_text(encoding="utf-8")
    header_re = re.compile(r"^##\s+BT-(\d+)\s*:?\s*(.*)$", re.MULTILINE)
    matches = list(header_re.finditer(text))
    for i, m in enumerate(matches):
        num = int(m.group(1))
        title = m.group(2).strip()
        start = m.end()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        body = text[start:end]
        bt_id = f"bt_{num}"
        nodes.append({
            "id": bt_id,
            "label": f"BT-{num}: {title}"[:80],
            "layer": 3,
            "type": "breakthrough",
            "bt_num": num,
            "title": title,
        })
        # Cross-link: BT-xxx 참조 → 엣지
        for cm in re.finditer(r"BT-(\d+)", body):
            tgt = int(cm.group(1))
            if tgt != num:
                edges.append({
                    "source": bt_id,
                    "target": f"bt_{tgt}",
                    "type": "bt_crosslink",
                })
    return nodes, edges


def load_tools():
    """Rust/Python 도구 → L3 노드"""
    nodes = []
    tools_dir = ROOT / "tools"
    if not tools_dir.exists():
        return nodes
    for d in sorted(tools_dir.iterdir()):
        if d.is_dir() and not d.name.startswith("."):
            nodes.append({
                "id": f"tool_{d.name}",
                "label": d.name,
                "layer": 3,
                "type": "tool",
                "path": f"tools/{d.name}",
            })
    return nodes


def _keywords(text: str) -> set[str]:
    """텍스트에서 매칭용 키워드 추출"""
    return {w for w in re.split(r"[\s,/·_\-]+", text.lower()) if len(w) >= 3}


SECTION_KEYWORDS = {
    "fusion":      {"fusion", "plasma", "tokamak", "tritium", "deuterium", "blanket", "magnetic", "confinement"},
    "chip":        {"chip", "semiconductor", "transistor", "silicon", "dram", "memory", "nand", "cpu", "gpu", "fpga", "asic", "lithograph", "wafer", "process", "interconnect", "pim"},
    "energy":      {"energy", "solar", "battery", "grid", "power", "wind", "turbine", "thermal", "hydrogen", "fuel"},
    "ai":          {"neural", "attention", "transformer", "moe", "scaling", "inference", "training", "llm", "dropout", "norm", "activation", "decoding", "embedding", "gradient"},
    "environment": {"environment", "carbon", "water", "recycling", "desalination", "pollution", "climate", "soil", "ocean", "waste"},
    "physics":     {"particle", "quantum", "neutrino", "cosmology", "relativity", "gravity", "string", "higgs", "boson", "standard"},
    "materials":   {"material", "crystal", "alloy", "polymer", "ceramic", "graphene", "superconductor", "composite", "diamond", "glass", "metal", "foam", "piezo", "ferr"},
    "robotics":    {"robot", "actuator", "servo", "locomotion", "manipulator", "humanoid", "autonomous", "sensor", "lidar"},
    "software":    {"compiler", "operating", "programming", "algorithm", "database", "file", "kernel", "runtime", "protocol"},
    "display":     {"display", "oled", "lcd", "pixel", "refresh", "hdr", "micro", "panel", "backlight"},
    "audio":       {"audio", "speaker", "microphone", "codec", "acoustic", "noise", "dac", "amplifier", "equalizer"},
    "safety":      {"safety", "fire", "earthquake", "structural", "redundan", "fault", "hazard", "seismic"},
    "play":        {"game", "haptic", "render", "engine", "simulation"},
    "aerospace":   {"aerospace", "rocket", "satellite", "orbit", "propulsion", "space", "launch", "reentry"},
    "sf":          {"consciousness", "telekinesis", "telepathy", "dream", "mind", "brain", "neuro"},
    "frontier":    {"frontier", "civilization", "religion", "law", "currency", "insurance", "architecture", "ferment"},
}


def add_cross_layer_edges(all_nodes, edges):
    """모든 계층간 촘촘한 연결 — 상수→모두, 재료→모두, 응용↔기술↔제품"""
    node_map = {n["id"]: n for n in all_nodes}

    by_layer = {i: [] for i in range(5)}
    for n in all_nodes:
        by_layer[n["layer"]].append(n)

    dse_nodes = [n for n in all_nodes if n["type"] == "dse_domain"]
    sec_nodes = [n for n in all_nodes if n["type"] == "section"]
    tech_nodes = [n for n in all_nodes if n["type"] == "technique"]
    tool_nodes = [n for n in all_nodes if n["type"] == "tool"]

    # ═══ 1. 상수(L0) → 모든 상위 계층 ═══
    for const in by_layer[0]:
        const_kw = _keywords(
            const.get("domain", "") + " " + const.get("description", ""))

        # 상수 → DSE 도메인 (L1+L2)
        for dse in dse_nodes:
            dse_kw = _keywords(dse["label"] + " " + dse.get("note", ""))
            if const_kw & dse_kw:
                edges.append({"source": const["id"], "target": dse["id"],
                              "type": "constant_domain"})

        # 상수 → 섹션 (L4)
        for sec in sec_nodes:
            sid = sec["id"].replace("sec_", "")
            if const_kw & SECTION_KEYWORDS.get(sid, set()):
                edges.append({"source": const["id"], "target": sec["id"],
                              "type": "constant_section"})

        # 상수 → 기법 (L3)
        for tech in tech_nodes:
            tech_kw = _keywords(tech["label"] + " " + tech.get("description", ""))
            if const_kw & tech_kw:
                edges.append({"source": const["id"], "target": tech["id"],
                              "type": "constant_technique"})

    # 기본 상수 7개 → 모든 섹션 (n=6은 만물의 기초)
    for cid in [n["id"] for n in by_layer[0] if n["type"] == "constant"]:
        for sec in sec_nodes:
            edges.append({"source": cid, "target": sec["id"],
                          "type": "foundation"})

    # ═══ 2. 재료(L1) → 응용(L2) + 기술(L3) + 제품(L4) ═══
    for mat in by_layer[1]:
        mat_kw = _keywords(mat["label"] + " " + mat.get("note", ""))

        for app in by_layer[2]:
            app_kw = _keywords(app["label"] + " " + app.get("note", ""))
            if mat_kw & app_kw:
                edges.append({"source": mat["id"], "target": app["id"],
                              "type": "material_application"})

        for sec in sec_nodes:
            sid = sec["id"].replace("sec_", "")
            if mat_kw & SECTION_KEYWORDS.get(sid, set()):
                edges.append({"source": mat["id"], "target": sec["id"],
                              "type": "material_section"})

        for tool in tool_nodes:
            if _keywords(tool["label"]) & mat_kw:
                edges.append({"source": mat["id"], "target": tool["id"],
                              "type": "material_tool"})

    # ═══ 3. 응용(L2) → 기술(L3) + 제품(L4) ═══
    for app in by_layer[2]:
        app_kw = _keywords(app["label"] + " " + app.get("note", ""))

        for sec in sec_nodes:
            sid = sec["id"].replace("sec_", "")
            if app_kw & SECTION_KEYWORDS.get(sid, set()):
                edges.append({"source": app["id"], "target": sec["id"],
                              "type": "application_section"})

        for tech in tech_nodes:
            tech_kw = _keywords(tech["label"] + " " + tech.get("description", ""))
            if app_kw & tech_kw:
                edges.append({"source": app["id"], "target": tech["id"],
                              "type": "application_technique"})

    # ═══ 4. 기법(L3) → 제품(L4) ═══
    for tech in tech_nodes:
        tech_kw = _keywords(tech["label"] + " " + tech.get("description", ""))
        for sec in sec_nodes:
            sid = sec["id"].replace("sec_", "")
            if tech_kw & SECTION_KEYWORDS.get(sid, set()):
                edges.append({"source": tech["id"], "target": sec["id"],
                              "type": "technique_section"})

    # ═══ 5. 도구(L3) → 제품(L4) ═══
    for tool in tool_nodes:
        tool_kw = _keywords(tool["label"])
        for sec in sec_nodes:
            sid = sec["id"].replace("sec_", "")
            if tool_kw & SECTION_KEYWORDS.get(sid, set()):
                edges.append({"source": tool["id"], "target": sec["id"],
                              "type": "tool_section"})

    # ═══ 6. DSE levels "Material/Fuel/Cell" → L1 재료 ═══
    for dse in dse_nodes:
        levels = [l.lower() for l in dse.get("levels", [])]
        if any("material" in l or "cell" in l or "fuel" in l for l in levels):
            dse_kw = _keywords(dse["label"])
            for mat in by_layer[1]:
                if _keywords(mat["label"]) & dse_kw:
                    edges.append({"source": mat["id"], "target": dse["id"],
                                  "type": "material_uses"})


def deduplicate_edges(edges):
    """엣지 중복 제거"""
    seen = set()
    result = []
    for e in edges:
        key = (e["source"], e["target"], e["type"])
        rev = (e["target"], e["source"], e["type"])
        if key not in seen and rev not in seen:
            seen.add(key)
            result.append(e)
    return result


def filter_dangling_edges(edges, node_ids):
    """존재하지 않는 노드를 가리키는 엣지 제거"""
    return [e for e in edges if e["source"] in node_ids and e["target"] in node_ids]


def main():
    all_nodes = []
    all_edges = []

    print("Loading constants...")
    all_nodes.extend(load_constants())

    print("Loading DSE domains...")
    dse_nodes, dse_edges = load_dse_domains()
    all_nodes.extend(dse_nodes)
    all_edges.extend(dse_edges)

    print("Loading techniques...")
    all_nodes.extend(load_techniques())

    print("Loading breakthroughs...")
    bt_nodes, bt_edges = load_breakthroughs()
    all_nodes.extend(bt_nodes)
    all_edges.extend(bt_edges)

    print("Loading tools...")
    all_nodes.extend(load_tools())

    print("Loading products...")
    prod_nodes, prod_edges = load_products()
    all_nodes.extend(prod_nodes)
    all_edges.extend(prod_edges)

    print("Adding cross-layer edges...")
    add_cross_layer_edges(all_nodes, all_edges)

    node_ids = {n["id"] for n in all_nodes}
    all_edges = filter_dangling_edges(all_edges, node_ids)
    all_edges = deduplicate_edges(all_edges)

    # 계층별 통계
    layer_counts = {}
    for n in all_nodes:
        layer_counts[n["layer"]] = layer_counts.get(n["layer"], 0) + 1

    output = {
        "meta": {
            "title": "n=6 Architecture — Full Layer Map",
            "version": "1.0",
            "layers": {
                "0": "원소/상수 (Constants & Elements)",
                "1": "재료/화학 (Materials & Chemistry)",
                "2": "조합/응용 (Applications & Combinations)",
                "3": "기술/기법 (Technology & Techniques)",
                "4": "제품 (Products)",
            },
            "node_count": len(all_nodes),
            "edge_count": len(all_edges),
            "layer_counts": layer_counts,
        },
        "nodes": all_nodes,
        "edges": all_edges,
    }

    out_path = DOCS / "n6-map-data.json"
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(output, f, ensure_ascii=False, indent=2)

    print(f"\n✅ Generated {out_path}")
    print(f"   Nodes: {len(all_nodes)} (L0={layer_counts.get(0,0)}, L1={layer_counts.get(1,0)}, L2={layer_counts.get(2,0)}, L3={layer_counts.get(3,0)}, L4={layer_counts.get(4,0)})")
    print(f"   Edges: {len(all_edges)}")


if __name__ == "__main__":
    main()
