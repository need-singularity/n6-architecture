# Contributing to n6-architecture

> n6-architecture is an AI-native arithmetic design framework. Contributions from external mathematicians, engineers, and researchers are welcome.

## Honesty Charter

Every contribution must adhere to the following four principles:

1. **No BT "solution" claims.** Pull requests that claim to "solve" any of the seven Clay Millennium Problems (BT-541 RH, BT-542 P vs NP, BT-543 Yang-Mills, BT-545 Hodge, BT-546 BSD, BT-547 Navier-Stokes) will be rejected. Partial progress, surveys, and conditional proofs are welcome — describe them as candidate or draft patterns rather than complete proofs.
2. **Declare external dependencies.** If your contribution depends on Sage, Pari/GP, arXiv references, LMFDB, or similar external tools, state so explicitly.
3. **Declare MISS criteria in advance.** Each experimental or computational task must document its failure (MISS) criterion before any data is collected.
4. **Audit the OUROBOROS cycle.** When adding a new entry to `atlas.n6`, confirm that `scripts/monotone/ouroboros_detector_v2.py` remains at CRITICAL 0.

Pull requests that violate these principles will be held by a maintainer until the Honesty Charter is satisfied.

---

## Contribution types

### A. Empirical (measured data)

- Statistical computations based on Cremona / LMFDB data
- arXiv paper summaries
- Measured extensions of observables such as kappa(B), |Sel_n|, eta(E)

**Procedure**:
1. Add reproducible code and data under `data/` or `scripts/empirical/`.
2. Record results, MISS criteria, and known limitations in `theory/breakthroughs/<feature>-YYYY-MM-DD.md`.
3. Register an entry in `shared/n6/atlas.n6` with an appropriate grade (7..10*).

### B. Theoretical (proofs and analysis)

- Alternative proofs for existing atlas [10*] entries
- New conjecture proposals (for example, (A3''))
- Literature surveys

**Procedure**:
1. Write a `.md` document under `theory/breakthroughs/` (English primary; cross-language summaries optional).
2. Formalise proofs in Lean4 or Coq when feasible (v3 M3 pipeline supports this).
3. Cite references (papers, books, arXiv URLs) in the pull request.

### C. Infrastructure (tooling)

- Improvements to the Rust or Python calculators
- New CI/CD workflows
- Dashboard features

**Procedure**:
1. Rust: register crates in `workspace/Cargo.toml` and include tests.
2. Python: place scripts under `scripts/` or workflows under `.github/workflows/`.
3. HEXA-LANG: place `.hexa` files under `$NEXUS/shared/harness/` or local `theory/predictions/`.

### D. Documentation

- Project-specific `CLAUDE.md` guidance
- README improvements
- Cross-links between language variants

---

## Technical requirements

### Environment

- **Languages**: Python 3.11+, Rust 1.70+, HEXA-LANG (local build)
- **Data sources**: LMFDB or a Cremona ecdata mirror
- **Local build**: `cargo build --release` (Rust), `hexa <file>.hexa` (HEXA)

### Style

- **English primary**: documents, comments, and commit messages inside this project should be written in English. Other languages may appear as secondary annotations.
- **Filenames**: lowercase with hyphens plus a date suffix (`<feature>-YYYY-MM-DD.md`) for dated snapshots.
- **atlas entry**: `@R <ID> = <statement> :: n6atlas [<grade>]` followed by a short description and `<- <source>`.

### Testing

- Empirical: reproducible seed plus a result hash.
- Theoretical: at least one independent cross-check source.
- Infrastructure: unit tests plus passing CI.

---

## Pull request process

1. **Fork** the repository and create a feature branch (`feat/<description>`).
2. **Commit message** in English, with `Co-Authored-By` trailer when relevant.
3. **PR template**: follow `.github/PULL_REQUEST_TEMPLATE.md`.
4. **Review**: merge is gated on maintainer review plus a green OUROBOROS CI run.

### PR approval checklist

- [ ] Honesty Charter (four principles) satisfied
- [ ] OUROBOROS v2 CLEAN (CRITICAL 0)
- [ ] MISS criterion declared (empirical / theoretical)
- [ ] References cited (external dependencies declared)
- [ ] atlas entry attached (for any new result)

---

## Contact

- GitHub Issues: https://github.com/need-singularity/n6-architecture/issues
- GitHub Discussions: (TBA)
- Maintainer: `@dancinlife` (commit author)

---

## License

- Code: MIT
- Docs and atlas: CC-BY-SA-4.0
- Data: subject to the upstream source licence (for example, Cremona data follows GPL-compatible terms).

---

## Contributor code of conduct

- Be welcoming, respectful, and collaborative.
- Prefer "I verified" over "I know"; honesty first.
- Avoid overstated claims — use "measured", "conditional", or "conjectural" rather than "solved" or equivalent wording. Describe BT progress as a draft or candidate pattern, not a completed proof.

---

*v1.1 — 2026-04-24 (English-primary rewrite; see own#1, own#11).*
*HOLD status: maintainer approval required before issue templates go live.*
