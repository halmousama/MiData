# Oracles — pedigree, birth action & reproduction rules

> **Extracted:** 2026-10-26 · **Method:** `REBUILD_PLAN.md` Appendix D — one owner-authorized session read
> **only** the legacy test files and transcribed test **names + asserted behaviours**. No code, API shape,
> endpoint path, node-key spelling or legacy file name is reproduced. **The firewall exception was used
> and expired with that session** (AGENTS.md rule 5; `docs/gaps.md` P0-11 closed).
>
> **Scenarios:** 60 (pedigree 26 · health/reproduction 34) → **59 kept, 1 skipped placeholder** (PED-23);
> 1 legacy-payload note (PED-14).
> **Inventory ids:** `docs/inventory.md`.
>
> Scenario ids (`PED-nn`) are the stable handle: cite them in an inventory row's `evidence` column.

## Pedigree graph

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| PED-01 | pedigree | `test_empty_herd` | Herd with zero animals → 0 nodes, 0 edges, total 0, no warnings; an empty state, not an error. | PD-1 |
| PED-02 | pedigree | `test_single_orphan` | One animal with no sire and no dam recorded → exactly 1 node, 0 edges; the node is identified by the animal. | PD-1 |
| PED-03 | pedigree | `test_parent_child_link` | One male, one female and a calf whose sire and dam are those two → 3 nodes, 2 edges, one labelled sire and one labelled dam, each directed parent → child. | PD-1, PD-7 |
| PED-04 | pedigree | `test_missing_columns_default_to_none` | Animal whose sex and species are unrecorded → the node is still produced and the absent attributes surface as unknown; no error, no warning. | PD-21, HM-14 |
| PED-05 | pedigree | `test_multiple_children` | One sire with two offspring (one male, one female) → 3 nodes, 2 edges, both labelled sire. | PD-1, PD-8 |
| PED-06 | pedigree | `test_root_id_filters_to_family` | Root animal with two parents, depth 1 → root plus both parents (3 nodes); a third animal sharing those same parents is excluded. | PD-1, PD-2 |
| PED-07 | pedigree | `test_root_with_generations_depth` | Linear chain grandparent → parent → child, traversed from the middle: depth 1 → all 3 nodes (one hop up, one down); depth 2 → the identical node set, no invented nodes. | PD-1, PD-2 |
| PED-08 | pedigree | `test_missing_root_returns_empty` | Root identifier absent from the herd → 0 nodes, 0 edges, plus a "not found" warning. | PD-1 |
| PED-09 | pedigree | `test_self_referential_cycle` | Animal recorded as its own sire → traversal terminates; 1 node and 1 self-loop edge. | PD-3 |
| PED-10 | pedigree | `test_mutual_cycle` | Two animals each recorded as the other's sire → traversal terminates; 2 nodes, 2 edges. | PD-3 |
| PED-11 | pedigree | `test_triangle_cycle_bfs_terminates` | Three animals in a sire cycle, traversed from one of them at the maximum depth (10) → terminates with 3 nodes and 3 edges, no duplication. | PD-3, PD-2 |
| PED-12 | pedigree | `test_phantom_sire_node_created` | Animal whose sire is recorded only as an external tag absent from the herd → 2 nodes (animal + phantom ancestor); the phantom is marked external and labelled with the recorded tag. | PD-4, HM-10, HM-11 |
| PED-13 | pedigree | `test_phantom_dam_node_created` | Animal whose dam is recorded only as an external tag → 2 nodes, one phantom ancestor labelled with the tag. | PD-4, HM-10, HM-11 |
| PED-14 | pedigree | `test_phantom_edge_has_dashed_style` | Animal with an external sire tag → the edge to the phantom parent is visually distinguishable from a herd-parent edge. | PD-4, HM-11 |
| PED-15 | pedigree | `test_shared_phantom_deduplicated` | Two offspring of the same external sire tag → exactly 1 phantom node (deduplicated), 2 edges into it, 3 nodes total. | PD-4 |
| PED-16 | pedigree | `test_female_sire_emits_warning` | Animal whose sire is recorded as Female → the sire edge is flagged as a warning and a warning entry names the sex mismatch; the graph still renders (2 nodes, 1 edge) — a read-time warning, never a block. | PD-6, HM-8 |
| PED-17 | pedigree | `test_male_dam_emits_warning` | Animal whose dam is recorded as Male → the dam edge is flagged and a warning names the mismatch; the graph still renders. | PD-6, HM-9 |
| PED-18 | pedigree | `test_no_warning_on_correct_gender` | Male sire and Female dam → zero warning-flagged edges and zero warnings. | PD-6 |
| PED-19 | pedigree | `test_no_gender_no_warning` | Parent with no recorded sex → no sex warning raised (nothing to validate). | PD-6 |
| PED-20 | pedigree | `test_under_limit_not_truncated` | 10 animals with no parent links → truncation flag false, exactly 10 nodes. | PD-5 |
| PED-21 | pedigree | `test_over_limit_truncated_with_warning` | 510 animals with no parent links → truncation flag true, at most 500 nodes, and a warning naming the 500-node cap. | PD-5 |
| PED-22 | pedigree | `test_root_id_bypasses_truncation` | 510 animals, root-centric query at depth 1 on an animal with no parents → 1 node, truncation flag false (the cap is not applied on this path). | PD-5, PD-1 |
| PED-24 | pedigree | `test_all_nulls_in_parent_fields` | Animal with neither sire nor dam recorded → 1 node, 0 edges. | PD-1 |
| PED-25 | pedigree | `test_empty_string_parent_id` | Parent reference stored as an empty string → treated as absent: no edge and no phantom node created. | HM-10, PD-4 |
| PED-26 | pedigree | `test_only_dam_link` | Animal with a dam and no sire → 2 nodes, exactly 1 edge labelled dam. | PD-1 |

