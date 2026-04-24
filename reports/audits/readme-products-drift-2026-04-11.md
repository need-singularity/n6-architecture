# README <-> products.json drift audit

- Date: 2026-04-11
- Targets:
 - SSOT: `$NEXUS/shared/n6/docs/products.json` (154,868 byte, 4,865 , 34 , 173 )
 - : `$N6_ARCH/README.md` (850 , AUTO )
- Author: Claude (GO , )
- Write scope: this report file only (README / products.json edits prohibited, R18 minimal)
- Background: `sync_products_readme.hexa` is a STUB, so products.json -> README auto-sync does not run. The two files have drifted in both structure and content due to months of manual accumulation.

---

## 1. Drift summary

### 1.1 Section-level drift

| Category | products.json | README | Diff | |------|---:|---:|---:| | Total sections | 34 | 29 | -5 | | Common sections | — | — | **21** | | Only in products.json (missing) | **13** | 0 | +13 | | Only in README (orphan) | 0 | **8** | +8 | | products.json total product count | **173** | — | — | | Sum product count of 13 missing sections | **27** | 0 | +27 | | README product count in 8 orphan sections | — | **34** | — |

### 1.2 Within-common-section product-count drift (products.json measurement basis)

| Section id | products.json | README | Diff | Note | |---------|---:|---:|---:|------| | energy | 5 | 4 | **+1** | `HEXA-AUTO ` | | audio | 7 | 4 | **+3** | `HEXA-BONE` / `HEXA-EAR-CELL` / `HEXA-SPEAKER` | | Other 19 common sections | — | — | 0 | Counts match (content match out of scope) |

> Note: This audit verifies only section/product counts + id existence. Drift of product-internal text requires a separate audit.

### 1.3 13 missing section list (only in products.json)

| id | heading | product count | alien_index | bt_exact_pct | ceiling | |----|---------|---:|---:|---:|:---:| | virology | Virology | 4 | 10 | 100 | O | | hiv-treatment | HIV Treatment | 1 | 10 | 100 | O | | natural-science | Natural Science | 4 | 10 | 95 | O | | cognitive-social | Cognitive & Social | 6 | 10 | 95 | O | | mobility | Mobility & Transport | 2 | 10 | 90 | O | | digital-medical | Digital & Medical Device | 3 | 10 | 92 | O | | tattoo-removal | Tattoo Removal | 1 | 10 | 100 | O | | keyboard | Keyboard | 1 | 10 | 97 | O | | mouse | Mouse | 1 | 10 | 100 | O | | manufacturing-quality | Manufacturing Quality | 1 | 10 | 100 | O | | network | Network | 1 | 10 | 98 | O | | quantum-computer | Quantum Computer | 1 | 10 | 83 | X | | horology | Horology | 1 | 10 | 100 | O | | **Total** | — | **27** | — | — | — |

### 1.4 8 orphan section list (only in README)

| README id | Product rows | BT range (README-declared) | BT range exists in products.json? | |-----------|---:|-----------------------|:-:| | computer | 4 | BT-49/1115~1128 | (keyboard/mouse/quantum-computer 3) | | millennium | 7 | BT-541~547 | **X** (products.json BT-541~547 0) | | dimension | 7 | BT-588~597 | **X** (products.json BT-588~597 0) | | music | 4 | BT-598~607 | **X** (products.json BT-598~607 0) | | linguistics | 4 | BT-608~617 | **X** (products.json BT-608~617 0) | | crypto | 4 | BT-618~627 | **X** (products.json BT-618~627 0) | | astronomy | 4 | BT-628~637 | **X** (products.json BT-628~637 0) | | fantasy | 0 ( ) | — | X ( `fantasy` products.json 0) |

---

## 2. README 8 

 " " — README .

### 2.1 `computer` (README 589~604 , 4 )

- ****: README 4 
- **products.json **: 4 3 ** **
 - ` n=6 ` → products.json `keyboard` ( , BT-1125~1128)
 - `HEXA-MOUSE n=6 ` → products.json `mouse` ( , BT-1115~1124)
 - ` n=6 ` → products.json `quantum-computer` ( HEXA-QUANTUM, BT-49)
 - `HEXA-BCI - ` → **products.json ** (BCI/brain-computer 0 grep )
