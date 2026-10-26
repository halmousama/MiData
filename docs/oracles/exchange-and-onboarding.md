# Oracles — import, packs/extensions & onboarding

> **Extracted:** 2026-10-26 · **Method:** `REBUILD_PLAN.md` Appendix D — one owner-authorized session read
> **only** the legacy test files and transcribed test **names + asserted behaviours**. No code, API shape,
> endpoint path, manifest field spelling or legacy file name is reproduced. **The firewall exception was
> used and expired with that session** (AGENTS.md rule 5; `docs/gaps.md` P0-11 closed).
>
> **Scenarios:** 81 (import 24 · extensions 32 functions / 33 cases · onboarding 25) → **81 kept, 0 waived**;
> 3 legacy-modelling notes.
> **Inventory ids:** `docs/inventory.md`.
>
> Scenario ids (`EXC-nn`) are the stable handle: cite them in an inventory row's `evidence` column.

## Import

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| EXC-01 | import | `test_creates_new_entities_and_measurements` | Two rows, each a new identifier plus two numeric field values → 2 animals created, 0 updated, 0 skipped, no errors; 4 observations written (2 per animal). | DE-10, DE-1, HM-1 |
| EXC-02 | import | `test_import_with_record_date` | A row carrying an explicit record date → the observation is stored against that date, not the import date. | DE-1, DE-10 |
| EXC-03 | import | `test_skip_existing_entity` | A row whose identifier already exists, strategy skip → counted skipped, 0 added, 0 observations written, the existing animal untouched. | DE-11, HM-2 |
| EXC-04 | import | `test_update_adds_measurements` | Existing animal plus a field never recorded before, strategy update → counted updated (not added), one observation appended to that animal. | DE-12, DE-1 |
| EXC-05 | import | `test_update_replaces_existing_measurement` | Same animal, same field, same date imported twice under update → still exactly one observation holding the second value, reported updated, no errors. | DE-8, DE-12, DE-6 |
| EXC-06 | import | `test_duplicate_custom_id_in_single_batch` | Two rows in one file sharing an identifier under update → one animal (1 added + 1 updated) and one observation for that field and date; last value wins. | DE-8, DE-12, DE-19 |
| EXC-07 | import | `test_import_does_not_partially_commit_on_entity_error` | A batch mixing one pre-existing and two new identifiers → both new rows commit and the herd grows to 3 (per-row continuation is intentional). | DE-13 |
| EXC-08 | import | `test_commit_failure_triggers_rollback` | A second batch on the same store after a first → both rows commit and the store stays queryable. *(No failure is injected — see anti-pattern 1.)* | DE-13 |
| EXC-09 | import | `test_batch_wrapped_in_single_transaction` | Two rows in one batch → both committed and visible when the call returns. | DE-16 |
| EXC-10 | import | `test_number_field_with_bad_string` | A numeric field cell holding non-numeric text → the animal is still created, that value is rejected with a conversion error, and no observation is written for it. | DE-13, OR-4, DE-4 |
| EXC-11 | import | `test_select_field_with_invalid_choice` | A select field cell outside the defined choices → the animal is created, the value is rejected as not a valid option, no observation for it. | OR-4, DE-13, DE-4 |
| EXC-12 | import | `test_select_field_with_valid_choice` | A select field cell matching a defined choice → one observation holding that choice. | DE-4, DE-1 |
| EXC-13 | import | `test_boolean_field_stored_as_string` | A boolean field cell reading Yes → normalised to true and stored as a boolean value. | DE-4 |
| EXC-14 | import | `test_boolean_false_value` | A boolean field cell reading no → normalised to false. | DE-4 |
| EXC-15 | import | `test_date_parsing_error` | A row whose record date is unparseable → the whole row is discarded (no animal created) with an invalid-date error. | DE-13 |
| EXC-16 | import | `test_unknown_field_key` | A column matching no field definition → reported as an unknown field key, the animal is still created, no observation for it. | IX-3, DE-13 |
| EXC-17 | import | `test_entity_fields_not_measured` | A row with identifier, a free-text note and one field value → the note lands on the animal's identity record; only the field value becomes an observation. | IX-4 |
| EXC-18 | import | `test_empty_rows` | Empty batch → all counters zero, empty error list, no writes. | DE-17 |
| EXC-19 | import | `test_missing_custom_id_skipped` | A row with values but no identifier → row skipped, no animal created. | DE-13, HM-1 |
| EXC-20 | import | `test_null_values_skipped` | A row with an identifier and two empty cells → the animal is created and no observations are written for the empty cells. | DE-18, DE-13 |
| EXC-21 | import | `test_entity_notes_updated_on_update` | Existing animal plus a row carrying a note and a field value under update → the note is replaced and the row counted updated. | DE-12, HM-5 |
| EXC-22 | import | `test_import_with_text_field` | A free-text field cell → stored as the observation's text value. | DE-4, DE-1 |
| EXC-23 | import | `test_number_field_exact_float` | A fractional numeric cell (650.5) → stored without rounding. | DE-4, DE-1 |
| EXC-24 | import | `test_import_result_shape` | Any import → the result exposes exactly four members: added, updated and skipped as integers, errors as a list. | DE-13 |