## Birth action, rules & pack manifests

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| PED-27 | health | `test_date_diff_positive` | Two dates nine days apart → 9. | OR-23 |
| PED-28 | health | `test_date_diff_zero` | The same date twice → 0. | OR-23 |
| PED-29 | health | `test_date_diff_negative` | The pair reversed → −9; the difference is signed, direction preserved. | OR-23 |
| PED-30 | health | `test_date_add_positive` | A date plus 10 days → the date ten days later, still ISO text. | OR-23 |
| PED-31 | health | `test_date_add_zero` | A date plus 0 days → unchanged. | OR-23 |
| PED-32 | health | `test_date_add_negative` | A date plus −5 days → five days earlier. | OR-23 |
| PED-33 | health | `test_evaluate_withdrawal_active` | Treatment recorded yesterday with a 5-day withdrawal, evaluated today → the withdrawal condition holds (true). | OR-22, OR-23 |
| PED-34 | health | `test_evaluate_withdrawal_expired` | Treatment recorded 7 days ago with a 3-day withdrawal → the condition is false. | OR-22 |
| PED-35 | health | `test_evaluate_expected_birth_date_passed` | Expected birth date already in the past → true; the animal is overdue. | OR-23, PD-19 |
| PED-36 | health | `test_evaluate_missing_field_safe` | Expression referencing a value the animal does not carry → evaluates false, no crash, no error surfaced. | OR-24 |
| PED-37 | health | `test_evaluate_current_date_injection` | Expression using "today" while the animal stores no such value → the engine supplies today itself. | OR-23 |
| PED-38 | health | `test_evaluate_strict_equality` | Equality comparison between two dates → true when identical, false when one day apart. | OR-23 |
| PED-39 | health | `test_resolves_by_semantic_role` | Fields tagged with breeding purposes (mating date, pregnancy status) plus an untagged numeric field → requesting the two purposes resolves exactly those two; the untagged field is not returned. | OR-1, PD-13 |
| PED-40 | health | `test_inactive_fields_excluded` | A deactivated field carrying a health purpose tag → not resolved (empty result). | OR-2, HM-22 |
| PED-41 | health | `test_empty_roles_returns_empty` | Request for a purpose no field carries → empty result, not an error. | OR-1 |
| PED-42 | health | `test_new_roles_accepted` | A field may carry any of twelve reproduction/health purposes (mating date, pregnancy status, gestation period, expected birth date, birth date, offspring count, calving ease, treatment date, withdrawal period, withdrawal end, next check-up, condition score) → all accepted at save time. | OR-1 |
| PED-43 | health | `test_legacy_roles_still_accepted` | Older purpose values (primary, normal, system) remain valid alongside the new ones. | OR-1 |
| PED-44 | health | `test_invalid_role_rejected` | Unknown purpose name → rejected with a validation error at save time. | OR-4 |
| PED-45 | health | `test_evaluate_withdrawal_alert_triggers` | Animal treated yesterday with a 5-day withdrawal, plus a warning-severity withdrawal rule carrying a message template → the rule fires and the animal shows a warning alert with the rendered message. | OR-22, OR-25, OR-26, HM-19 |
| PED-46 | health | `test_evaluate_withdrawal_expired_no_alert` | Animal treated 10 days ago with a 3-day withdrawal → the rule does not fire, no alert raised. | OR-22 |
| PED-47 | health | `test_birth_creates_offspring_with_parent_links` | Birth action for an existing dam and sire with two offspring (each with sex, birth date, breed, species) plus parturition observations (offspring count 2, calving ease unassisted) → success; both offspring exist with their identity attributes; each carries a dam link and a sire link; the dam's pregnancy status becomes open; offspring count 2 and calving ease are recorded on the dam. | PD-11, PD-12, PD-13, PD-14, PD-15, PD-16, DE-15 |
| PED-48 | health | `test_birth_rejects_invalid_gender` | Birth action naming a sire whose recorded sex is Female → rejected with a message stating a male was expected; no offspring created. | PD-12, HM-8, PD-17 |
| PED-49 | health | `test_birth_rejects_missing_dam` | Birth action naming a dam absent from the herd → rejected as not found; nothing created. | PD-11, PD-17 |
| PED-50 | health | `test_manifest_fields_valid` | Bundled health/reproduction pack → validates; declares 14 fields, 2 modes, 5 widgets and covers all twelve reproduction/health purposes. | EX-1, EX-11 |
| PED-51 | health | `test_install_creates_fields_modes_widgets` | Install the pack onto an empty configuration → its fields, modes and widgets are created and become visible on the data surfaces. | EX-2, EX-5, EX-6 |
| PED-52 | health | `test_install_idempotent` | Install the pack twice → the second install creates 0 fields, reports every existing field as skipped and creates 0 modes; no duplicates. | EX-2, EX-13, OB-2 |
| PED-53 | health | `test_install_records_installed_extension` | Install the pack → the pack is recorded as installed and active. | EX-10, EX-16 |
| PED-54 | health | `test_gestation_seeded_for_known_species` | Install with a cow, a sheep and a goat in the herd → each animal receives its species' gestation default (283 / 147 / 150 days). | EX-7, OR-7 |
| PED-55 | health | `test_gestation_skips_unknown_species` | Animal whose species has no gestation default → no value written. | EX-7 |
| PED-56 | health | `test_gestation_skips_existing_values` | Animal that already carries a gestation value (deliberately wrong, 999) → the existing value is preserved and never overwritten by the seed. | EX-7, DE-11 |
| PED-57 | health | `test_manifest_in_bundle_list` | Bundled pack → discoverable in the pack list by its identifier. | EX-1, EX-11 |
| PED-58 | health | `test_all_widgets_have_sql` | Every widget in the pack declares a read-only query. | EX-6, AI-6, DB-9 |
| PED-59 | health | `test_all_modes_have_fields` | Every mode in the pack references at least one field, and every referenced key is either pack-defined or a core identity attribute. | EX-5, OR-10 |
| PED-60 | health | `test_widget_types_are_valid` | The pack exercises the KPI, list, alert and chart widget types, at least one of each. | DB-1, DB-2, DB-3, DB-5, DB-7, EX-6 |

