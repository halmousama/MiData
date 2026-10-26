# Oracles — launcher surface (waived) & the misnamed sharing module (kept)

> **Extracted:** 2026-10-26 · **Method:** `REBUILD_PLAN.md` Appendix D — one owner-authorized session read
> **only** the legacy test files and transcribed test **names + asserted behaviours**. No code, API shape,
> endpoint path, port number, credential format or legacy file name is reproduced. **The firewall exception
> was used and expired with that session** (AGENTS.md rule 5; `docs/gaps.md` P0-11 closed).
>
> **Scenarios:** 54 (launcher 17 · "local-sharing" 37) → **42 kept, 12 waived**.

## ⚠ Correction to the waiver record

`docs/testing.md` §7 and `docs/inventory.md` (Waivers table) previously recorded **`test_local_sharing` (37
tests) as waived** on the assumption that the module covers the dropped LAN/tunnel surface. **It does not.**
The module contains no sharing, tunnel, QR-code or browser-UI assertion anywhere. Its 37 tests cover:

| class | tests | real subject |
|---|---|---|
| `TestAnalyticsSecurity` | 16 | read-only query validation — the direct precedent for `midata-queryguard` |
| `TestExport` | 7 | setup-template export (configuration only) |
| `TestImport` | 11 | setup-template import, idempotency, type conflicts |
| `TestRoundtrip` | 3 | export → import → export round-trip |

Only `test_launcher.py` matches the dropped-surface premise, and even there **5 of its 17 tests are kept**.
Waiving the sharing module would have silently discarded the rebuild's two strongest existing precedents
(the queryguard rules and the pack/template contract). **The waiver now applies to 12 tests, not 49.**
Corrected here, in `docs/testing.md` §7 and in `docs/inventory.md`.

## `test_launcher.py` — launcher / desktop bootstrap (17 → 12 waived, 5 kept)

| id | legacy test | behaviour guarded | verdict | inventory |
|---|---|---|---|---|
| WAV-01 | `test_returns_valid_port` | A free TCP port is found for the bundled local server and is bindable | waived — D2 no server / no LAN surface | — |
| WAV-02 | `test_specific_start_value` | Port probing honours a caller-supplied start port | waived — D2 no server | — |
| WAV-03 | `test_raises_when_port_range_exhausted` | Probing fails cleanly when every port in the range is taken | waived — D2 no server | — |
| WAV-04 | `test_port_is_available_after_release` | A port freed by closing its socket is found again | waived — D2 no server | — |
| WAV-05 | `test_dev_mode_no_meipass` | A bundled resource resolves relative to the launcher when unfrozen | waived — not applicable to a native binary (packaging/process bootstrap) | — |
| WAV-06 | `test_frozen_mode_with_meipass` | The same resource resolves inside the frozen bundle root | waived — not applicable to a native binary | — |
| WAV-07 | `test_ensure_dirs_creates_directory` | The app creates its own private directory before the first write | **kept** — idempotent, non-fatal directory creation | SY-12 |
| WAV-08 | `test_save_and_load_token` | A tunnel credential written to disk reads back unchanged | waived — D2 no public tunnel | — |
| WAV-09 | `test_load_token_empty_when_no_file` | An absent config file yields an empty value instead of crashing | **kept** | SY-16 |
| WAV-10 | `test_load_token_empty_when_corrupt_json` | Corrupt config content degrades to a default instead of crashing | **kept** | SY-16 |
| WAV-11 | `test_load_token_missing_key` | Config content lacking an expected key degrades to a default | **kept** | SY-16 |
| WAV-12 | `test_overwrite_token` | Writing a setting twice replaces the earlier value | **kept** | SY-1 |
| WAV-13 | `test_returns_string` | Local network address discovery always yields a non-empty string | waived — D2 no LAN surface | — |
| WAV-14 | `test_initial_state` (server manager) | The embedded server starts not-running on a default port | waived — D2 no server | — |
| WAV-15 | `test_initial_state` (tunnel manager) | No tunnel is active and no public URL exists before start | waived — D2 no public tunnel | — |
| WAV-16 | `test_double_stop_is_safe` | Stopping an already-stopped tunnel is a no-op, never raises | waived — D2 no public tunnel | — |
| WAV-17 | `test_log_updates_text` | A launcher console window appends and shows log lines | waived — not applicable to a native binary | — |

## The 37 "sharing" tests — all kept

### Read-only query validation (16) — precedent for `midata-queryguard`

| id | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|
| WAV-18 | `test_select_only_passes` | A single plain read executes and returns its value. | AI-6 |
| WAV-19 | `test_select_with_cte_allowed` | A read beginning with a common table expression is accepted. | AI-6 |
| WAV-20 | `test_select_with_union_passes` | A union of reads is accepted. *(Note: our contract requires every branch to pass the read-only gate and the whitelist — see `query-validation.md`.)* | AI-6 |
| WAV-21 | `test_drop_rejected` | A schema-dropping statement is refused. | AI-19 |
| WAV-22 | `test_insert_rejected` | An insert statement is refused. | AI-19 |
| WAV-23 | `test_update_rejected` | An update statement is refused. | AI-19 |
| WAV-24 | `test_delete_rejected` | A delete statement is refused. | AI-19 |
| WAV-25 | `test_attach_rejected` | Attaching an external database file is refused. | AI-19 |
| WAV-26 | `test_copy_to_rejected` | Writing query output to a filesystem path is refused. | AI-19 |
| WAV-27 | `test_alter_table_rejected` | A schema-altering statement is refused. | AI-19 |
| WAV-28 | `test_call_rejected` | A procedure call is refused. | AI-19 |
| WAV-29 | `test_multi_statement_rejected` | A stacked second statement after a semicolon is refused. | AI-6 |
| WAV-30 | `test_empty_query_rejected` | An empty query string is refused. | AI-19 |
| WAV-31 | `test_trailing_semicolon_allowed` | One trailing semicolon does not disqualify a valid query. | AI-6 |
| WAV-32 | `test_comment_wrapper_blocked` | A forbidden statement hidden behind a leading comment is still refused. | AI-19 |
| WAV-33 | `test_mixed_case_drop_blocked` | A mixed-case spelling of a forbidden verb is still refused. | AI-19 |