## Packs / extensions

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| EXC-25 | extensions | `test_valid_manifest_parses` | A well-formed pack manifest (identifier, name, version, 2 fields, 1 mode, 1 widget) → parses and reports 2 fields, 1 mode, 1 widget. | EX-1 |
| EXC-26 | extensions | `test_missing_required_key_raises` | Manifest without a pack identifier → rejected at validation, nothing installed. | EX-1 |
| EXC-27 | extensions | `test_invalid_data_type_raises` | Manifest field declaring an unsupported data type → rejected with an invalid-data-type error. | EX-1, OR-4 |
| EXC-28 | extensions | `test_select_without_choices_raises` | Manifest select field with an empty options list → rejected at validation. | EX-1, OR-4 |
| EXC-29 | extensions | `test_invalid_role_raises` | Manifest field with an unknown purpose tag → rejected. | EX-1, OR-1 |
| EXC-30 | extensions | `test_invalid_widget_type_raises` | Manifest widget with an unsupported widget type → rejected. | EX-1, DB-9 |
| EXC-31 | extensions | `test_empty_fields_list_valid` | Manifest with zero fields, modes and widgets → accepted as a metadata-only pack. | EX-1 |
| EXC-32 | extensions | `test_double_install_skips_fields` | The same pack installed twice → the first creates 2 fields, the second creates 0 and reports 2 skipped; the field count is unchanged. | EX-2, EX-13 |
| EXC-33 | extensions | `test_double_install_does_not_duplicate_modes` | Same pack twice → the mode count is identical before and after. | EX-5, EX-13 |
| EXC-34 | extensions | `test_double_install_updates_widgets` | Same pack twice → exactly one widget, refreshed in place rather than added. | EX-6, EX-13 |
| EXC-35 | extensions | `test_type_mismatch_raises` | A pack field whose key already exists with a different data type → install refused with a type-mismatch error and nothing written. | EX-3 |
| EXC-36 | extensions | `test_same_type_skips` | A pack declaring one field with an existing key and type plus one brand-new field → 1 skipped and 1 created in the same install. | EX-3, EX-2 |
| EXC-37 | extensions | `test_rollback_on_field_type_conflict` | A manifest listing a conflicting field first and a valid new field second → the install aborts on the conflict and the later field is never persisted. | EX-8, EX-3 |
| EXC-38 | extensions | `test_rollback_on_bad_manifest_format` | A valid pack already installed, then an install attempt from a malformed manifest → field, mode and widget counts are identical to the pre-attempt snapshot. | EX-8, EX-1 |
| EXC-39 | extensions | `test_successful_install_is_persistent` | Installing one pack → 2 fields, 1 mode, 1 widget and at least 2 mode→field links persisted. | EX-2, EX-5, EX-6, EX-12 |
| EXC-40 | extensions | `test_select_allowed` | A plain read-only statement → accepted unchanged. | AI-6 |
| EXC-41 | extensions | `test_select_with_cte_allowed` | A single statement beginning with a common table expression → accepted. | AI-6 |
| EXC-42 | extensions | `test_drop_blocked` | A statement dropping a table → rejected as not read-only. | AI-6, AI-19 |
| EXC-43 | extensions | `test_delete_blocked` | A delete statement → rejected. | AI-6, AI-19 |
| EXC-44 | extensions | `test_insert_blocked` | An insert statement → rejected. | AI-6, AI-19 |
| EXC-45 | extensions | `test_update_blocked` | An update statement → rejected. | AI-6, AI-19 |
| EXC-46 | extensions | `test_alter_table_blocked` | A schema-altering statement → rejected. | AI-6, AI-19 |
| EXC-47 | extensions | `test_copy_to_blocked` | A statement exporting query output to a file path → rejected (no filesystem writes from a query). | AI-6, AI-19 |
| EXC-48 | extensions | `test_attach_blocked` | A statement attaching an external file as a data source → rejected. | AI-6, AI-19 |
| EXC-49 | extensions | `test_empty_query_blocked` | Empty query text → rejected with an empty-query diagnostic. | AI-6 |
| EXC-50 | extensions | `test_multiple_statements_blocked` | Two statements separated by a semicolon → rejected as multiple statements. | AI-6 |
| EXC-51 | extensions | `test_trailing_semicolon_allowed` | One statement with a single trailing semicolon → accepted. | AI-6 |
| EXC-52 | extensions | `test_whitespace_around_select_allowed` | A statement padded with leading and trailing whitespace → accepted and returned trimmed. | AI-6 |
| EXC-53 | extensions | `test_select_with_pragma_like_string` | A read-only statement containing a write keyword inside a string literal → accepted, the literal preserved. | AI-6 |
| EXC-54 | extensions | `test_bundled_manifest_valid` *(parametrised over 2 bundled packs)* | Each bundled pack's manifest → exists, parses, and its declared identifier matches its directory name. | EX-11, EX-1 |
| EXC-55 | extensions | `test_livestock_manifest_has_required_fields` | The bundled livestock pack → defines the core identity and production fields (species, sex, breed, weight, milk yield). | EX-11, OR-1 |
| EXC-56 | extensions | `test_analytics_manifest_has_widgets` | The bundled analytics pack → defines at least one widget and every widget carries a query. | EX-11, DB-9 |

