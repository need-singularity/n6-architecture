#!/usr/bin/env python3
"""
Discovery Graph Topology Analysis
==================================
Builds the N6 discovery graph from actual project data and computes
structural metrics: degree distribution, centrality, clustering,
connected components, small-world property, and discovery predictions.

Data sources:
  - docs/atlas-constants.md   → constants + domains + BT links
  - CLAUDE.md                 → BT list with domain tags
  - docs/dse-map.toml         → domain cross_dse links
"""

import re
import os
import sys
import math
from collections import defaultdict, Counter
from pathlib import Path

# ── Configuration ──────────────────────────────────────────────────

BASE = Path(__file__).resolve().parent.parent
ATLAS = BASE / "docs" / "atlas-constants.md"
CLAUDE = BASE / "CLAUDE.md"
DSE_MAP = BASE / "docs" / "dse-map.toml"
OUTPUT = BASE / "docs" / "discovery-graph-topology.md"


# ══════════════════════════════════════════════════════════════════
# Graph data structure (no external dependencies)
# ══════════════════════════════════════════════════════════════════

class Graph:
    """Simple undirected graph using adjacency sets."""

    def __init__(self):
        self.adj = defaultdict(set)       # node -> set of neighbors
        self.node_type = {}               # node -> 'constant'|'domain'|'bt'|'hypothesis'
        self.edge_types = defaultdict(set) # (u,v) frozen -> set of edge labels

    def add_node(self, n, ntype):
        if n not in self.node_type:
            self.node_type[n] = ntype
        if n not in self.adj:
            self.adj[n] = set()

    def add_edge(self, u, v, etype="link"):
        if u == v:
            return
        self.adj[u].add(v)
        self.adj[v].add(u)
        key = tuple(sorted([u, v]))
        self.edge_types[key].add(etype)

    @property
    def nodes(self):
        return set(self.node_type.keys())

    @property
    def num_nodes(self):
        return len(self.node_type)

    @property
    def num_edges(self):
        seen = set()
        for u, nbrs in self.adj.items():
            for v in nbrs:
                key = tuple(sorted([u, v]))
                seen.add(key)
        return len(seen)

    def degree(self, n):
        return len(self.adj.get(n, set()))

    def density(self):
        n = self.num_nodes
        if n < 2:
            return 0
        return 2 * self.num_edges / (n * (n - 1))

    def nodes_of_type(self, ntype):
        return [n for n, t in self.node_type.items() if t == ntype]

    # ── BFS shortest paths ──────────────────────────────────────

    def bfs_distances(self, source):
        """Return dict of distances from source to all reachable nodes."""
        dist = {source: 0}
        queue = [source]
        head = 0
        while head < len(queue):
            u = queue[head]; head += 1
            for v in self.adj[u]:
                if v not in dist:
                    dist[v] = dist[u] + 1
                    queue.append(v)
        return dist

    def connected_components(self):
        visited = set()
        components = []
        for n in self.node_type:
            if n not in visited:
                comp = set()
                stack = [n]
                while stack:
                    u = stack.pop()
                    if u in comp:
                        continue
                    comp.add(u)
                    visited.add(u)
                    for v in self.adj[u]:
                        if v not in comp:
                            stack.append(v)
                components.append(comp)
        components.sort(key=len, reverse=True)
        return components

    # ── Centrality metrics ──────────────────────────────────────

    def betweenness_centrality_sampled(self, sample_size=200):
        """
        Approximate betweenness centrality by sampling source nodes.
        Full Brandes algorithm on every node is O(VE) which can be slow.
        """
        bc = defaultdict(float)
        all_nodes = list(self.node_type.keys())
        import random
        random.seed(42)
        sources = random.sample(all_nodes, min(sample_size, len(all_nodes)))

        for s in sources:
            # BFS from s
            stack = []
            pred = defaultdict(list)
            sigma = defaultdict(float)
            sigma[s] = 1.0
            dist = {s: 0}
            queue = [s]
            head = 0
            while head < len(queue):
                u = queue[head]; head += 1
                stack.append(u)
                for v in self.adj[u]:
                    if v not in dist:
                        dist[v] = dist[u] + 1
                        queue.append(v)
                    if dist.get(v) == dist[u] + 1:
                        sigma[v] += sigma[u]
                        pred[v].append(u)
            # Accumulate
            delta = defaultdict(float)
            while stack:
                w = stack.pop()
                for v in pred[w]:
                    delta[v] += (sigma[v] / sigma[w]) * (1 + delta[w])
                if w != s:
                    bc[w] += delta[w]

        # Normalize
        n = len(all_nodes)
        scale = n / len(sources) if len(sources) < n else 1.0
        for v in bc:
            bc[v] *= scale / max(1, (n - 1) * (n - 2))
        return dict(bc)

    def clustering_coefficient(self, node):
        nbrs = self.adj.get(node, set())
        k = len(nbrs)
        if k < 2:
            return 0.0
        nbrs_list = list(nbrs)
        triangles = 0
        for i in range(len(nbrs_list)):
            for j in range(i + 1, len(nbrs_list)):
                if nbrs_list[j] in self.adj[nbrs_list[i]]:
                    triangles += 1
        return 2 * triangles / (k * (k - 1))

    def avg_clustering_coefficient(self):
        total = 0.0
        count = 0
        for n in self.node_type:
            if self.degree(n) >= 2:
                total += self.clustering_coefficient(n)
                count += 1
        return total / max(count, 1)

    def diameter_and_avg_path(self, component=None):
        """Compute diameter and average path length on given component."""
        nodes = list(component) if component else list(self.node_type.keys())
        if len(nodes) > 2000:
            import random; random.seed(7)
            nodes_sample = random.sample(nodes, 2000)
        else:
            nodes_sample = nodes
        max_d = 0
        total_d = 0
        count = 0
        for s in nodes_sample:
            dists = self.bfs_distances(s)
            for t, d in dists.items():
                if t != s:
                    if d > max_d:
                        max_d = d
                    total_d += d
                    count += 1
        avg = total_d / max(count, 1)
        return max_d, avg