## Structural oracles (exact shapes to assert)

| shape | asserted outcome |
|---|---|
| empty herd | 0 nodes, 0 edges, 0 total, no warnings |
| single parentless animal | 1 node, 0 edges |
| sire + dam + calf | 3 nodes, 2 edges (1 sire, 1 dam), direction parent → child |
| one sire, two offspring | 3 nodes, 2 edges, both sire-labelled |
| dam only | 2 nodes, 1 edge, dam-labelled |
| root, depth 1, two parents, sibling present | exactly {root, sire, dam}; sibling absent |
| chain of 3, root = middle | depth 1 → 3 nodes; depth 2 → the same 3 |
| unknown root | 0 nodes, 0 edges, ≥1 "not found" warning |
| self-parent | 1 node, 1 self-loop edge, terminates |
| two-node cycle | 2 nodes, 2 edges, terminates |
| three-node cycle at depth 10 | 3 nodes, 3 edges, terminates, no duplicates |
| one external sire tag | 2 nodes (1 external), 1 edge |
| same external sire tag, two offspring | 3 nodes (exactly 1 external), 2 edges |
| female sire / male dam | 1 warning-flagged edge, ≥1 warning naming the mismatch, graph still complete |
| correct or unrecorded sex | 0 warning edges |
| 10 animals | truncation false, 10 nodes |
| 510 animals | truncation true, ≤ 500 nodes, ≥1 warning naming the cap |
| 510 animals, root-centric depth 1 | truncation false, exactly 1 node |
| birth, 2 offspring | success; 2 offspring; each with dam link + sire link + sex; dam: pregnancy status open, offspring count 2, calving ease recorded |
| birth, female sire / unknown dam | rejection with a typed reason; 0 offspring |
| install on empty config | 14 fields, 2 modes, 5 widgets, all readable |
| install twice | run 2: 0 fields created, 14 skipped, 0 modes created |
| gestation seeding | cow 283, sheep 147, goat 150; unknown species → nothing; pre-existing 999 → still 999 |