## Onboarding

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| EXC-57 | onboarding | `test_not_onboarded_when_empty` | Fresh database → status reports not onboarded with zero fields, animals and modes. | OB-11 |
| EXC-58 | onboarding | `test_onboarded_when_config_and_fields_exist` | A completion marker present and at least one field defined → status reports onboarded with a field count of at least 1. | OB-11 |
| EXC-59 | onboarding | `test_not_onboarded_when_missing_config` | Fields exist but no completion marker → not onboarded. | OB-11 |
| EXC-60 | onboarding | `test_not_onboarded_when_config_only` | Marker present but no fields → not onboarded. | OB-11 |
| EXC-61 | onboarding | `test_returns_catalogue` | Request the field-bundle catalogue → six named bundles returned. | OB-4 |
| EXC-62 | onboarding | `test_each_bundle_has_fields` | Every bundle in the catalogue → defines at least one field; no empty bundle. | OB-4 |
| EXC-63 | onboarding | `test_basic_identity_has_core_fields` | The basic-identity bundle → contains species, sex, breed and birth date. | OB-4 |
| EXC-64 | onboarding | `test_creates_fields_and_modes` | One bundle with 4 of 6 fields included, two preset species plus one custom species, health toggles on, no demo data → 4 fields, 1 user mode plus 1 system mode, 5 widgets, 4 mode→field links, 0 animals, 0 observations. | OB-1, OB-3, OB-4, OB-5, OB-6, OB-7 |
| EXC-65 | onboarding | `test_double_run_is_idempotent` | The same onboarding executed twice → the second run creates 0 fields, 0 modes, 0 widgets and all four counts are unchanged. | OB-2 |
| EXC-66 | onboarding | `test_merge_preserves_existing_data` | A populated database (one user field, one user animal), then onboarding with a bundle and demo seed → the user field keeps its label, the user animal keeps its note, and 10 demo animals are appended (11 total). | OB-2, OB-8 |
| EXC-67 | onboarding | `test_custom_field_exclusion` | Bundle selection excluding 5 of 6 fields → exactly 1 field created. | OB-4 |
| EXC-68 | onboarding | `test_multiple_bundles` | Two bundles selected → 12 fields, 2 modes, 7 widgets. | OB-4, OB-5, OB-6 |
| EXC-69 | onboarding | `test_empty_bundles_does_nothing` | No bundles selected → 0 fields, 0 modes, 0 widgets, success. | OB-13 |
| EXC-70 | onboarding | `test_unknown_bundle_id_skipped` | An unrecognised bundle identifier → ignored, nothing created. | OB-14, OB-4 |
| EXC-71 | onboarding | `test_seeds_entities_and_measurements` | Onboarding with demo data → 10 animals all carrying a demo identifier prefix and more than 50 observations across a time series. | OB-8, OB-9 |
| EXC-72 | onboarding | `test_idempotent_seed` | Demo seed executed twice → still exactly 10 animals, no duplicate identifiers. | OB-8 |
| EXC-73 | onboarding | `test_failure_does_not_leave_partial_data` | A pre-existing field whose key collides with a bundle field but with a different data type → onboarding succeeds, reusing the existing field and creating the other 5. | OB-4, OB-2 |
| EXC-74 | onboarding | `test_no_financial_fields_by_default` | Financial tracking off → no purchase-price, treatment-cost or feed-cost fields. | OB-15 |
| EXC-75 | onboarding | `test_financial_purchase_creates_pricing_fields` | Purchase option on → purchase-price and sale-price fields created. | OB-15 |
| EXC-76 | onboarding | `test_financial_vet_creates_cost_fields` | Vet option on → treatment-cost and last-vet-visit fields created. | OB-15 |
| EXC-77 | onboarding | `test_financial_feed_creates_feed_cost` | Feed option on → feed-cost field created. | OB-15 |
| EXC-78 | onboarding | `test_all_financial_creates_five_fields` | All three financial options on → exactly those five money/visit fields present. | OB-15 |
| EXC-79 | onboarding | `test_financial_creates_mode_and_widgets` | Financial options on → a financial-tracking mode exists with its widgets. | OB-5, OB-6 |
| EXC-80 | onboarding | `test_financial_idempotent` | The same financial onboarding twice → the second run creates 0 fields and the field count is unchanged. | OB-2 |
| EXC-81 | onboarding | `test_financial_uses_currency_as_unit` | Currency TND chosen → the money fields carry TND as their unit label. | OB-12, OR-1 |