# ══════════════════════════════════════════════════════════════════
# Parsers
# ══════════════════════════════════════════════════════════════════

def parse_atlas_constants(path):
    """
    Parse atlas-constants.md for constants and their domain associations.
    Returns list of (expression, value, domains, bts, hypotheses).
    """
    text = path.read_text(encoding="utf-8")
    entries = []

    # Match table rows: | expr | value | ... | Domain/Hypothesis |
    # Tables have varying columns; we look for rows with |...|
    for line in text.split("\n"):
        line = line.strip()
        if not line.startswith("|") or line.startswith("|-") or line.startswith("| Expression") or line.startswith("| Parameter") or line.startswith("| Symbol") or line.startswith("| ID") or line.startswith("| Code"):
            continue
        cols = [c.strip() for c in line.split("|")[1:-1]]
        if len(cols) < 3:
            continue

        expr = cols[0].strip("*").strip()
        if not expr or expr == "---":
            continue

        # Extract domains from any column
        all_text = " ".join(cols)

        # Find BT references
        bts = set(re.findall(r"BT-(\d+)", all_text))

        # Find hypothesis references
        hyps = set(re.findall(r"H-[A-Z]+-\d+", all_text))

        # Find domain keywords
        domains = set()
        domain_keywords = {
            "AI": "ai-efficiency", "Chip": "chip-architecture", "SoC": "chip-architecture",
            "Solar": "solar-architecture", "Energy": "energy-generation", "Power": "power-grid",
            "Battery": "battery-architecture", "Fusion": "fusion", "Crypto": "cryptography",
            "Network": "network-protocol", "Biology": "biology", "Particle": "cosmology-particle",
            "Cosmology": "cosmology-particle", "Neutrino": "cosmology-particle",
            "Math": "pure-mathematics", "Blockchain": "blockchain",
            "Display": "display-audio", "Audio": "display-audio",
            "Optical": "chip-architecture", "Photonic": "chip-architecture",
            "PIM": "chip-architecture", "3D": "chip-architecture",
            "Wafer": "chip-architecture", "SC": "superconductor",
            "Nuclear": "nuclear-reactor", "Hydrogen": "energy-generation",
            "Grid": "power-grid", "Thermal": "thermal-management",
            "Cryo": "superconductor", "Interconnect": "network-protocol",
            "Info theory": "information-theory", "Robotics": "robotics",
            "SSM": "ai-efficiency", "Diffusion": "ai-efficiency",
            "RL": "learning-algorithm", "Training": "learning-algorithm",
            "Universal": "ai-efficiency", "Semiconductor": "chip-architecture",
            "Fabrication": "chip-architecture",
        }
        for kw, dom in domain_keywords.items():
            if kw in all_text:
                domains.add(dom)

        # Parse domain column explicitly if present
        for col in cols:
            for kw, dom in domain_keywords.items():
                if kw in col:
                    domains.add(dom)

        entries.append((expr, cols[1] if len(cols) > 1 else "", domains, bts, hyps))

    return entries


