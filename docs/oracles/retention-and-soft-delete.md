# Oracles — retention, deactivation & pack uninstall

> **Extracted:** 2026-10-26 · **Method:** `REBUILD_PLAN.md` Appendix D — one owner-authorized session read
> **only** the legacy test files and transcribed test **names + asserted behaviours**. No code, API shape,
> endpoint path, active-flag spelling or legacy file name is reproduced. **The firewall exception was used
> and expired with that session** (AGENTS.md rule 5; `docs/gaps.md` P0-11 closed).
>
> **Scenarios:** 36 → **36 kept, 0 waived**; 3 legacy-modelling notes.
> **Inventory ids:** `docs/inventory.md`.
>
> Scenario ids (`RET-nn`) are the stable handle: cite them in an inventory row's `evidence` column.

## Scenarios

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| RET-01 | soft-delete | `test_field_soft_delete_sets_inactive` | Deactivate an active field definition → the record is retained marked inactive, nothing is removed from storage, and the confirmation identifies which field was deactivated. | OR-2 |
| RET-02 | soft-delete | `test_field_soft_delete_preserves_measurements` | Deactivate a numeric field holding one recorded observation (100) → the observation row and its value survive untouched; zero cascade. | OR-2, HM-22, DE-7 |
| RET-03 | soft-delete | `test_field_soft_delete_removes_from_active_query` | List fields for active surfaces before and after deactivating one → 1 visible, then 0 visible; the definition still exists in storage. | OR-2, HM-22 |
| RET-04 | soft-delete | `test_field_reactivation_restores_visibility` | Reactivate an inactive field → it becomes active again and reappears in active listings. *(The same toggle on an already-active field deactivates it — one command, both directions.)* | OR-2, OR-35 |
| RET-05 | soft-delete | `test_system_field_cannot_be_deactivated` | Attempt to deactivate a field carrying the protected system purpose → refused with an actionable message; the field stays active and nothing is written. | OR-31 |
| RET-06 | soft-delete | `test_primary_field_cannot_be_deactivated` | Attempt to deactivate the field carrying an animal's primary identifier → refused with an actionable message; the field stays active. | OR-32 |
| RET-07 | soft-delete | `test_mode_soft_delete_sets_inactive` | Deactivate a mode → the record is retained marked inactive. *(Boundary: deactivating an already-inactive mode is a no-op or a clear error, never a delete.)* | OR-9 |
| RET-08 | soft-delete | `test_mode_soft_delete_preserves_field_links` | Deactivate a mode that links one field → the mode↔field link rows survive intact; the mode leaves active surfaces while its link set is preserved for reactivation. | OR-9, OR-10 |
| RET-09 | soft-delete | `test_mode_soft_delete_removes_from_active_query` | Deactivate a mode, then list modes for active surfaces → 0 returned; the mode record still exists. | OR-33 |
| RET-10 | soft-delete | `test_mode_reactivation_restores_visibility` | Reactivate an inactive mode → it reappears in active mode listings with its name and link set intact. | OR-34, OR-33 |
| RET-11 | soft-delete | `test_system_mode_cannot_be_deactivated` | Attempt to deactivate the protected system mode (the one that always shows all fields) → refused with an actionable message; the mode stays active. | OR-11 |
| RET-12 | soft-delete | `test_widget_soft_delete_sets_inactive` | Deactivate a user-owned dashboard widget → the widget record (title, chart type, data-source configuration) is retained marked inactive and stops being hydrated on the dashboard. | DB-12, DB-9 |
| RET-13 | soft-delete | `test_widget_soft_delete_removes_from_active_query` | Deactivate a widget, then list widgets for the dashboard → 0 active; the record is retained. | DB-12 |
| RET-14 | soft-delete | `test_extension_widget_cannot_be_deleted_directly` | Attempt to deactivate a widget belonging to an installed pack → refused; only uninstalling the pack changes its state. | EX-14 |
| RET-15 | soft-delete | `test_widget_reactivation` | Reactivate an inactive widget → it returns to the dashboard and is hydrated again. | DB-12, OR-35 |
| RET-16 | soft-delete | `test_mode_toggle_cycles_active_state` | Toggle a mode twice → inactive, then active; the state persists between reads. | OR-34, OR-35 |
| RET-17 | soft-delete | `test_system_mode_toggle_blocked` | Attempt to toggle the protected system mode → refused with an actionable message; the mode remains active. | OR-11 |
| RET-18 | soft-delete | `test_widget_toggle_cycles_active_state` | Toggle a widget twice → hidden, then visible again; the state persists. | DB-12, OR-35 |
| RET-19 | soft-delete | `test_uninstall_deactivates_all_resources` | Install a pack contributing 2 fields, 1 mode and 1 widget, then uninstall it → all four resources (including the install record) become inactive, none deleted, and the report counts 2 fields / 1 mode / 1 widget deactivated. | EX-9, EX-5, EX-6 |
| RET-20 | soft-delete | `test_uninstall_preserves_measurements` | Record an observation (42) on a pack-contributed field, then uninstall the pack → the observation and its value survive. | EX-9, HM-22 |
| RET-21 | soft-delete | `test_uninstall_then_reinstall_reactivates` | Install → uninstall → reinstall the same pack → reports 2 fields reactivated and 0 created; all resources active again with no duplicates. | EX-4, EX-5, EX-6, EX-2 |
| RET-22 | soft-delete | `test_uninstall_nonexistent_extension_raises_error` | Uninstall a pack that was never installed → graceful, actionable error; no state change. | EX-15 |
| RET-23 | soft-delete | `test_uninstall_twice_is_idempotent` | Uninstall the same pack twice → the second attempt is refused as already inactive; the first uninstall's state is preserved unchanged. | EX-9 |
| RET-24 | soft-delete | `test_import_reactivates_deactivated_field` | Import a setup template whose field key matches an existing inactive field of the same data type → the field is reactivated (1 reactivated, 0 created) and its label and category are refreshed from the template. | OR-30, EX-4 |
| RET-25 | soft-delete | `test_import_skips_field_type_conflict_even_if_inactive` | Import a template field whose key matches an inactive field but with a different data type → skipped as a type conflict, listed in the report, and the existing field stays inactive with its original type. | EX-3, OR-30 |
| RET-26 | soft-delete | `test_import_reactivates_deactivated_mode` | Import a template whose mode name matches an inactive mode → the mode is reactivated (1 reactivated, 0 created). | OR-30, EX-5 |
| RET-27 | soft-delete | `test_import_reactivates_deactivated_widget` | Import a template whose widget title matches an inactive widget → the widget is reactivated (1 reactivated, 0 created) and its chart type and data source are updated. | OR-30, EX-6 |
| RET-28 | soft-delete | `test_list_modes_active_only` | List modes with an active-only filter over one active and one inactive mode → only the active one is returned. | OR-33 |
| RET-29 | soft-delete | `test_list_modes_all_when_active_only_false` | List modes without the filter → both active and inactive modes are returned, so management surfaces can reactivate them. | OR-33 |
| RET-30 | soft-delete | `test_export_excludes_inactive_modes` | Export a setup template containing one active and one inactive mode → the inactive mode is absent from the export. | OR-29 |
| RET-31 | soft-delete | `test_export_excludes_inactive_widgets` | Export a setup template containing one active and one inactive widget → the inactive widget is absent from the export. | OR-29 |
| RET-32 | soft-delete | `test_export_excludes_inactive_fields` | Export a setup template containing one active and one inactive field → the inactive field is absent from the export. | OR-29 |
| RET-33 | soft-delete | `test_field_soft_delete_does_not_cascade_to_measurements` | Record an observation (99.9), deactivate its field, re-read → observation count unchanged; the row still points at that field and keeps its value. | OR-2, HM-22, DE-7 |
| RET-34 | soft-delete | `test_installed_extension_is_active` | A pack currently installed and enabled → its install state reads as active on the packs surface. | EX-10, EX-16 |
| RET-35 | soft-delete | `test_deactivated_extension_status` | A pack that was installed and then uninstalled → its install state reads as deactivated, not absent. | EX-10, EX-16 |
| RET-36 | soft-delete | `test_not_installed_has_no_status` | A pack that was never installed → no install-state entry; the packs surface presents it as available to install. | EX-10, EX-16 |