- ** **:
 1. README `computer` "# 💻 (Computer)" 
 2. 3 (keyboard/mouse/quantum-computer) §3 
 3. **`HEXA-BCI` products.json → ** ( : `digital-medical` `bci` ). , .

### 2.2 `millennium` (README 664~682 , 7 )

- ****: README 7 (/P vs NP/-/NS//BSD/) , BT-541~547
- **products.json **: products.json `millennium` BT-541~547 ** 0** (grep )
- ****: SSOT — " products.json " 
- ** **: products.json `millennium` ( ). README .

### 2.3 `dimension` (README 686~704 , 7 )

- ****: BT-588~597 10/10 EXACT 
- **products.json **: BT-588~597 **0**. `hexa-holo`/`display-8stack`/`consciousness-chip` , " " 7 (24 ) 0.
- ** **: + products.json ( )

### 2.4 `music` (README 708~723 , 4 )

- ****: BT-598~607 10/10 EXACT (12 , 6, 24, )
- **products.json **: 0
- ** **: + products.json ( )

### 2.5 `linguistics` (README 727~742 , 4 )

- ****: BT-608~617 10/10 EXACT (///)
- **products.json **: `cognitive-social` `HEXA-LING n=6 ` 1 , BT (BT-33/48/73/108) README linguistics BT-608~617 **** — .
- ** **: + products.json ( ). (BT-33/48/73/108 vs BT-608~617) .

### 2.6 `crypto` (README 746~761 , 4 )

- ****: BT-618~627 10/10 EXACT (AES/RSA+SHA/+ECC/CIA+PQC)
- **products.json **: BT-618~627 0. `software-crypto` software , .
- ** **: + products.json ( )

### 2.7 `astronomy` (README 765~780 , 4 )

- ****: BT-628~637 10/10 EXACT (ΛCDM/BBN/+/+BAO)
- **products.json **: BT-628~637 0. `particle-cosmology` physics , `space-systems` aerospace BT — README astronomy 10/10 EXACT .
- ** **: + products.json ( )

### 2.8 `fantasy` (README 801~808 , 0 )

- ****: . 60 / 16 EXACT / BT 16 .
- **products.json **: `fantasy` 0. `civilization` `/ n=6 ` (BT-370) " " ( ).
- ** **: ( `[!WARNING]` " " ). README , products.json .

### 2.9 

| | README | |------|---:| | 8 | **34 ** | | products.json (computer 3) | 3 | | products.json () | **30 ** | | (fantasy ) | 0 ( ) | | ** MISS**: HEXA-BCI | **1 ** (products.json ) |

---

## 3. products.json 13 

 . README (: `# {icon} {heading}`, AUTO:SUMMARY , , AUTO:FOOTER ) .

> SUMMARY products.json `summary` `alien_index` / `ceiling` / `bt_exact_pct` / . `sync_products_readme.hexa` .

### 3.1 virology (4 , : tech-industry )

```markdown
---

# 🦠 Virology

<!-- AUTO:SUMMARY_virology:START -->
> **🛸10** | ✅ | BT 4 100%EXACT | //·· + Mk.I~V 
<!-- AUTO:SUMMARY_virology:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v1 | ** - n=6 ** | BT-351: pentamer σ=12, T-number {1,3,4,7}={μ,n/φ,τ,σ-sopfr}, Baltimore σ-sopfr=7, τ=4, 11/11 EXACT | [](docs/virology/goal.md) · [](docs/paper/n6-virology-paper.md) | | 10 | ✅ | v1 | ** - n=6 ** | BT-352: σ-τ=8, σ-μ=11/σ=12, σ-φ=10, HIV n/φ+φ+τ=9, CoV NSP φ^τ=16, 15/15 EXACT | [](docs/virology/goal.md) | | 10 | ✅ | v1 | ** -- n=6 ** | BT-353: n=6, mRNA sopfr=5, LNP τ=4, RdRp σ-sopfr=7, V-E+F=φ=2, 13/13 EXACT | [](docs/virology/goal.md) | | 10 | ✅ | v1 | ** Mk.I~V** | 5 : Mk.I () → Mk.II (2035) → Mk.III (2050) → Mk.IV (2070) → Mk.V () | [Mk.I](docs/virology/evolution/mk-1-current.md) |

<!-- AUTO:FOOTER_virology:START -->
> : [virology/](docs/virology/) · BT: 351~353 · --- 4 n=6 
<!-- AUTO:FOOTER_virology:END -->
```

### 3.2 hiv-treatment (1 , : virology )

```markdown
---

# 🧬 HIV Treatment

<!-- AUTO:SUMMARY_hiv-treatment:START -->
> **🛸10** | ✅ | BT 1 100%EXACT | HAART 6 + CCR5 CRISPR + bNAb 6 + 6 | Mk.I~V
<!-- AUTO:SUMMARY_hiv-treatment:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v1 | **HEXA-HIV 6 ** | BT-461~470: HAART 6 (RT/PR/IN///), CCR5 CRISPR , bNAb 6, 6 | [](docs/paper/n6-hiv-paper.md) · [Mk.I](docs/hiv-treatment/evolution/mk-1-basic.md) · [Mk.V](docs/hiv-treatment/evolution/mk-5-ultimate.md) |

<!-- AUTO:FOOTER_hiv-treatment:START -->
> : [hiv-treatment/](docs/hiv-treatment/) · BT: 461~470 · 6 - 
<!-- AUTO:FOOTER_hiv-treatment:END -->
```

### 3.3 natural-science (4 , : hiv-treatment )

```markdown
---

# 🌿 Natural Science

<!-- AUTO:SUMMARY_natural-science:START -->
> **🛸10** | ✅ | BT 4 95%EXACT | BIO/AGRI/FOOD/OCEAN 4 n=6 --- 
<!-- AUTO:SUMMARY_natural-science:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v1 | **HEXA-BIO n=6 ** | DNA 4=τ, 3=n/φ, 64=2^n, 20=J₂-τ, C₆H₁₂O₆=100% n=6, n=6 (BT-27/51/101/103/104/122) | [](docs/biology/goal.md) | | 10 | ✅ | v1 | **HEXA-AGRI n=6 ** | 6CO₂+12H₂O→C₆H₁₂O₆+6O₂ 100% n=6, 60t/ha=σ·sopfr, σ=12, IPM n/φ=3, 4=τ (BT-101/103/118/120/122) | [](docs/agriculture/goal.md) | | 10 | ✅ | v1 | **HEXA-FOOD n=6 ** | 6 =n, C₆H₁₂O₆, 20 =J₂-τ, HACCP 7=σ-sopfr, τ=4, n=6 (BT-27/51/101/103/118/120) | [](docs/food-science/goal.md) | | 10 | ✅ | v1 | **HEXA-OCEAN n=6 ** | 6 =n, 5=sopfr, Beaufort 12=σ, 6=n(Hexacorallia), pH 8=σ-τ, σ=12 (BT-30/62/118/119/122) | [](docs/oceanography/goal.md) |

<!-- AUTO:FOOTER_natural-science:START -->
> : [biology/](docs/biology/) · [agriculture/](docs/agriculture/) · [food-science/](docs/food-science/) · [oceanography/](docs/oceanography/)
<!-- AUTO:FOOTER_natural-science:END -->
```

### 3.4 cognitive-social (6 , : natural-science )

```markdown
---

# 🧠 Cognitive & Social

<!-- AUTO:SUMMARY_cognitive-social:START -->
> **🛸10** | ✅ | BT 6 95%EXACT | COGNI/CONSCIOUSNESS/SOCIAL/TEMPORAL/LING/ECON 6 n=6 
<!-- AUTO:SUMMARY_cognitive-social:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v1 | **HEXA-COGNI n=6 ** | 6=n(BT-210), =n(BT-211), τ±μ=3~5(BT-219), - τ=4(BT-222), -- (BT-225), Dunbar σ²+n=150 | [](docs/cognitive-architecture/goal.md) | | 10 | ✅ | v1 | **HEXA-CONSCIOUSNESS ** | ANIMA-6 3 , 192=σ·φ^τ, 288GB HBM4=σ·J₂, TCU R(6)=1.0 , σ²=144 , J₂=24 , {1/2,1/3,1/6} , 81 n=6 | [](docs/paper/n6-consciousness-chip-paper.md) | | 10 | ✅ | v1 | **HEXA-SOCIAL n=6 ** | 6 =n(Milgram), Dunbar σ²+n=150, σ=12, n/φ=3, 6=n(Haidt), Kohlberg 6, Christaller (BT-214/215/220/223/225) | [](docs/social-architecture/goal.md) | | 10 | ✅ | v1 | **HEXA-TEMPORAL n=6 ** | 12=σ, 24=J₂, 4=τ, 60=σ·sopfr, 12=σ, 2=φ, 100% EXACT, ~85% EXACT (BT-48/62/225) | [](docs/temporal-architecture/goal.md) | | 10 | ✅ | v1 | **HEXA-LING n=6 ** | 6 =3!=n, Zipf α=R(6)=1, Chomsky 4=τ, 6=n, φ=2 (/), n/φ=3 , BLEU σ·τ=48 (BT-33/48/73/108) | [](docs/linguistics/goal.md) | | 10 | ✅ | v1 | **HEXA-ECON n=6 ** | φ=2, n/φ=3, 4=τ, Porter 5 Forces=sopfr, G6=n, 24h =J₂, Basel n/φ=3 pillar, DJIA σ=12 (BT-53/62/74/113/114) | [](docs/economics/goal.md) |

<!-- AUTO:FOOTER_cognitive-social:START -->
> : [cognitive-architecture/](docs/cognitive-architecture/) · [social-architecture/](docs/social-architecture/) · [temporal-architecture/](docs/temporal-architecture/) · [linguistics/](docs/linguistics/) · [economics/](docs/economics/)
<!-- AUTO:FOOTER_cognitive-social:END -->
```

### 3.5 mobility (2 , : cognitive-social )

```markdown
---

# 🚗 Mobility & Transport

<!-- AUTO:SUMMARY_mobility:START -->
> **🛸10** | ✅ | BT 2 90%EXACT | DRIVE + WING 2
<!-- AUTO:SUMMARY_mobility:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v1 | **HEXA-DRIVE n=6 ** | SAE L0-L5=n=6 EXACT, BEV Fusion n=6 sensor, σ-τ=8 cam, LiDAR σ=12, MPC+PID n/φ=3, ViT d=2^σ=4096, σ=12 fleet/zone (BT-327/328+BT-56/58/61/66/69/84) | [](docs/autonomous-driving/goal.md) | | 10 | ✅ | v1 | **HEXA-WING n=6 ** | SE(3) 6-DOF=n, 12km=σ, ICAO 6=n, τ=4 , CFRP Z=6, Ti-6Al-4V, eVTOL τ=4 quad, ILS n/φ=3 cat (BT-85/93/119/123/125/127) | [](docs/aviation/goal.md) |

<!-- AUTO:FOOTER_mobility:START -->
> : [autonomous-driving/](docs/autonomous-driving/) · [aviation/](docs/aviation/)
<!-- AUTO:FOOTER_mobility:END -->
```

### 3.6 digital-medical (3 , : mobility )

```markdown
---

# 🏥 Digital & Medical Device

<!-- AUTO:SUMMARY_digital-medical:START -->
> **🛸10** | ✅ | BT 3 92%EXACT | BROWSER/MED/AESTHETIC 3 | Mk.I~V
<!-- AUTO:SUMMARY_digital-medical:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 7 | ✅ | v1 | **HEXA-BROWSER ** | 124/134 EXACT(92.5%), 10 × 10+ , DSE 4,500 , σ-φ=10 , φ=2, n/φ=3 (BT-48/113/115/116/140/162/180/211/329/348) | [](docs/browser/goal.md) | | 10 | ✅ | v1 | **HEXA-MED n=6 ** | ECG σ=12 (6+6), MRI σ=12 , n=6 MHz, US τ=4 , SE(3) 6-DOF , Mk.I~V (BT-48/51/56/58/64/66/85/90/113/118/123/300) | [](docs/medical-device/goal.md) | | 10 | ✅ | v1 | **HEXA-AESTHETIC n=6 ** | Fitzpatrick n=6 , D=67nm, FDA J₂-τ=20U/J₂=24U, σ·φ+τ=28, 36 33 EXACT(91.7%), 6 , 10 cross-domain (BT-1129~1134) | [](docs/cosmetic-surgery/goal.md) |

<!-- AUTO:FOOTER_digital-medical:START -->
> : [browser/](docs/browser/) · [medical-device/](docs/medical-device/) · [cosmetic-surgery/](docs/cosmetic-surgery/)
<!-- AUTO:FOOTER_digital-medical:END -->
```

> : `HEXA-BROWSER` products.json `ufo=7` ( 2 ufo=10). `alien_index=10` .

### 3.7 tattoo-removal (1 , : hygiene )

```markdown
---

# 🔬 Tattoo Removal

<!-- AUTO:SUMMARY_tattoo-removal:START -->
> **🛸10** | ✅ | BT 1 100%EXACT | 6 + + TP3
<!-- AUTO:SUMMARY_tattoo-removal:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v2 | ** n=6 ** | BT-1159/1160: 36/36 EXACT (100%) — Fitzpatrick6=n, R20 4=τ, 3=n/φ, 48h=τ·τ/2, sopfr=5 IARC , J₂=24 , TP3 (95%/<1%/3) | [](docs/tattoo-removal/goal.md) |

<!-- AUTO:FOOTER_tattoo-removal:START -->
> : [tattoo-removal/](docs/tattoo-removal/) · BT: 1159~1160
<!-- AUTO:FOOTER_tattoo-removal:END -->
```

> : README `frontier` ` n=6 ` 1 (README 497 ). SSOT → frontier ( ).

### 3.8 keyboard (1 , : marketing computer )

```markdown
---

# ⌨️ Keyboard

<!-- AUTO:SUMMARY_keyboard:START -->
> **🛸10** | ✅ | BT 1 97%EXACT | C(n,2) + USB 6KRO 
<!-- AUTO:SUMMARY_keyboard:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v1 | ** n=6 ** | BT-1125~1128: 30/31 EXACT — 104/87/84/68/61/60/48/17 C(n,2) , USB 6KRO/8/12Mbps, 4mm(τ)/2mm(φ)/5ms(sopfr) | [](docs/keyboard/goal.md) |

<!-- AUTO:FOOTER_keyboard:START -->
> : [keyboard/](docs/keyboard/) · BT: 1125~1128
<!-- AUTO:FOOTER_keyboard:END -->
```

### 3.9 mouse (1 , : keyboard )

```markdown
---

# 🖱️ Mouse

<!-- AUTO:SUMMARY_mouse:START -->
> **🛸10** | ✅ | BT 1 100%EXACT | PS/2 6 + 8kHz + J₂=24
<!-- AUTO:SUMMARY_mouse:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v1 | **HEXA-MOUSE n=6 ** | BT-1115~1124: 25/25 EXACT — PS/2 n=6, sopfr=5 /, n/φ=3 /, σ-τ=8kHz , σ=12 MMO/, J₂=24 | [](docs/mouse/goal.md) |

<!-- AUTO:FOOTER_mouse:START -->
> : [mouse/](docs/mouse/) · BT: 1115~1124
<!-- AUTO:FOOTER_mouse:END -->
```

### 3.10 manufacturing-quality (1 , : mouse )

```markdown
---

# 🏭 Manufacturing Quality

<!-- AUTO:SUMMARY_manufacturing-quality:START -->
> **🛸10** | ✅ | BT 1 100%EXACT | 6 + SPC + Ishikawa + DMAIC + ISO9001 n=6 
<!-- AUTO:SUMMARY_manufacturing-quality:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v2 | **HEXA-QC ** | BT-1161/1162: 36/36 EXACT (100%) — 6=n, Cpk=φ=2, SPC n=6, Ishikawa 6M=n, DMAIC sopfr=5, ISO9001 σ-τ=8, Egyptian 1/2+1/3+1/6=1 | [](docs/manufacturing-quality/goal.md) · [](docs/paper/n6-manufacturing-quality-paper.md) |

<!-- AUTO:FOOTER_manufacturing-quality:START -->
> : [manufacturing-quality/](docs/manufacturing-quality/) · BT: 1161~1162
<!-- AUTO:FOOTER_manufacturing-quality:END -->
```

> : README `tech-industry` ( 581) `HEXA-QC n=6 ` . tech-industry ( ).

### 3.11 network (1 , : manufacturing-quality )

```markdown
---

# 🌐 Network

<!-- AUTO:SUMMARY_network:START -->
> **🛸10** | ✅ | BT 1 98%EXACT | OSI/TCP-IP/Wi-Fi/ n=6 
<!-- AUTO:SUMMARY_network:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v1 | **HEXA-NET ** | 39/40 EXACT (97.5%) — OSI σ-sopfr=7, TCP/IP τ=4 , Wi-Fi n=6, 65536=2^(σ+τ), 8 | [](docs/network/goal.md) |

<!-- AUTO:FOOTER_network:START -->
> : [network/](docs/network/)
<!-- AUTO:FOOTER_network:END -->
```

> : README software ( 340) ` ` 50/50 EXACT . SSOT `network` README software.network-protocol — .

### 3.12 quantum-computer (1 , : network computer )

```markdown
---

# 🔮 Quantum Computer

<!-- AUTO:SUMMARY_quantum-computer:START -->
> **🛸10** | ❌ | BT 1 83%EXACT | NeutralAtom + SurfaceCode + Clifford + kissing number
<!-- AUTO:SUMMARY_quantum-computer:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v1 | **HEXA-QUANTUM ** | 20/24 EXACT — NeutralAtom n=6, SurfaceCode σ=12 data qubit, Clifford τ×n=24 gate, Grover sopfr, kissing number BT-49 | [](docs/quantum-computer/goal.md) |

<!-- AUTO:FOOTER_quantum-computer:START -->
> : [quantum-computer/](docs/quantum-computer/) · BT: 49
<!-- AUTO:FOOTER_quantum-computer:END -->
```

> : products.json `sections[id=quantum-computer].ceiling = false` / `ufo=10, ceiling=true`. - ❌ ✅ — . `products-drift-fix-2026-04-11.md` ( ceiling true , ).

### 3.13 horology (1 , : quantum-computer )

```markdown
---

# ⏱️ Horology

<!-- AUTO:SUMMARY_horology:START -->
> **🛸10** | ✅ | BT 1 100%EXACT | σ=12/J₂=24/σ·sopfr=60/n/φ=3 
<!-- AUTO:SUMMARY_horology:END -->

| 🛸 | | ver | | | |
|:--:|:--:|:---:|---------|------|------| | 10 | ✅ | v1 | ** n=6 ** | 17/17 EXACT (100%) — 12=σ, J₂=24, 60=σ·sopfr, n/φ=3, 2^(sopfr·n/φ)=32768Hz, 6Hz | [](docs/horology/hypotheses.md) |

<!-- AUTO:FOOTER_horology:START -->
> : [horology/](docs/horology/)
<!-- AUTO:FOOTER_horology:END -->
```

> : README `civilization` ( 519) `/ n=6 ` 1 . SSOT → civilization ( ).

---

## 4. 

products.json (line 1 ~ 4865) , README 13 :

| | id | (README ) | |
|---:|----|-----------------------|------| | 1 | virology | `tech-industry` (L586) | products.json 20 | | 2 | hiv-treatment | `virology` | products.json 21 | | 3 | natural-science | `hiv-treatment` | products.json 22 | | 4 | cognitive-social | `natural-science` | products.json 23 | | 5 | mobility | `cognitive-social` | products.json 24 | | 6 | digital-medical | `mobility` | products.json 25 | | — | marketing | ( , L608) | products.json 26 — | | — | hygiene | ( , L786) | products.json 27 — (marketing ) | | 7 | tattoo-removal | `hygiene` | products.json 28 | | 8 | keyboard | `tattoo-removal` | products.json 29 | | 9 | mouse | `keyboard` | products.json 30 | | 10 | manufacturing-quality | `mouse` | products.json 31 | | 11 | network | `manufacturing-quality` | products.json 32 | | 12 | quantum-computer | `network` | products.json 33 | | 13 | horology | `quantum-computer` | products.json 34 |

### 4.1 ( , TODO)

1. **`computer` **: README L589~604. 3 keyboard/mouse/quantum-computer , **HEXA-BCI 1 ** (products.json ).
2. **frontier ` ` (L497) ** — `tattoo-removal` .
3. **tech-industry `HEXA-QC` (L581) ** — `manufacturing-quality` .
4. **civilization `/` (L519) ** — `horology` .
5. **`energy` `HEXA-AUTO ` 1 ** (products.json 5 , README 4).
6. **`audio` `HEXA-BONE` / `HEXA-EAR-CELL` / `HEXA-SPEAKER` 3 ** (products.json 5~7 , README 4).
7. **`millennium` / `dimension` / `music` / `linguistics` / `crypto` / `astronomy` / `fantasy` **: SSOT products.json , README .
8. **`sync_products_readme.hexa` STUB **: .

---

## 5. ASCII 

### 5.1 (products.json 34 = 100%)

```
products.json 34 
────────────────────────────────────────────────────
 21 ████████████████████████ 61.8%
 13 () ██████████████ 38.2%

README 29 
────────────────────────────────────────────────────
 21 ████████████████████████ 72.4%
 8 () ██████████ 27.6%
```

### 5.2 (products.json 173 = 100%)

```
products.json 173 (34 )
────────────────────────────────────────────────────
 21 146 █████████████████████████████████ 84.4%
 13 27 ██████ 15.6%

 (27 ):
 cognitive-social 6 ████████████████████████
 natural-science 4 ████████████████
 virology 4 ████████████████
 digital-medical 3 ████████████
 mobility 2 ████████
 hiv-treatment 1 ████
 tattoo-removal 1 ████
 keyboard 1 ████
 mouse 1 ████
 manufacturing-quality 1 ████
 network 1 ████
 quantum-computer 1 ████
 horology 1 ████
```

### 5.3 

```
 21 2 drift 
────────────────────────────────────────────────────
energy json=5 readme=4 ▓▓▓▓▓ vs ▓▓▓▓ diff=+1
audio json=7 readme=4 ▓▓▓▓▓▓▓ vs ▓▓▓▓ diff=+3
```

### 5.4 README 

```
 8 34 (1 = ▓)
────────────────────────────────────────────────────
millennium 7 ▓▓▓▓▓▓▓
dimension 7 ▓▓▓▓▓▓▓
computer 4 ▓▓▓▓ (keyboard/mouse/quantum 3 , BCI 1 )
music 4 ▓▓▓▓
linguistics 4 ▓▓▓▓
crypto 4 ▓▓▓▓
astronomy 4 ▓▓▓▓
fantasy 0 . ( , )
```

### 5.5 

```
 (products.json SSOT )
────────────────────────────────────────────────────
 21/34 = 61.8% ████████████▌
 146/173 = 84.4% ████████████████▉
 + 19/34 = 55.9% ███████████▎
 (energy/audio 2 drift )
```

---

## A. 

- SSOT: `$NEXUS/shared/n6/docs/products.json`
- README: `$N6_ARCH/README.md`
- : `$N6_ARCH/reports/audits/products-drift-fix-2026-04-11.md` (products.json `_meta` )
- : `$N6_ARCH/reports/audits/products-backup-2026-04-11.json`
- (STUB): `sync_products_readme.hexa` ( , )

## B. (Honest Limitations)

- ** id / / BT ** 3 . description / ver / drift .
- (§3) SUMMARY products.json `summary` (`products-drift-fix-2026-04-11.md` ), `alien_index` / `ceiling` / `bt_exact_pct` / . `sync_products_readme.hexa` .
- " 8 " README . `docs/` (: `docs/millennium-riemann/goal.md`) .
- `computer` `HEXA-BCI` products.json , ( / / ) .
- README (σ, τ, φ, J₂, sopfr ) products.json description . `sigma`, `tau` ASCII README .