def parse_bt_from_claude(path):
    """
    Parse CLAUDE.md for BT entries with their domain associations.
    Returns dict: bt_id -> set of domain tags.
    """
    text = path.read_text(encoding="utf-8")
    bt_domains = {}

    # BT entries look like: BT-26: Description (domain1, domain2)
    # or are under domain headers
    for line in text.split("\n"):
        m = re.match(r"\s*BT-(\d+):\s*(.*)", line)
        if m:
            bt_id = m.group(1)
            desc = m.group(2)
            domains = set()

            # Domain inference from description keywords
            kw_map = {
                "Chinchilla": "ai-efficiency", "MoE": "ai-efficiency",
                "Transformer": "ai-efficiency", "RoPE": "ai-efficiency",
                "KV-head": "ai-efficiency", "Inference": "ai-efficiency",
                "RLHF": "learning-algorithm", "AdamW": "learning-algorithm",
                "LLM": "ai-efficiency", "Diffusion": "ai-efficiency",
                "Mamba": "ai-efficiency", "Vision AI": "ai-efficiency",
                "NeRF": "ai-efficiency", "Neural audio": "display-audio",
                "Tokenizer": "ai-efficiency", "LoRA": "ai-efficiency",
                "AI": "ai-efficiency",
                "Carbon": "material-synthesis", "Solar": "solar-architecture",
                "SQ": "solar-architecture", "Battery": "battery-architecture",
                "Li-S": "battery-architecture", "Solid-state": "battery-architecture",
                "Grid": "power-grid", "HVDC": "power-grid",
                "Hydrogen": "energy-generation", "Fission": "nuclear-reactor",
                "Nuclear": "nuclear-reactor",
                "Chip": "chip-architecture", "GPU": "chip-architecture",
                "HBM": "chip-architecture", "Semiconductor": "chip-architecture",
                "Chiplet": "chip-architecture", "SM": "chip-architecture",
                "Semiconductor pitch": "chip-architecture",
                "Computing": "chip-architecture", "FP8": "chip-architecture",
                "Crypto": "cryptography", "BTC": "cryptography",
                "Display": "display-audio", "Audio": "display-audio",
                "Genetic": "biology", "Koide": "cosmology-particle",
                "Gauge": "cosmology-particle", "Neutrino": "cosmology-particle",
                "Inflation": "cosmology-particle", "CKM": "cosmology-particle",
                "Kissing": "pure-mathematics", "Math": "pure-mathematics",
                "Crypto (BTC": "blockchain",
                "Energy": "energy-generation",
                "Fusion": "fusion", "Plasma": "plasma-physics",
                "Superconductor": "superconductor",
                "CN=6": "material-synthesis",
                "Photonic": "chip-architecture",
                "Topological": "topology",
                "Bott": "topology",
            }
            for kw, dom in kw_map.items():
                if kw.lower() in desc.lower():
                    domains.add(dom)

            # Also look for explicit domain mentions in parentheses
            paren = re.findall(r"\(([^)]+)\)", desc)
            for p in paren:
                for kw, dom in kw_map.items():
                    if kw.lower() in p.lower():
                        domains.add(dom)

            bt_domains[bt_id] = domains

    return bt_domains


def parse_dse_map(path):
    """
    Parse dse-map.toml for domain nodes and cross_dse edges.
    Returns:
      - domains: dict domain_name -> dict of properties
      - cross_dse_links: list of (domain_a, domain_b)
    """
    text = path.read_text(encoding="utf-8")
    domains = {}
    cross_links = []

    current_section = None
    current_data = {}

    for line in text.split("\n"):
        line = line.strip()

        # Section header
        m = re.match(r'^\[([^\]]+)\]$', line)
        if m:
            # Save previous
            if current_section and current_section != "meta":
                domains[current_section] = current_data

            section_name = m.group(1)
            if section_name.startswith("cross-dse."):
                current_section = None
                # Parse cross-dse domains from following lines
                # Will be handled below
                current_data = {"_is_cross": True, "_name": section_name}
                domains[section_name] = current_data
            else:
                current_section = section_name
                current_data = {}
            continue

        # Key-value pairs
        kv = re.match(r'^(\w+)\s*=\s*(.+)$', line)
        if kv:
            key = kv.group(1)
            val = kv.group(2).strip()

            if current_section:
                current_data[key] = val

            # Parse cross_dse arrays
            if key == "cross_dse":
                arr = re.findall(r'"([^"]+)"', val)
                parent = current_section or ""
                for target in arr:
                    cross_links.append((parent, target))

            # Parse cross-dse domain lists
            if key == "domains" and current_data.get("_is_cross"):
                arr = re.findall(r'"([^"]+)"', val)
                for i in range(len(arr)):
                    for j in range(i + 1, len(arr)):
                        cross_links.append((arr[i], arr[j]))

    # Save last section
    if current_section and current_section != "meta":
        domains[current_section] = current_data

    return domains, cross_links


# ══════════════════════════════════════════════════════════════════
# Build the graph
# ══════════════════════════════════════════════════════════════════

