# Oracles — storage bootstrap, configuration & computed formulas

> **Extracted:** 2026-10-26 · **Method:** `REBUILD_PLAN.md` Appendix D — one owner-authorized session read
> **only** the legacy test files and transcribed test **names + asserted behaviours**. No code, API shape,
> endpoint path, table/column name or legacy file name is reproduced. **The firewall exception was used
> and expired with that session** (AGENTS.md rule 5; `docs/gaps.md` P0-11 closed).
>
> **Scenarios:** 42 (database 8 · settings 16 · formula 18) → **39 kept, 3 waived**.
> **Inventory ids:** `docs/inventory.md`. Where no row existed, a new row was added and its id is used here.
>
> Scenario ids (`STO-nn`) are the stable handle: cite them in an inventory row's `evidence` column and in
> the test's doc comment. The `our test` name is filled in at implementation time.

## Scenarios

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| STO-01 | database | `test_engine_creates_tables` | Empty data file opened for the first time → the complete schema is created without error and without a manual setup step. | SY-13 |
| STO-02 | database | `test_engine_sqlite_driver` | Storage backend in use → exactly one embedded file-based database; no second engine, no server process. | SY-14 |
| STO-05 | database | `test_session_read_write` | Create an animal with farm identifier `TEST-001` in an empty herd → stored and read back unchanged, with a stable internal id assigned. | HM-1 |
| STO-06 | database | `test_wal_mode_can_be_enabled` | A file-based data file is opened → journal mode is write-ahead logging; a memory-only database can never report WAL. | SY-15 |
| STO-07 | database | `test_database_works_in_wal_mode` | With WAL active, create a text field definition and read it back → the write succeeds and the stored value matches exactly. | OR-1 |
| STO-09 | settings | `test_deactivate_field_soft_deletes` | Deactivate a field that already holds 2 stored values → the field record survives marked inactive and both stored values are retained untouched. | OR-2, HM-22 |
| STO-10 | settings | `test_deactivate_field_preserves_mode_links` | Deactivate a field linked to a custom mode → the mode↔field association survives, so reactivation restores the workspace unchanged. | OR-2, OR-10, EX-4 |
| STO-11 | settings | `test_deactivate_primary_field_blocked` | Deactivate the field carrying the animal's primary identifier → refused with an actionable error naming the protected role; the field stays active. | OR-2, OR-32 |
| STO-12 | settings | `test_deactivate_system_field_blocked` | Deactivate a field carrying the protected system role → refused with an actionable error naming the role; the field stays active and nothing is written. | OR-2, OR-31 |
| STO-13 | settings | `test_deactivate_nonexistent_field_returns_404` | Deactivate an unknown field id → a not-found error, no mutation, no crash. | OR-2 |
| STO-14 | settings | `test_delete_custom_mode_preserves_links` | Soft-delete a custom mode holding 2 linked fields → the mode survives marked inactive and both field links are retained. | OR-9, OR-10, EX-9 |
| STO-15 | settings | `test_delete_system_mode_blocked` | Delete the protected system mode → refused with a clear "cannot delete system modes" error; the mode is still present afterwards. | OR-11 |
| STO-16 | settings | `test_delete_nonexistent_mode_raises` | Delete an unknown mode id → a not-found error, nothing mutated. | OR-9 |
| STO-17 | settings | `test_create_mode` | Create a custom mode with name, icon and description → created with those exact values, marked non-system, and assigned an id. | OR-9 |
| STO-18 | settings | `test_create_duplicate_name_blocked` | Create a mode reusing an existing mode name → refused as already existing; no second mode is created. | OR-9 |
| STO-19 | settings | `test_update_mode_metadata` | Update a custom mode's name, icon and description in one call → all three changes persist. | OR-9 |
| STO-20 | settings | `test_update_system_mode_blocked` | Rename the system mode → refused with a "cannot modify system modes" error; the mode is unchanged. | OR-11 |
| STO-21 | settings | `test_update_mode_fields_transactional` | Replace a mode's field selection, empty → {weight, height} → {milk yield} → after each call exactly the requested set is linked (2, then 1); earlier links are replaced, never accumulated. | OR-10 |
| STO-22 | settings | `test_update_mode_fields_invalid_field_id_raises` | Replace a mode's field selection with a set containing one unknown field → the whole request is rejected and no partial link set is written. | OR-10, OR-4 |
| STO-23 | settings | `test_get_modes_for_field` | Ask which modes reference a given field → the custom mode linking it is returned (reverse lookup used to warn before deactivation). | OR-10, OR-2 |
| STO-24 | settings | `test_verify_in_memory_db` | Integrity check on a freshly created database → exactly one result line, reported ok. | IX-11 |
| STO-25 | formula | `test_returns_data_for_animals_with_all_fields` | Herd-wide formula `weight × 2` where every animal has a weight → every animal receives a non-null result; none is silently dropped. | OR-17, OR-14 |
| STO-26 | formula | `test_missing_field_data_returns_none` | Herd-wide formula over a field only one animal carries (30) → that animal gets 30.0; the animal with no value for the field is absent from the results. | OR-17 |
| STO-27 | formula | `test_query_uses_entity_id_column` | Herd-wide calculation → every result is attributed to a real animal, with no null or unattributed entries (guards against a join losing the animal key). | OR-17 |
| STO-28 | formula | `test_fetch_with_specific_animal_ids` | Calculate for a subset of exactly one animal → only that animal appears in the results; no other animal leaks in. | OR-17 |
| STO-29 | formula | `test_basic_arithmetic` | Expression evaluation → `2+3 = 5`; `10/4 = 2.5` (true division, not integer truncation); `(2+3)*4 = 20` (parentheses respected). | OR-14 |
| STO-30 | formula | `test_builtin_function_access_blocked` | Formula referencing language built-ins, dynamic import, file opening, or eval/exec constructs → rejected; nothing executes. | OR-14, OR-15 |
| STO-31 | formula | `test_attribute_access_blocked` | Formula containing attribute access on an object → rejected. | OR-14, OR-15 |
| STO-32 | formula | `test_list_comprehension_blocked` | Formula containing a list or generator comprehension → rejected; only scalar arithmetic is permitted. | OR-14, OR-15 |
| STO-33 | formula | `test_formula_evaluation_with_simpleeval` | Herd-wide formula `weight ÷ 2` with weights 100 and 200 → 50.0 and 100.0 (decimal results preserved, no truncation). | OR-17, OR-14 |
| STO-34 | formula | `test_formula_with_malicious_pattern_blocked` | Expression chaining a dynamic import to a shell command → rejected; no command runs. | OR-14, OR-15 |
| STO-35 | formula | `test_entirely_missing_field_returns_empty_dict` | Herd-wide formula on a field key that neither the data nor the field registry contains → empty result, not a crash. | OR-17 |
| STO-36 | formula | `test_no_valid_field_keys_raises` | Evaluate a formula containing no field placeholders (a bare constant `42`) → rejected with a "no field references" error. | OR-15 |
| STO-37 | formula | `test_missing_field_for_one_animal_returns_none` | Herd-wide formula over milk yield where only animal 1 has 30 → 30.0 for animal 1; animal 2 is absent from the results. | OR-17 |
| STO-38 | formula | `test_division_by_zero_returns_none` | Formula `weight ÷ 0` for a single animal → that animal's result is uncomputable, no crash, no infinity. | OR-16 |
| STO-39 | formula | `test_other_animals_unaffected_by_zero_division` | Formula `weight ÷ milk yield` across the herd where one animal lacks milk yield → the animal with both values gets a decimal, the other is uncomputable; one animal's failure never aborts the rest. | OR-16, OR-17 |
| STO-40 | formula | `test_valid_formula_passes` | Validate a formula referencing an existing field → passes with an empty message. | OR-15 |
| STO-41 | formula | `test_nonexistent_field_key_fails` | Validate a formula referencing an unknown field key → fails, and the message names the offending key. | OR-15 |
| STO-42 | formula | `test_no_placeholders_fails` | Validate a formula with no placeholders → fails with a message explaining the required placeholder syntax. | OR-15 |