### Setup-template export (7)

| id | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|
| WAV-34 | `test_export_empty_db_returns_empty_template` | Exporting an empty setup yields empty sections plus a format version and a timestamp. | OR-29 |
| WAV-35 | `test_export_includes_all_fields` | Every field definition exports with its label, category, data type, unit and order. | OR-29 |
| WAV-36 | `test_export_includes_modes_with_field_links` | Modes export together with their field references. | OR-29 |
| WAV-37 | `test_export_includes_widgets` | Dashboard widget definitions export with their configuration. | OR-29 |
| WAV-38 | `test_export_excludes_entity_data` | An export carries configuration only — never animals or observations. | OR-29 |
| WAV-39 | `test_export_format_version` | An export declares a format version for forward compatibility. | OR-29 |
| WAV-40 | `test_export_with_real_data` | A fully populated setup exports all sections consistently. | OR-29 |

### Setup-template import (11)

| id | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|
| WAV-41 | `test_import_creates_fields` | Importing creates the field definitions that are missing. | EX-2 |
| WAV-42 | `test_import_creates_modes` | Importing creates modes and their field links. | EX-5 |
| WAV-43 | `test_import_creates_widgets` | Importing creates widget definitions with their configuration. | EX-6 |
| WAV-44 | `test_import_idempotent_second_import` | Importing the same content twice updates rather than duplicates. | EX-4 |
| WAV-45 | `test_import_type_conflict_skipped` | A field whose key exists with another data type is skipped and reported. | EX-3 |
| WAV-46 | `test_import_updates_field_metadata` | Same key and type refreshes label and category in place. | EX-4 |
| WAV-47 | `test_import_empty_template_noop` | Importing empty content changes nothing. | OR-30 |
| WAV-48 | `test_import_preserves_entities_and_measurements` | An import never touches animal or observation records. | OR-30 |
| WAV-49 | `test_import_mode_field_links` | Imported modes end up linked to exactly the referenced fields. | EX-5 |
| WAV-50 | `test_import_with_all_types` | Every supported value type imports with its type intact. | EX-2 |
| WAV-51 | `test_import_partial_template` | A template with only one section creates only that section. | OR-30 |

### Export → import round-trip (3)

| id | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|
| WAV-52 | `test_roundtrip_noop` | Export then re-import is a no-op that creates no duplicates. | OR-29 |
| WAV-53 | `test_roundtrip_export_after_import` | A re-export reflects exactly what was imported. | OR-29 |
| WAV-54 | `test_import_mode_rebuilds_links` | Re-importing a mode replaces its links instead of accumulating them. | EX-5 |

## Waiver summary

| waiver class | count |
|---|---|
| D2 — no server / no LAN surface | 6 |
| D2 — no public tunnel | 3 |
| D2 — no browser UI / no SPA bundle | 0 |
| Not applicable to a native binary (packaging / tray / process bootstrap) | 3 |
| **Kept — behaviour survives in the rebuild** | **42** |
| **Total** | **54** |

**Why the 12 are waived.** They guard the lifecycle of a desktop launcher that exists only to wrap a bundled
service and a browser UI: picking a free port for that service, discovering the machine's LAN address to
print a share URL, holding a tunnel credential and starting/stopping the tunnel, resolving resources inside a
frozen bundle, and drawing a launcher console window. MiData is a single native Rust binary — there is no
process to launch, no port to bind, no tunnel to authenticate, no bundle root to resolve and no share link.
Nothing here was waived for convenience: the tunnel-credential test is waived even though MiData does store a
secret, because that secret belongs to the LLM endpoint (AI-16, keyring), not to a tunnel.

## Anti-patterns noted while transcribing

> **Feeds `REBUILD_PLAN.md` §3 (one authoritative home — this list is provenance only):**
> 1 → `test-integrity.named-for-boundary` · 2 → §3 row 5 + `errors.typed-reasons` ·
> 3 → §3 row 8 · 4 → §3 row 7 · 5 → `identity.machine-keys`

1. **A module named for a surface it does not test** — the single most misleading artefact in the legacy
   suite, and the reason a blanket waiver would have silently dropped 42 valid oracles.
2. **Transport-layer behaviour asserted as domain behaviour** (port probing, socket release, process
   freezing) — none of it is user-visible product behaviour.
3. **A credential file on disk with no integrity or permission model** — superseded by the OS keyring.
4. **Launcher console as a UI surface** — superseded by the real application window.
5. **Export/import identity keyed on display names** (see `retention-and-soft-delete.md` anti-pattern 6).