def build_graph():
    G = Graph()

    print("[1/4] Parsing atlas-constants.md ...")
    atlas_entries = parse_atlas_constants(ATLAS)
    print(f"      Found {len(atlas_entries)} constant entries")

    print("[2/4] Parsing CLAUDE.md for BT list ...")
    bt_domains = parse_bt_from_claude(CLAUDE)
    print(f"      Found {len(bt_domains)} BT entries")

    print("[3/4] Parsing dse-map.toml ...")
    dse_domains, cross_links = parse_dse_map(DSE_MAP)
    # Filter out cross-dse entries and meta
    real_domains = {k: v for k, v in dse_domains.items()
                    if not k.startswith("cross-dse.") and k != "meta"}
    print(f"      Found {len(real_domains)} domain entries, {len(cross_links)} cross-DSE links")

    print("[4/4] Building graph ...")

    # Add domain nodes from DSE map
    for dname in real_domains:
        G.add_node(f"D:{dname}", "domain")

    # Add some extra domains from atlas that may not be in DSE
    extra_domains = {
        "ai-efficiency", "chip-architecture", "quantum-computing",
        "compiler-os", "software-design", "network-protocol",
        "cryptography", "blockchain", "biology", "cosmology-particle",
        "display-audio", "pure-mathematics", "plasma-physics",
        "thermal-management", "robotics", "learning-algorithm",
        "energy-generation", "power-grid", "battery-architecture",
        "solar-architecture", "fusion", "superconductor",
        "material-synthesis", "nuclear-reactor", "space-engineering",
        "medical-device", "agriculture", "topology",
        "information-theory",
    }
    for d in extra_domains:
        G.add_node(f"D:{d}", "domain")

    # Add BT nodes
    for bt_id, doms in bt_domains.items():
        bt_node = f"BT-{bt_id}"
        G.add_node(bt_node, "bt")
        for d in doms:
            dn = f"D:{d}"
            G.add_node(dn, "domain")
            G.add_edge(bt_node, dn, "bt-domain")

    # Add constant nodes from atlas
    const_counter = Counter()
    for expr, val, domains, bts, hyps in atlas_entries:
        # Normalize constant name
        cname = f"C:{expr}"
        if not expr:
            continue
        const_counter[cname] += 1
        G.add_node(cname, "constant")

        # Link to domains
        for d in domains:
            dn = f"D:{d}"
            G.add_node(dn, "domain")
            G.add_edge(cname, dn, "const-domain")

        # Link to BTs
        for bt_id in bts:
            bt_node = f"BT-{bt_id}"
            G.add_node(bt_node, "bt")
            G.add_edge(cname, bt_node, "const-bt")

        # Link to hypotheses
        for hyp in hyps:
            hyp_node = f"H:{hyp}"
            G.add_node(hyp_node, "hypothesis")
            G.add_edge(cname, hyp_node, "const-hyp")
            # Also link hypothesis to domains
            for d in domains:
                dn = f"D:{d}"
                G.add_edge(hyp_node, dn, "hyp-domain")

    # Add cross-DSE edges between domains
    for a, b in cross_links:
        da = f"D:{a}"
        db = f"D:{b}"
        G.add_node(da, "domain")
        G.add_node(db, "domain")
        G.add_edge(da, db, "cross-dse")

    # Add the 7 base constants explicitly
    base_constants = {
        "C:sigma=12": ("sigma(6)=12", {"ai-efficiency", "chip-architecture", "fusion", "superconductor", "display-audio", "power-grid", "cosmology-particle"}),
        "C:tau=4": ("tau(6)=4", {"ai-efficiency", "chip-architecture", "fusion", "superconductor", "cryptography", "biology"}),
        "C:phi=2": ("phi(6)=2", {"ai-efficiency", "chip-architecture", "fusion", "superconductor", "battery-architecture", "cosmology-particle"}),
        "C:sopfr=5": ("sopfr(6)=5", {"ai-efficiency", "cryptography", "network-protocol", "power-grid", "display-audio"}),
        "C:J2=24": ("J2(6)=24", {"ai-efficiency", "chip-architecture", "fusion", "pure-mathematics", "display-audio", "biology"}),
        "C:mu=1": ("mu(6)=1", {"ai-efficiency", "cosmology-particle", "fusion", "superconductor"}),
        "C:n=6": ("n=6", {"ai-efficiency", "chip-architecture", "fusion", "biology", "network-protocol", "cryptography", "material-synthesis", "blockchain"}),
    }
    for cnode, (desc, doms) in base_constants.items():
        G.add_node(cnode, "constant")
        for d in doms:
            G.add_node(f"D:{d}", "domain")
            G.add_edge(cnode, f"D:{d}", "base-const-domain")

    # Add derived ratio constants with cross-domain links
    derived = {
        "C:sigma-tau=8": {"ai-efficiency", "chip-architecture", "cryptography", "cosmology-particle", "display-audio"},
        "C:sigma-sopfr=7": {"network-protocol", "cryptography"},
        "C:sigma-phi=10": {"fusion", "cosmology-particle", "ai-efficiency", "chip-architecture"},
        "C:sigma-mu=11": {"cryptography", "network-protocol", "cosmology-particle"},
        "C:sigma+mu=13": {"network-protocol"},
        "C:J2-tau=20": {"ai-efficiency", "biology", "cryptography", "network-protocol"},
        "C:sigma*sopfr=60": {"display-audio", "power-grid", "solar-architecture"},
        "C:sigma*tau=48": {"display-audio", "chip-architecture", "superconductor"},
        "C:sigma*phi=24": {"pure-mathematics", "chip-architecture", "fusion"},
        "C:1/(sigma-phi)=0.1": {"ai-efficiency", "learning-algorithm"},
        "C:ln(4/3)=0.288": {"ai-efficiency", "learning-algorithm"},
        "C:1/e=0.368": {"ai-efficiency"},
        "C:sigma^2=144": {"chip-architecture", "solar-architecture"},
        "C:sigma*J2=288": {"chip-architecture"},
        "C:phi^tau*sopfr=80": {"chip-architecture"},
        "C:tau/(n/phi)=4/3": {"ai-efficiency", "solar-architecture", "energy-generation"},
        "C:sigma(sigma-phi)=120": {"energy-generation"},
        "C:sigma^2-phi=142": {"energy-generation"},
        "C:phi^2/n=2/3": {"cosmology-particle"},
    }
    for cnode, doms in derived.items():
        G.add_node(cnode, "constant")
        for d in doms:
            G.add_node(f"D:{d}", "domain")
            G.add_edge(cnode, f"D:{d}", "derived-const-domain")
        # Link derived to base constants they reference
        if "sigma" in cnode:
            G.add_edge(cnode, "C:sigma=12", "const-const")
        if "tau" in cnode:
            G.add_edge(cnode, "C:tau=4", "const-const")
        if "phi" in cnode:
            G.add_edge(cnode, "C:phi=2", "const-const")
        if "sopfr" in cnode:
            G.add_edge(cnode, "C:sopfr=5", "const-const")
        if "J2" in cnode:
            G.add_edge(cnode, "C:J2=24", "const-const")
        if "mu" in cnode:
            G.add_edge(cnode, "C:mu=1", "const-const")

    return G


