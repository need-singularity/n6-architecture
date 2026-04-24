# sedi + brainwire domain folder removal audit log

Date: 2026-04-11
Work: complete removal of domains/sedi/ (970 files), domains/brainwire/ (174 files)

## Background
- sedi: originally an independent repo (Search for Extra-Dimensional Intelligence), absorbed into n6-architecture domains/
- brainwire: originally an independent repo (Neural Interface Hardware), absorbed into n6-architecture domains/
- Both domains were already removed from domains/_index.json (not included in the 10-axis classification)
- Only folder and project-wide references remained

## Absorption target confirmation
- sedi content -> distributed absorption into physics/ (cosmology, particle related), cognitive/ (reality-map) etc.
- brainwire content -> absorbed into cognitive/ (brain-computer-interface, hexa-neuro), life/ (neuro)
- domains/_index.json: confirmed already not-included

## Removal items
1. domains/sedi/ (970 files) -- git rm -rf
2. domains/brainwire/ (174 files) -- git rm -rf
3. n6shared/projects.json -- remove sedi, brainwire entries
4. n6shared/readme-data.json -- remove brainwire references
5. nexus/src/gate/source.rs -- remove sedi, brainwire strings
6. nexus/src/growth/lens_grower.rs -- remove sedi_lenses references
7. nexus/src/telescope/registry.rs -- remove sedi comment references
8. nexus/tests/telescope_test.rs -- remove sedi_lenses import/usage
9. engine/CLAUDE.md -- remove sedi references
10. engine/sedi_training_monitor.hexa -- remove sedi stub file
11. README.md -- remove SEDI reference

## Preserved items (not removed)
- theory/breakthroughs/breakthrough-theorems.md: "sedimentary" = geological term, unrelated to sedi domain
- theory/constants/atlas-constants.md: "sediment" = engineering term, unrelated to sedi domain
- techniques/: "H-SEDI-" prefix = technique-specific name, preserved
- n6shared/config/dse-map.toml: "sedi-universe" = DSE domain name, independent reference
- n6shared/dse/dse_cross_results_v2.json: "sedi-universe" = DSE result, independent reference
- nexus/src/telescope/sedi_lenses.rs: legacy Rust (being deprecated; CLAUDE.md explicitly says "no new additions")
- nexus/src/telescope/quantum_lenses.rs: "SEDI" = explanatory text, legacy
- nexus/src/graph/knowledge_nodes.rs: "SEDI Training Monitor" = graph node description
- papers/_registry.json: "BrainWire" = external repo link
- nexus/origins/ready-absorber/: past absorption records, history preserved
- n6shared/logs/absorbed/: past logs, history preserved
- reports/audits/: prior audit logs
- brainwire mentions inside domains/*/goal.md: content of other domain documents (only reference strings cleaned)