## Waived (3)

| id | legacy test | why |
|---|---|---|
| note · EXC-13/14 | boolean text encoding | The *behaviour* (Yes/no → true/false) is kept in DE-4; the assertion that a boolean is stored as the text `"true"`/`"false"` is waived — our observations store a typed numeric boolean |
| note · EXC-58/60 | on-disk completion marker | The *behaviour* (status reports setup complete) is kept in OB-11; deriving it from a marker **file** is waived — our setup state is derived from the database alone |
| note · EXC-63/81 | identity fields and currency as pack field unit | Waived as *modelling*: species/sex/breed/birth date are identity columns (D7), and money is integer minor units with one global currency setting (rule 11) — never a field unit label |

## Atomicity & idempotency oracles

**Import**
- Re-importing the same animal + field + date under update leaves **exactly one** observation holding the last value (upsert, never append).
- An in-file duplicate identifier under update yields **one** animal and **one** observation — proves no duplicate entity row, *not* batch atomicity.
- A batch mixing a pre-existing animal with two new ones commits both new rows; per-row continuation is intentional.
- A single bad cell does **not** roll back the animal created in the same row: the animal survives, the observation is omitted (row-level partial success by design).
- A bad record date discards that row entirely, but no multi-row test proves the remaining rows still commit.
- **Absent:** no test proves all-or-nothing for a failed import batch. The module's docstring promises atomicity coverage its bodies do not deliver.