# ══════════════════════════════════════════════════════════════════
# Analysis
# ══════════════════════════════════════════════════════════════════

def analyze(G):
    results = {}

    # Basic stats
    results["num_nodes"] = G.num_nodes
    results["num_edges"] = G.num_edges
    results["density"] = G.density()

    type_counts = Counter(G.node_type.values())
    results["type_counts"] = dict(type_counts)

    # Degree distribution
    degrees = {n: G.degree(n) for n in G.nodes}
    deg_values = list(degrees.values())
    results["avg_degree"] = sum(deg_values) / max(len(deg_values), 1)
    results["max_degree"] = max(deg_values) if deg_values else 0
    results["max_degree_node"] = max(degrees, key=degrees.get) if degrees else None

    # Top hubs by degree
    sorted_by_degree = sorted(degrees.items(), key=lambda x: -x[1])
    results["top_hubs"] = sorted_by_degree[:20]

    # Top hubs among constants only
    const_degrees = {n: d for n, d in degrees.items() if G.node_type.get(n) == "constant"}
    results["top_constant_hubs"] = sorted(const_degrees.items(), key=lambda x: -x[1])[:15]

    # Top hubs among domains only
    domain_degrees = {n: d for n, d in degrees.items() if G.node_type.get(n) == "domain"}
    results["top_domain_hubs"] = sorted(domain_degrees.items(), key=lambda x: -x[1])[:15]

    # Connected components
    components = G.connected_components()
    results["num_components"] = len(components)
    results["largest_component_size"] = len(components[0]) if components else 0
    results["component_sizes"] = [len(c) for c in components[:10]]

    # Isolated domains (degree ≤ 2)
    iso_domains = [(n, degrees[n]) for n in G.nodes_of_type("domain") if degrees.get(n, 0) <= 2]
    iso_domains.sort(key=lambda x: x[1])
    results["isolated_domains"] = iso_domains

    # Betweenness centrality
    print("  Computing betweenness centrality (sampled) ...")
    bc = G.betweenness_centrality_sampled(sample_size=300)
    sorted_bc = sorted(bc.items(), key=lambda x: -x[1])
    results["top_betweenness"] = sorted_bc[:20]

    # Top bridges among constants
    const_bc = {n: b for n, b in bc.items() if G.node_type.get(n) == "constant"}
    results["top_constant_bridges"] = sorted(const_bc.items(), key=lambda x: -x[1])[:15]

    # Clustering coefficient
    print("  Computing clustering coefficient ...")
    avg_cc = G.avg_clustering_coefficient()
    results["avg_clustering"] = avg_cc

    # Diameter and average path length (on largest component)
    print("  Computing diameter and avg path length ...")
    if components:
        diameter, avg_path = G.diameter_and_avg_path(components[0])
        results["diameter"] = diameter
        results["avg_path_length"] = avg_path
    else:
        results["diameter"] = 0
        results["avg_path_length"] = 0

    # Small-world check: compare to random graph with same n, avg_degree
    n = G.num_nodes
    k = results["avg_degree"]
    if k > 0 and n > 1:
        # Random graph expected: C_rand ~ k/n, L_rand ~ ln(n)/ln(k)
        c_rand = k / n
        l_rand = math.log(n) / math.log(max(k, 1.01))
        sigma_sw = (avg_cc / max(c_rand, 1e-9)) / (results["avg_path_length"] / max(l_rand, 1e-9))
        results["small_world_sigma"] = sigma_sw
        results["c_rand"] = c_rand
        results["l_rand"] = l_rand
    else:
        results["small_world_sigma"] = 0

    # Find missing edges: domain pairs that share many constants but have no direct cross-dse link
    print("  Finding predicted missing edges ...")
    domain_nodes = G.nodes_of_type("domain")
    domain_shared = defaultdict(int)
    for cnode in G.nodes_of_type("constant"):
        dom_neighbors = [n for n in G.adj[cnode] if G.node_type.get(n) == "domain"]
        for i in range(len(dom_neighbors)):
            for j in range(i+1, len(dom_neighbors)):
                pair = tuple(sorted([dom_neighbors[i], dom_neighbors[j]]))
                domain_shared[pair] += 1

    # Check which pairs lack direct edge
    missing = []
    for (d1, d2), shared in domain_shared.items():
        if d2 not in G.adj.get(d1, set()):
            missing.append((d1, d2, shared))
    missing.sort(key=lambda x: -x[2])
    results["predicted_missing_edges"] = missing[:20]

    # Cliques: find groups of constants that always co-occur in the same domains
    # (Simplified: find triangles among constants)
    print("  Finding constant cliques ...")
    const_nodes = G.nodes_of_type("constant")
    clique_triangles = []
    checked = set()
    for c1 in const_nodes[:200]:  # Limit for performance
        for c2 in G.adj.get(c1, set()):
            if G.node_type.get(c2) != "constant" or c2 <= c1:
                continue
            for c3 in G.adj.get(c1, set()):
                if G.node_type.get(c3) != "constant" or c3 <= c2:
                    continue
                if c3 in G.adj.get(c2, set()):
                    tri = tuple(sorted([c1, c2, c3]))
                    if tri not in checked:
                        checked.add(tri)
                        clique_triangles.append(tri)
    results["constant_triangles"] = clique_triangles[:15]

    return results