## Waived / skipped (2)

| id | legacy test | why |
|---|---|---|
| PED-23 | `test_end_to_end` | **Skipped placeholder** in the legacy suite — asserts nothing, transcribes no behaviour |
| note · PED-14 | `test_phantom_edge_has_dashed_style` | The *behaviour* (phantoms are visually distinct) is kept in PD-4/HM-11, but the assertion that the **data payload carries a stroke style** is waived: style belongs to theme tokens in the UI layer |

## Fresh — write from `REBUILD_PLAN` (no legacy coverage)

- **Depth bounds:** depth 0, negative, 11 and non-numeric are never exercised; the default of 3 is never asserted; only one test reaches the ceiling of 10. Need clamp-or-reject oracles (PD-2).
- **Cap determinism:** truncation is asserted only as "≤ 500" with no selection order — which animals survive is unspecified. Need exactly 500 with a defined order, cap × depth interaction, and the cap applied on the root-centric path (PD-5).
- **Phantom depth and chaining:** a phantom whose own parents are unknown, a phantom as graph root, a phantom recurring across generations, the same tag used as sire for one animal and dam for another (PD-4).
- **Cycles through phantoms:** an external tag that resolves back to a herd animal; cycle detection combined with phantom deduplication (PD-3).
- **Status filtering:** Sold or Dead parents — whether they appear and how they are marked.
- **Direction control:** ancestors-only and descendants-only views have no legacy oracle (PD-7, PD-8).
- **Layout maths and canvas:** generation rows, spacing, crossing minimisation, pan/zoom, click-through to the animal record (PD-9, PD-10).
- **Write-time sex validation:** rejecting a female sire or male dam at create/edit — legacy only warns at read time (HM-8, HM-9).
- **Birth atomicity:** no legacy test injects a mid-way failure and proves zero partial state; both rejection tests assert only that an error was raised (PD-17).
- **Birth: duplicate offspring identifier aborts the whole action** (PD-15).
- **Birth payload edges:** zero offspring, one offspring, offspring count disagreeing with the offspring list, missing or future birth date, offspring born before the dam, sire absent, sire given as an external tag.
- **Pregnancy lifecycle:** statuses beyond open (pregnant, aborted, failed insemination), history vs latest value, and the pregnant-animals view (PD-18).
- **Upcoming and overdue expected births** as a view; only a bare expression is checked (PD-19).
- **Reproduction alerts in the alerts centre** (PD-20); severity styling, message rendering with substituted values, herd-wide vs per-animal evaluation (OR-25…OR-27).
- **Date sandbox hardening:** month/year offsets, leap-day boundaries, invalid date text, nested calls, unknown function names, non-date values, and the boundary between "fails to false" and "reports a malformed expression" (OR-23, OR-24).
- **Pack uninstall and retention (EX-9), type conflict and reactivation (EX-3, EX-4), install rollback on failure (EX-8), install state after restart (EX-12).**
- **Gestation defaults as data:** editing a species default, adding a species, pack defaults overriding presets, and reporting which animals were skipped (OR-7, EX-7).
- **Audit:** no legacy oracle for the audit row the birth action or a pack install must write (SY-6, SY-7).