**Packs**
- Installing twice: 2 fields created once, 0 created and 2 skipped the second time — field creation is idempotent.
- Second install leaves the mode count identical and exactly one widget (refreshed in place, not appended).
- A type conflict on the *first* manifest field aborts the install so a *later, valid* field in the same manifest is never persisted — the only genuine rollback proof in the whole extraction.
- A malformed manifest leaves field, mode and widget counts identical to the pre-attempt snapshot.
- A same-key-same-type field is skipped while a brand-new field in the same manifest is still created — deliberate partial application, distinct from rollback.
- **Absent:** no proof that a failure *after* a successful flush rolls back; no uninstall, so no uninstall→reinstall oracle (EX-9, EX-4); no restart-survival oracle (EX-12 asserted only within one session).

**Onboarding**
- Same onboarding twice: 0 fields, 0 modes, 0 widgets on the second run; field, mode, widget **and** link counts unchanged.
- Onboarding on a populated database preserves the user's field label and the user animal's note byte-for-byte and appends the 10 demo animals.
- Demo seeding twice keeps exactly 10 animals — no duplicate identifiers.
- Financial onboarding twice creates 0 new fields.
- A pre-existing field key is reused rather than duplicated — key-level merge.
- An unknown bundle identifier creates nothing; an empty selection creates nothing.
- **Absent:** there is **no** rollback oracle for onboarding, despite it writing fields, modes, links, widgets and thousands of observations in one call.

## Fresh — write from `REBUILD_PLAN` (no legacy coverage)