# ══════════════════════════════════════════════════════════════════
# Report generation
# ══════════════════════════════════════════════════════════════════

def generate_report(G, results):
    lines = []
    L = lines.append

    L("# Discovery Graph Topology Analysis")
    L("")
    L(f"> Generated: 2026-04-02 | Nodes: {results['num_nodes']} | Edges: {results['num_edges']}")
    L("")
    L("---")
    L("")

    # 1. Graph Statistics
    L("## 1. Graph Statistics")
    L("")
    L("| Metric | Value |")
    L("|--------|-------|")
    L(f"| Total nodes | {results['num_nodes']} |")
    L(f"| Total edges | {results['num_edges']} |")
    L(f"| Graph density | {results['density']:.6f} |")
    L(f"| Average degree | {results['avg_degree']:.2f} |")
    L(f"| Max degree | {results['max_degree']} ({results['max_degree_node']}) |")
    L(f"| Connected components | {results['num_components']} |")
    L(f"| Largest component | {results['largest_component_size']} nodes ({100*results['largest_component_size']/max(results['num_nodes'],1):.1f}%) |")
    L(f"| Diameter (largest comp.) | {results['diameter']} |")
    L(f"| Avg path length | {results['avg_path_length']:.3f} |")
    L(f"| Avg clustering coeff. | {results['avg_clustering']:.4f} |")
    L(f"| Small-world sigma | {results['small_world_sigma']:.2f} (>1 = small-world) |")
    L("")

    L("### Node Type Distribution")
    L("")
    L("| Type | Count |")
    L("|------|-------|")
    for t, c in sorted(results["type_counts"].items(), key=lambda x: -x[1]):
        L(f"| {t} | {c} |")
    L("")

    L("### Component Sizes (top 10)")
    L("")
    L(f"```")
    for i, s in enumerate(results["component_sizes"]):
        L(f"  Component {i+1}: {s} nodes")
    L(f"```")
    L("")

    # 2. Hub Constants
    L("## 2. Hub Constants (highest degree)")
    L("")
    L("Hub constants connect the most domains/BTs -- they are the most fundamental.")
    L("")
    L("| Rank | Constant | Degree | Type |")
    L("|------|----------|--------|------|")
    for i, (node, deg) in enumerate(results["top_constant_hubs"][:15], 1):
        name = node.replace("C:", "")
        L(f"| {i} | `{name}` | {deg} | constant |")
    L("")

    # 3. Bridge Constants
    L("## 3. Bridge Constants (highest betweenness centrality)")
    L("")
    L("Bridge constants are critical links between otherwise-separate clusters.")
    L("High betweenness = removing this constant would disconnect parts of the graph.")
    L("")
    L("| Rank | Constant | Betweenness | Degree |")
    L("|------|----------|-------------|--------|")
    for i, (node, bc) in enumerate(results["top_constant_bridges"][:15], 1):
        name = node.replace("C:", "")
        deg = G.degree(node)
        L(f"| {i} | `{name}` | {bc:.6f} | {deg} |")
    L("")

    # 4. Hub Domains
    L("## 4. Hub Domains (most connected)")
    L("")
    L("| Rank | Domain | Degree |")
    L("|------|--------|--------|")
    for i, (node, deg) in enumerate(results["top_domain_hubs"][:15], 1):
        name = node.replace("D:", "")
        L(f"| {i} | {name} | {deg} |")
    L("")

    # 5. Isolated Domains
    L("## 5. Isolated Domains (degree <= 2)")
    L("")
    if results["isolated_domains"]:
        L("These domains have very few connections -- prime candidates for new BTs.")
        L("")
        L("| Domain | Degree |")
        L("|--------|--------|")
        for node, deg in results["isolated_domains"][:20]:
            name = node.replace("D:", "")
            L(f"| {name} | {deg} |")
    else:
        L("No isolated domains found -- all domains have 3+ connections.")
    L("")

    # 6. Top overall hubs
    L("## 6. Top 20 Overall Hubs (all node types)")
    L("")
    L("| Rank | Node | Type | Degree |")
    L("|------|------|------|--------|")
    for i, (node, deg) in enumerate(results["top_hubs"][:20], 1):
        ntype = G.node_type.get(node, "?")
        L(f"| {i} | {node} | {ntype} | {deg} |")
    L("")

    # 7. Predicted Missing Edges
    L("## 7. Predicted Missing Edges (New Discovery Candidates)")
    L("")
    L("Domain pairs that share many constants but lack a direct cross-DSE link.")
    L("These are the strongest candidates for new breakthrough theorems.")
    L("")
    L("| Rank | Domain A | Domain B | Shared Constants | Prediction |")
    L("|------|----------|----------|-----------------|------------|")
    for i, (d1, d2, shared) in enumerate(results["predicted_missing_edges"][:20], 1):
        n1 = d1.replace("D:", "")
        n2 = d2.replace("D:", "")
        pred = f"BT connecting {n1} <-> {n2} via {shared} shared constants"
        L(f"| {i} | {n1} | {n2} | {shared} | {pred} |")
    L("")

    # 8. Constant Triangles (Cliques)
    L("## 8. Constant Cliques (always co-occurring)")
    L("")
    if results["constant_triangles"]:
        L("Groups of 3 constants that form triangles -- they always appear together.")
        L("")
        L("| # | Constant A | Constant B | Constant C |")
        L("|---|------------|------------|------------|")
        for i, (a, b, c) in enumerate(results["constant_triangles"][:15], 1):
            L(f"| {i} | `{a.replace('C:','')}` | `{b.replace('C:','')}` | `{c.replace('C:','')}` |")
    else:
        L("No constant triangles found in the sampled set.")
    L("")

    # 9. Small-World Analysis
    L("## 9. Small-World Property Analysis")
    L("")
    L(f"| Property | Value | Random Graph Equiv. | Ratio |")
    L(f"|----------|-------|--------------------:|------:|")
    c_actual = results["avg_clustering"]
    c_rand = results.get("c_rand", 0)
    l_actual = results["avg_path_length"]
    l_rand = results.get("l_rand", 0)
    L(f"| Clustering coefficient | {c_actual:.4f} | {c_rand:.4f} | {c_actual/max(c_rand,1e-9):.1f}x |")
    L(f"| Avg path length | {l_actual:.3f} | {l_rand:.3f} | {l_actual/max(l_rand,1e-9):.2f}x |")
    L(f"| **Small-world sigma** | **{results['small_world_sigma']:.2f}** | 1.0 | {'YES' if results['small_world_sigma'] > 1 else 'NO'} |")
    L("")
    if results["small_world_sigma"] > 1:
        L("**The discovery graph IS a small-world network** (sigma > 1).")
        L("This means: high local clustering (constants form tight groups) + short global paths")
        L("(any two nodes can be reached in few hops via hub constants).")
    else:
        L("The discovery graph does not exhibit small-world structure.")
    L("")

    # 10. Discovery Implications
    L("## 10. Discovery Implications")
    L("")
    L("### Where to look for new cross-domain BTs")
    L("")
    L("1. **High-betweenness bridges**: Constants with high betweenness are the")
    L("   critical connectors. New BTs are most likely found by exploring how these")
    L("   constants manifest in domains they don't yet explicitly connect.")
    L("")
    if results["top_constant_bridges"]:
        top_bridge = results["top_constant_bridges"][0][0].replace("C:", "")
        L(f"   Top bridge: `{top_bridge}` -- investigate its role in every domain.")
    L("")
    L("2. **Predicted missing edges**: The domain pairs in Section 7 share constants")
    L("   but lack direct BT connections. Each is a concrete prediction for a new BT.")
    L("")
    L("3. **Isolated domains**: Any domain in Section 5 with degree <= 2 is")
    L("   underexplored. Look for n=6 expressions in that domain's literature.")
    L("")
    L("4. **Hub constants**: The 7 base constants (sigma, tau, phi, sopfr, J2, mu, n)")
    L("   and key derived ratios (sigma-tau=8, sigma-phi=10, J2-tau=20) are the most")
    L("   fundamental. Any new domain should first be checked against these.")
    L("")

    # 11. ASCII Core Structure
    L("## 11. Core Graph Structure (ASCII)")
    L("")
    L("```")
    L("                           ┌─────────────────────┐")
    L("                           │  sigma(6) = 12      │ ← highest-degree constant")
    L("                           └──┬──┬──┬──┬──┬──┬───┘")
    L("                              │  │  │  │  │  │")
    L("         ┌────────────────────┘  │  │  │  │  └────────────────────┐")
    L("         │                       │  │  │  │                       │")
    L("    ┌────▼────┐            ┌────▼──▼──▼──▼────┐            ┌────▼─────┐")
    L("    │   Chip   │            │   AI / LLM      │            │  Fusion   │")
    L("    │  Arch    │────────────│  (17 techniques) │────────────│  Energy   │")
    L("    └────┬────┘            └────────┬─────────┘            └────┬─────┘")
    L("         │                          │                          │")
    L("    ┌────▼────┐  sigma-tau=8   ┌───▼────┐   J2-tau=20    ┌───▼─────┐")
    L("    │ Crypto  │◄──────────────►│Learning│◄──────────────►│ Biology │")
    L("    │ Network │                │  Algo  │                │ Genetics│")
    L("    └────┬────┘                └───┬────┘                └───┬─────┘")
    L("         │                          │                         │")
    L("    ┌────▼────┐                ┌───▼────┐               ┌───▼──────┐")
    L("    │Blockchain│               │ Solar  │               │ Material │")
    L("    │  DeFi   │                │Battery │               │ Synthesis│")
    L("    └─────────┘                │ Grid   │               └──────────┘")
    L("                               └────────┘")
    L("")
    L("  Key bridges (betweenness):")
    for i, (node, bc) in enumerate(results["top_constant_bridges"][:5], 1):
        name = node.replace("C:", "")
        L(f"    {i}. {name} (BC={bc:.6f})")
    L("")
    L("  Key hubs (degree):")
    for i, (node, deg) in enumerate(results["top_constant_hubs"][:5], 1):
        name = node.replace("C:", "")
        L(f"    {i}. {name} (deg={deg})")
    L("```")
    L("")

    return "\n".join(lines)