## Anti-patterns — never reproduce

1. **Internal row ids leak into the graph contract:** nodes are addressed by database identifier while parent references are farm tags — two identity spaces in one payload. Keep one stable animal identity; never expose storage ids.
2. **Pedigree links and identity stored as dynamic observations:** sex, species, breed, sire and dam are all "just measurements", so a sire link can be re-recorded with a later date and silently change ancestry. Identity and parent links are columns (D7).
3. **Presentation attributes in the data payload** (a stroke style proving an edge is dashed) — style is a theme token.
4. **Unknown root returns an empty success** plus a warning string the user may never see; return an actionable i18n'd error.
5. **Self-parenting accepted and drawn:** the self-loop renders instead of being rejected at write time; pre-existing bad data becomes a data-quality warning.
6. **Truncation asserted only as an upper bound** with no selection rule — the surviving set is non-deterministic and untestable.
7. **The node cap is not applied to root-centric traversal:** depth alone guards that path, so a wide component can exceed 500 nodes while reporting not truncated.
8. **Missing operand silently becomes false:** an unresolvable placeholder collapses the expression into invalid text and is swallowed as "false", conflating "not applicable / data missing" with "rule passed" — the most dangerous failure mode for a withdrawal or overdue-birth alert. Report "not evaluated" distinctly.
9. **Alert tests never touch the alert surface:** they rebuild the value map inline and assert on the expression evaluator, so a broken alert pipeline still passes.
10. **Transport status codes carry domain meaning** (sex mismatch as 400, unknown dam as 404) with English-only text.
11. **Birth tests do not prove rollback:** rejection is asserted, but not that zero offspring, zero observations and an unchanged pregnancy status remain.
12. **Business vocabulary hardcoded in the action:** the dam's pregnancy status is reset to a literal string inside the action, so the value set lives in code rather than in field configuration.
13. **Closed purpose vocabulary in code:** a fixed enumeration of twelve purposes, one per field, with "normal" meaning no purpose — a field cannot serve two purposes and adding one requires a code change. Purpose tags are data-driven and multi-valued.
14. **Species defaults hardcoded in code and tests:** gestation periods asserted as literals and any other species silently gets nothing. Read them from the species registry (rule 4).
15. **Silent skip with no report:** unknown species, skipped fields and skipped widgets counted only in an internal blob; the farmer is never told what was not applied.
16. **Exact pack totals pinned in tests** ("14 fields / 2 modes / 5 widgets") — assert invariants (every declared field installed, every mode non-empty), not magic numbers.
17. **Packs ship raw query text** with nothing validating it at install time — a third-party pack is an injection surface. Route every widget data source through `midata-queryguard`.
18. **Identity attribute names duplicated as a whitelist inside a test** — schema drift becomes invisible.
19. **"Type supported" proven by "this pack happens to use it"** — the widget-type test passes or fails on pack content, not capability.
20. **A blank parent tag silently normalised to "no parent"** — normalise at write time and report the blank tag.