## Retention & exclusion oracles

**Survives**
- An observation recorded against a field survives that field's deactivation, keeps its value, and keeps its reference to the field — and survives a pack uninstall too.
- The field, mode and widget definitions themselves survive deactivation, retained with an inactive marker; never removed.
- A deactivated mode's mode↔field link rows survive intact.
- The pack's install record survives uninstall in a deactivated state — which is what makes reinstall a reactivation path rather than a fresh install.
- Inactive configuration remains listable when the active-only filter is lifted (the management path back to reactivation).

**Disappears from**
- Active field listings, and therefore from anything derived from active configuration (forms, list columns, entry surfaces — implied by the design, untested by legacy).
- Active mode listings; the dashboard's hydrated widget set; the setup-template export (fields, modes and widgets).

**Reactivation**
- A reactivated field reappears in active field listings; a reactivated mode reappears with its links intact; a reactivated widget returns to the dashboard and is hydrated again.
- Reinstall after uninstall reactivates existing resources: 2 reactivated, 0 created — no duplicate fields, modes or widgets.
- Import refreshes metadata (label, category) on a reactivated field and updates chart type and data source on a reactivated widget.

**Refusals that change nothing**
- Deactivating a system-purpose field or an identity-carrying field; deactivating or deleting the system mode; deactivating a pack-owned widget; uninstalling a never-installed pack; uninstalling an already-inactive pack.
- Importing a template field whose key exists with a different data type leaves the existing field inactive with its original type and surfaces the conflict in the report.