## Waived scenarios (4)

| id | legacy test | behaviour guarded | why waived |
|---|---|---|---|
| STO-03 | `test_check_same_thread_disabled` | A read works off the creating thread | Legacy server/thread-pool connection machinery; no HTTP server and no thread pool in the rebuild (D2) |
| STO-04 | `test_timeout_set_correctly` | A bounded lock-wait setting is applied | Asserts where a connect argument is placed, not an outcome. The *behaviour* we keep is covered fresh: bounded wait then an actionable "database busy" error |
| STO-08 | `test_production_engine_connect_args` | Production wiring accepts its own configuration | Asserts that configuration is accepted by configuration; no user-observable outcome |

## Fresh — write from `REBUILD_PLAN` (no legacy coverage)

- **Migration discipline:** schema stamped with a version, forward-only ordered migrations, each idempotent when run twice; opening a *newer*-version file fails with an actionable message instead of corrupting it (SY-13).
- **Foreign keys enforced** at the storage layer — the legacy suite only ever tested application-level soft deletes (AGENTS.md rule 2/§9).
- **Startup failure paths:** unwritable data directory, read-only volume, disk full, corrupt file → actionable i18n error, app stays alive, never a panic.
- **Data-file location:** platform data directory on Linux **and** Windows, plus portable mode (SY-11, SY-12).
- **Concurrent writer:** bounded wait, then a clear "database busy" error — never a hang, never a silent drop.
- **WAL checkpoint** on shutdown and before backup (IX-9).
- **Backup verify on a corrupt file must report failure** — legacy only asserted the happy-path ok (IX-11).
- **Backup rotation keeps the newest 10; restore round-trip yields an identical database** (IX-10, IX-12).
- **Deactivated field's values are hidden from current-value surfaces while history stays readable** — legacy asserted retention only, never hiding (HM-22).
- **Reactivation end-to-end:** a reactivated field returns to its modes with refreshed label, unit and options (EX-4).
- **Active-mode fallback:** deactivating the selected mode re-scopes the UI to the system mode; no dangling selection.
- **Mode and linked-field ordering** — legacy asserted membership, never order (OR-10).
- **Mode-name normalisation:** whitespace and case variants must not create near-duplicates (STO-18 only covers exact equality).
- **System mode always exposes every field** even when link rows are missing (OR-11).
- **Link-replacement rollback:** a rejected replacement leaves the *previous* link set intact — legacy asserted only that it raised (STO-22).
- **Audit row per field deactivation / mode create / mode update / mode delete**, with actor and old→new values (SY-6, SY-7); **undo** of field deactivation and mode soft delete.
- **FR localisation of every rejection message** in this file (SY-5).
- **The settings key/value store itself:** language, theme, currency (TND default), developer-mode flag (SY-1…SY-4, SY-8, SY-16). The legacy module named for settings never tested it.
- **Formula hostility:** NaN/infinity, overflow, extreme magnitudes, negative inputs; money-typed inputs stay integer minor units.
- **Empty herd / all inputs missing → a structured "no data" result**, distinct from "cannot compute" (AN-22 analogue for computed fields).
- **Validation beyond two cases:** unbalanced or nested placeholders, empty `{{}}`, unknown operators, trailing garbage, over-long expressions, evaluation depth/step limits (OR-15).
- **Which observation feeds a formula** when several exist — latest by record date, newest-row tie-break (DE-7) is never asserted.
- **Formulas over a deactivated field**, and over identity facts such as computed age.
- **Malicious formula rejected at save time**, not only at the evaluation primitive.
- **Determinism and a performance budget** for herd-wide recomputation at 1k animals; cache invalidation.