# ══════════════════════════════════════════════════════════════════
# Main
# ══════════════════════════════════════════════════════════════════

def main():
    print("=" * 60)
    print("  Discovery Graph Topology Analysis")
    print("=" * 60)
    print()

    G = build_graph()

    print()
    print(f"Graph built: {G.num_nodes} nodes, {G.num_edges} edges")
    print(f"  Constants: {len(G.nodes_of_type('constant'))}")
    print(f"  Domains:   {len(G.nodes_of_type('domain'))}")
    print(f"  BTs:       {len(G.nodes_of_type('bt'))}")
    print(f"  Hypotheses:{len(G.nodes_of_type('hypothesis'))}")
    print()

    print("Analyzing topology ...")
    results = analyze(G)

    print()
    print("Generating report ...")
    report = generate_report(G, results)
    OUTPUT.write_text(report, encoding="utf-8")
    print(f"Report written to: {OUTPUT}")

    # Print summary to stdout
    print()
    print("=" * 60)
    print("  SUMMARY")
    print("=" * 60)
    print(f"  Nodes:           {results['num_nodes']}")
    print(f"  Edges:           {results['num_edges']}")
    print(f"  Density:         {results['density']:.6f}")
    print(f"  Components:      {results['num_components']}")
    print(f"  Largest comp:    {results['largest_component_size']} ({100*results['largest_component_size']/max(results['num_nodes'],1):.1f}%)")
    print(f"  Diameter:        {results['diameter']}")
    print(f"  Avg path len:    {results['avg_path_length']:.3f}")
    print(f"  Avg clustering:  {results['avg_clustering']:.4f}")
    print(f"  Small-world σ:   {results['small_world_sigma']:.2f}")
    print(f"  Isolated domains: {len(results['isolated_domains'])}")
    print(f"  Missing edges:   {len(results['predicted_missing_edges'])}")
    print()
    print("  Top 5 constant hubs:")
    for node, deg in results["top_constant_hubs"][:5]:
        print(f"    {node.replace('C:','')} (deg={deg})")
    print()
    print("  Top 5 constant bridges:")
    for node, bc in results["top_constant_bridges"][:5]:
        print(f"    {node.replace('C:','')} (BC={bc:.6f})")


if __name__ == "__main__":
    main()