## Waived (3)

| id | legacy test | why |
|---|---|---|
| note · RET-12 | widget configuration holding raw query text | Kept as behaviour (DB-9), but a widget's data source goes through the query validation lab — we never store unvalidated query text (AI-19) |
| note · RET-14 | pack-owned rows distinguished by a sentinel string in an id column | Kept as behaviour (EX-14), but provenance is an explicit enum field, not a reserved value a user-supplied id could collide with |
| note · RET-19/35 | exact legacy counter key names and three-state wording | Kept as behaviour; we report per-item outcomes and i18n'd state labels (SY-5) |

## Fresh — write from `REBUILD_PLAN` (no legacy coverage)

- **A deactivated field's exclusion from the current-value surface** (animal list columns, detail card) — legacy checked only a raw active-configuration listing (HM-22).
- **Exclusion from entry forms, analytics aggregates, correlation matrices and the AI column whitelist** (DE-4, AN-1, AI-7).
- **Whether a deactivated field's history stays readable** to the user (timeline, per-field chart) — retention is asserted in storage, never in a surface (HM-17, HM-18).
- **Reactivation restoring a field's position and order** within a mode, and the same for a reactivated mode's links (OR-10).
- **Reactivation of a field that already holds values** — whether latest-value semantics still resolve correctly afterwards (DE-7).
- **Deactivating a field a computed field or rule references** — frozen, hidden or warned is undefined (OR-13, OR-27).
- **Deactivating a species** and the fate of fields, modes and animals referencing it (OR-8).
- **Deactivating a mode that owns per-mode dashboard widgets** — whether those widgets hide with it (DB-11).
- **Atomicity of uninstall:** a failure mid-way through deactivating four resource classes leaves what state (EX-8, EX-9).
- **Audit rows for every deactivation, reactivation, install and uninstall**, with actor (SY-6, SY-7); **undo** of a deactivation; **confirmation** for uninstall.
- **Restart survival** of deactivation and install state (EX-12).
- **i18n of the protection and already-inactive errors** in both UI languages (SY-5).
- **Listing with counts and pagination** under the active-only filter (HM-4).
- **Whether a deactivated field's stored values travel in backup/restore and in the wide XLSX export** (IX-5, IX-12).
- **Deactivating a field while an import is in flight** — no concurrency or conflict behaviour exists.

## Anti-patterns — never reproduce

1. **Tautological guard tests:** the pack-widget guard test never calls product code — it asserts the precondition and raises the expected error itself, so it would pass with the guard deleted. Dispatch the real command and assert the real refusal.
2. **Reactivation by direct row mutation:** several tests flip the active marker on the object directly instead of going through a command — no audit row, no validation, and the test cannot tell a real command path from a missing one.
3. **Soft delete as scattered attribute flips:** logic duplicated across a helper, a service method and raw test mutations, each with its own error behaviour. Exactly one command per resource class; everything dispatches by command id.
4. **Transport error types as domain errors:** one resource class signals refusal with a transport exception, another with a plain value error, English-only. Refusals are `CommandError` values rendered through i18n.
5. **Mislabeled idempotency:** the "uninstall twice is idempotent" test asserts that the second call *raises*, contradicting the merge-only idempotency the rest of the product relies on. State the contract once — a clean no-op reporting "already inactive", or a refusal — and make the name match.
6. **Identity keyed on mutable display strings:** import reactivates a widget by *title* and a mode by *name*, so renaming or translating silently creates a duplicate. Match on stable machine keys; display strings are labels.
7. **Import silently overwriting user-edited metadata:** reactivating a field replaces the user's label and category with the template's, unreported. Report metadata conflicts and require an explicit overwrite choice, or at minimum audit and surface the change.
8. **Ownership modelled by a magic sentinel string** in an id column.
9. **Status computed in the caller:** the three-state install status is assembled ad hoc by the test loop; any other caller can assemble it differently. One read command returns it, so UI, CLI and AI agree.
10. **Uninstall atomicity unverified:** uninstall touches four resource classes and asserts only final counts — a partially applied uninstall still passes if the failure lands after the counters are computed.
11. **Duplicate coverage masquerading as new coverage:** the "toggle blocked" test calls the same delete path as the "cannot be deactivated" test, so the toggle guard itself is untested.
12. **Deactivation tested only at the storage layer:** nearly every exclusion claim is asserted against a raw table read, never against the surfaces a user or agent actually sees.
13. **No audit trail on destructive-ish operations:** nothing asserts that deactivation, reactivation, install or uninstall records who did what and when.