## Anti-patterns — never reproduce

1. **Two representations of "no value":** an animal with no observation for a referenced field is *absent* from the result, while an animal whose computation fails is *present and uncomputable*. Same user-visible condition, two shapes.
2. **Validation and evaluation disagree:** evaluating a formula whose field key does not exist returns an empty result, while validation rejects the same key — a typo silently reads as "no animals have data".
3. **Division by zero and missing input collapse to the same null**, so the caller cannot distinguish "impossible" from "no data".
4. **Duplicate tests inflating coverage:** `test_missing_field_data_returns_none` and `test_missing_field_for_one_animal_returns_none` assert the identical scenario; `test_other_animals_unaffected_by_zero_division` never divides by zero.
5. **Sandbox asserted against a third-party evaluator's defaults** (deny-list style) rather than through the product's own formula path end-to-end.
6. **Transport status codes as the domain error contract** (400/404); domain errors must be typed and transport-free.
7. **Tests coupled to storage internals** (driver names, table and column names) — a rename breaks tests that guard nothing.
8. **Soft delete as a boolean flag with no single canonical "active" filter:** retention is asserted, exclusion from list surfaces is not — inactive fields and modes can leak into pickers and re-scoping.
9. **Inconsistent delete semantics across neighbouring entities** (configuration soft-deleted, animals hard-deleted) with no written rule.
10. **One "role" value overloading authorization with data purpose** (`primary`/`system` blocking deactivation *is* a purpose tag). Protection policy and purpose must be separate concepts.
11. **A module named for settings that tests everything except settings.**
12. **Fixtures reached across module boundaries** (values inferred from another file's fixture) — our fixtures come from `midata-testkit` only.