**Import / exchange**
- **Reading real files:** XLSX and CSV parsing, sheet selection, header-row detection, encoding/BOM, huge-file memory caps (IX-1, IX-2) — legacy fed pre-parsed rows only.
- **Column-mapping preview before commit** (IX-3); legacy only reported unknown columns post hoc.
- **Progress reporting and mid-import cancellation** (DE-14).
- **Idempotent re-import of the same file**; a second file with new dates appending correctly.
- **Structured, row-indexed per-row errors** — legacy errors are free-text substrings with no source-row link (DE-13).
- **Audit rows for import writes**, especially update overwrites (DE-12's "and audits" half).
- **Species validation on import**, and identity attributes beyond identifier and note — the sire/dam sex rules are never exercised through import (HM-8, HM-9).
- **Multi-select and date-typed field values**; unit and inline min/max bound enforcement (DE-5).
- **Identifier normalisation** (case, whitespace, leading zeros) and real cross-file duplicate detection (HM-2, DE-19).
- **Dry-run / validate-only mode; duplicate identifier columns in one mapping.**
- **Performance bound on large batches** (1k rows < 10 s).

**Packs**
- **Uninstall (soft deactivate, data retained) and the full uninstall→reinstall cycle** (EX-9, EX-4).
- **Per-species gestation seeding from a pack** (EX-7) — the legacy pack test never asserts it.
- **Install-state persistence across a restart** (EX-12).
- **Pack discovery beyond bundled packs**, user-added packs, version upgrade/downgrade, ordering, dependencies (EX-1).
- **Concurrent installs of two packs touching the same field key.**
- **Manifest size limits; validation of pack-supplied widget queries at install time** (AI-19).
- **EN/FR labels inside pack manifests** — legacy manifests are single-language (SY-5).
- **End-to-end proof that installed fields appear in entry, list and dashboard surfaces.**

**Onboarding**
- **The join between the halves:** no test runs onboarding and then asserts status reports onboarded; status tests fabricate the marker and fields by hand (OB-11).
- **A re-onboarding gate:** the docstring promises rejection; only idempotency is tested.
- **Custom-species persistence** — a custom species is passed and never asserted (OB-3).
- **Health diagnostic/vaccination/quarantine field creation** — three toggles passed, never asserted.
- **Locale persistence** — only currency is observed, and only as a unit label (OB-12).
- **Demo-seed determinism** (same seed → identical values); only count-level idempotency is proven (OB-8).
- **Seed species policy (OB-10) and seed richness:** pedigree generations, phantom parents, rules, computed fields (OB-9).
- **Rollback of a mid-onboarding failure.**
- **Mode ordering, system-mode field coverage (OR-11), widget ordering and collisions.**
- **Onboarding into a database that already has a different bundle installed.**

## Anti-patterns — never reproduce

> **Feeds `REBUILD_PLAN.md` §3 (one authoritative home — this list is provenance only):**
> 1 → `test-integrity.no-vacuous-assertions` · 2 → `test-integrity.named-for-boundary` ·
> 3 → `import.row-diagnostics` · 4 → `import.row-diagnostics` · 5 → `import.row-diagnostics` ·
> 6 → `import.one-field-path` · 7 → `test-integrity.no-vacuous-assertions` ·
> 8 → `onboarding.derived-state` · 9 → `seed.explicit-guard` · 10 → `config.inputs-exercised` ·
> 11 → `config.inputs-exercised` · 12 → `query.ast-validation` · 13 → `query.ast-validation` ·
> 14 → `errors.typed-reasons` · 15 → `value.typed-at-write` ·
> 16 → D21/D5 (waived surfaces — not a register row)

1. **Vacuous atomicity tests.** Three of four "atomicity" tests inject no failure and then assert success. Never accept an all-or-nothing claim without an injected failure and a zero-rows assertion (see PD-17 for the same defect).
2. **Mislabelled tests.** One test claims "does not partially commit" and asserts both rows *do* commit; another named for "uninstall twice is idempotent" asserts the second call *raises*.
3. **Silent merge of duplicate identifiers within one file:** two rows sharing a tag become one animal with the last value, reported as "1 added, 1 updated". Refuse with an actionable duplicate-identifier error (DE-19, HM-2).
4. **Silently dropped rows:** a row with no identifier is counted as skipped with no reason; the same for an unrecognised bundle identifier. The user cannot tell which rows were lost (OB-14).
5. **Aggregate-only feedback:** counters plus free-text error strings with no source-row indices — unactionable for the user and untestable structurally.
6. **Two install paths with two safety levels:** a field-key/type collision is fatal for a pack install but silently tolerated during onboarding. Exactly one field-creation path.
7. **Onboarding has no transaction boundary proof** despite writing thousands of rows per call.
8. **Split setup-complete state** (marker file *and* database rows must agree, and both disagreement cases end in "not onboarded" while data already exists). Derive state from one place.
9. **Demo seed into a live herd:** 10 demo animals appended next to a real animal with no confirmation or guard.
10. **Widgets created regardless of settings:** three profitability widgets produced even with financial tracking disabled.
11. **Dead configuration inputs:** three health toggles and a custom-species list accepted and never exercised.
12. **Query safety by first-keyword string check:** no AST, no column whitelist, no caps; it cannot prove the absence of a write path and couples validation to whitespace trimming. Use `midata-queryguard` (AI-7, AI-8).
13. **Raw query text shipped inside pack manifests** with no validation at install time.
14. **Misleading diagnostics:** a malformed manifest is reported as "not found" rather than a parse error.
15. **Encoding and money choices we must not copy:** booleans as text; a currency used as a field unit label; core identity attributes shipped as dynamic bundle fields.
16. **Legacy-only surfaces to waive, not rebuild:** the multi-step wizard (D21), the fixed six-bundle catalogue, marketplace-style pack browsing, the on-disk completion marker, and any legacy-database migration (D5).
