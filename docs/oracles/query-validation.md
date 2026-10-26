# Oracles — read-only query validation (`midata-queryguard`)

> **Extracted:** 2026-10-26 · **Method:** `REBUILD_PLAN.md` Appendix D — one owner-authorized session read
> **only** the legacy test files and transcribed test **names + asserted behaviours**. No code, API shape,
> endpoint path, private helper or legacy file name is reproduced. **The firewall exception was used and
> expired with that session** (AGENTS.md rule 5; `docs/gaps.md` P0-11 closed).
>
> **Scenarios:** 71 → **52 kept, 19 waived** (the legacy dual SQL/DSL surface plus two anti-oracles).
> The string-literal false-positive case lives in the extensions module (EXC-53), not here.
> **Inventory ids:** `docs/inventory.md` — this file is the acceptance set for **AI-6…AI-9, AI-19** and DB-9.
>
> Scenario ids (`QGV-nn`) are the stable handle: cite them in an inventory row's `evidence` column.
> Our contract is stricter than legacy on three points, stated up front: **decision is made on the parsed
> statement, never on the first keyword**; **`SELECT *` is expanded against the whitelist, never skipped**;
> **filter values are bound parameters, never spliced into the statement text**.

## Accepted shapes

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| QGV-01 | tool-query | `test_simple_select` | Plain projection over the canonical read surface → accepted, query returned unchanged. | AI-6 |
| QGV-02 | tool-query | `test_select_with_where` | Projection with a filter → accepted, filter preserved. | AI-6 |
| QGV-03 | tool-query | `test_select_with_aliases` | Aggregated projection with an alias → accepted, alias not stripped. | AI-6 |
| QGV-04 | tool-query | `test_with_cte` | A single common-table-expression block wrapping one read → accepted. | AI-6 |
| QGV-05 | tool-query | `test_multiple_ctes` | Two CTE blocks joined in the outer read → accepted. | AI-6 |
| QGV-16 | tool-query | `test_allows_trailing_semicolon` | A single read with exactly one trailing semicolon → accepted. | AI-6 |
| QGV-17 | tool-query | `test_select_with_complex_where` | Projection with AND/OR and parenthesised predicates → accepted. | AI-6 |
| QGV-18 | tool-query | `test_select_with_subquery` | Read with a scalar subquery in the filter → accepted. | AI-6 |
| QGV-19 | tool-query | `test_select_with_order_by_limit` | Projection with ordering and a limit → accepted, ordering preserved. | AI-6 |
| QGV-20 | tool-query | `test_select_with_group_by_having` | Projection with grouping and a having clause → accepted. | AI-6 |
| QGV-21 | tool-query | `test_select_with_window_function` | Projection with a window function → accepted. | AI-6 |
| QGV-34 | tool-query | `test_distinct_columns` | DISTINCT projection → column extracted, DISTINCT accepted. | AI-7, AI-6 |
| QGV-51 | tool-query | `test_with_limit` | An explicit limit supplied by the caller → applied verbatim **and clamped to the row cap** (legacy never clamped — see anti-pattern 6). | DB-9, AI-8 |

## Rejected shapes

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| QGV-06 | tool-query | `test_rejects_ddl` | A statement dropping a table → rejected: statement is not read-only. | AI-6, AI-19 |
| QGV-07 | tool-query | `test_rejects_ddl_create` | A statement creating a table → rejected: not read-only. | AI-6, AI-19 |
| QGV-08 | tool-query | `test_rejects_ddl_alter` | A statement altering a table → rejected: not read-only. | AI-6, AI-19 |
| QGV-09 | tool-query | `test_rejects_dml_delete` | A delete statement → rejected: not read-only. | AI-6, AI-19 |
| QGV-10 | tool-query | `test_rejects_dml_update` | An update with a tautological predicate → rejected: not read-only. | AI-6, AI-19 |
| QGV-11 | tool-query | `test_rejects_dml_insert` | An insert statement → rejected: not read-only. | AI-6, AI-19 |
| QGV-12 | tool-query | `test_rejects_multi_statement` | A read followed by a second statement after a semicolon → rejected: more than one statement. | AI-6, AI-19 |
| QGV-13 | tool-query | `test_rejects_empty` | Empty input → rejected: empty input. | AI-6 |
| QGV-14 | tool-query | `test_rejects_whitespace_only` | Whitespace and newlines only → rejected: empty input. | AI-6 |
| QGV-15 | tool-query | `test_ddl_with_cte_raises` | A CTE block followed by a write → rejected: not read-only (write smuggled behind a CTE). | AI-6, AI-19 |
| QGV-36 | tool-query | `test_unknown_column_rejected` | Projection naming one column that is not in the active whitelist → rejected: column not in the whitelist. | AI-7, AI-19 |
| QGV-37 | tool-query | `test_multiple_unknown_columns` | Two unknown columns → rejected, and **both** offending names are reported (legacy matched either one). | AI-7, AI-19 |
| QGV-39 | tool-query | `test_function_result_invalid` | An aggregate over an unknown column → rejected: column not in the whitelist. | AI-7, AI-19 |
| QGV-41 | tool-query | `test_validate_sql_rejects_dml` | Composed validation on a write → rejected: not read-only. | AI-6, AI-19 |
| QGV-42 | tool-query | `test_validate_sql_rejects_bad_column` | Composed validation with an unknown column → rejected: column not in the whitelist. | AI-7, AI-19 |
| QGV-45 | tool-query | `test_invalid_field_rejected` | A widget query naming an unknown field → rejected: column not in the whitelist. | DB-9, AI-7 |
| QGV-63 | tool-query | `test_injection_attempt_rejected` | A read plus a second statement attempting a destructive write → rejected before execution. | AI-19, AI-6 |
| QGV-64 | tool-query | `test_injection_ddl_rejected` | A destructive statement on its own → rejected before execution. | AI-19, AI-6 |
| QGV-65 | tool-query | `test_empty_sql_rejected` | Empty query text at the execution boundary → rejected (the gate is not bypassable by calling the executor directly). | AI-6 |

## Column extraction & whitelist

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| QGV-23 | tool-query | `test_simple_columns` | Projection of three plain columns → all three extracted for whitelisting. | AI-7 |
| QGV-24 | tool-query | `test_function_call_columns` | An aggregate over columns → the argument columns are extracted; function names are not treated as columns. | AI-7 |
| QGV-25 | tool-query | `test_qualified_columns` | A source-qualified projection → the qualifier is stripped and the bare column name is checked. | AI-7 |
| QGV-26 | tool-query | `test_aggregate_with_alias` | An aggregate with an alias → the source column is extracted; the alias is not treated as a column reference. | AI-7 |
| QGV-27 | tool-query | `test_wildcard_skipped` | A wildcard projection → legacy extracts **zero** column references, making the whitelist vacuous. **Our contract: a wildcard is expanded against the whitelist** (or rejected), never silently skipped. | AI-7 |
| QGV-28 | tool-query | `test_where_column_extracted` | A column referenced only in the filter → still extracted for whitelisting. | AI-7 |
| QGV-29 | tool-query | `test_multiple_functions` | Four aggregates over four columns → all four argument columns extracted. | AI-7 |
| QGV-30 | tool-query | `test_nested_functions` | Nested function call over a column → the innermost column is extracted. | AI-7 |
| QGV-31 | tool-query | `test_count_star_skipped` | A count over a wildcard → no column reference is emitted (the wildcard token is not a column). | AI-7 |
| QGV-32 | tool-query | `test_case_expression` | A conditional expression over a column → the projected column and the columns referenced inside the branches are extracted. | AI-7 |
| QGV-33 | tool-query | `test_window_function` | A window function partitioned by a column → projected columns extracted; partition and ordering columns must be whitelisted too (legacy asserted this only weakly). | AI-7 |
| QGV-35 | tool-query | `test_all_columns_valid` | A projection whose columns are all in the active whitelist → accepted. | AI-7 |
| QGV-38 | tool-query | `test_function_result_valid` | An aggregate over a whitelisted column → accepted. | AI-7 |
| QGV-40 | tool-query | `test_validate_sql_combined` | The read-only gate and the whitelist gate composed into one entry point → accepted, query returned unchanged. | AI-6, AI-7 |
| QGV-71 | tool-query | `test_valid_columns_function` | Whitelist enumeration → identity columns and active field keys are present, read from the live field registry rather than a constant list. | AI-7 |

## Result contract & caps

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| QGV-58 | tool-query | `test_raw_sql_execution` | A read executed through the tool path → a result envelope with columns, rows and a row count greater than 0; the projected identity columns are present. | AI-21, AI-6 |
| QGV-59 | tool-query | `test_raw_sql_with_where` | A read limited to one row → exactly one row returned. | AI-6, AI-8 |
| QGV-66 | tool-query | `test_truncation` | A 3-row result → the truncated flag is false and the row count is below the cap. *(The cap boundary itself is never exercised — see fresh.)* | AI-8 |
| QGV-67 | tool-query | `test_empty_results` | A read with an unsatisfiable predicate → zero rows, empty row list, **success not error**. | AI-22 |
| QGV-68 | tool-query | `test_query_tool_executor_sql_path` | A raw ordered read through the tool-call path → the expected row count and identities are present. | AI-6 |

## Waived (9)

| id | legacy test | why waived |
|---|---|---|
| QGV-22 | `test_rejects_union` | Named "rejects" but only asserts nothing crashed; legacy never decided UNION. Waived **as written**; our rule is explicit — UNION/INTERSECT/EXCEPT are accepted only if every branch passes the read-only gate *and* the whitelist (see fresh) |
| QGV-43, 44, 46…50, 52…55, 60…62 | the structured query-plan surface (`test_basic_select`, `test_select_all`, `test_with_where_clause`, `test_with_in_where`, `test_with_is_null_where`, `test_with_group_by`, `test_with_order_by`, `test_with_aggregate`, `test_invalid_from_rejected`, `test_invalid_operator_rejected`, `test_empty_select_rejected`, `test_dsl_basic_select`, `test_dsl_with_aggregate`, `test_dsl_with_where`) | The legacy product shipped **two** query languages (raw SQL *and* a structured plan). The rebuild ships one validated SQL path (AI-6). The behaviours worth keeping are covered elsewhere: unknown field → QGV-45, caller limit clamping → QGV-51, unknown source and empty projection → fresh |
| QGV-56 | `test_sql_injection_where_value_escaped` | Waived as an **anti-oracle**: legacy splices the filter value into the statement text and the test asserts the hostile payload appears verbatim. We bind parameters and assert the statement text is unchanged |
| QGV-57 | `test_entities_source` | Only meaningful with the legacy plan surface's two hardcoded sources |
| QGV-69 | `test_execution_time_ms_in_response` | Asserts the *absence* of a timing field, encoding a layering decision as a test; timing is measured in the AI layer |
| QGV-70 | `test_with_cte_query` | Waived: the legacy query needed a cast to compare a text-encoded numeric column. Our observations are typed at write time, so no in-query casting is needed |

## Accept/reject matrix (our validation test matrix)

**Accept:** plain projection · filter with AND/OR and parentheses · ordering + limit (clamped) · grouping + having · window functions · aggregate with alias · DISTINCT · scalar subquery in the filter · single CTE · multiple CTEs joined · one trailing semicolon · wildcard **expanded against the whitelist** · function-wrapped and nested column references · qualifier-prefixed references · read keyword inside a string literal as data · zero rows.

**Reject:** DDL (drop/create/alter) · DML (delete/update/insert) · write smuggled behind a CTE · two statements · empty string · whitespace only · empty query at the execution boundary · column outside the whitelist (in the projection, in a function argument, in the filter, in a partition/order clause) · wildcard that cannot be expanded · unknown data source · unknown operator · empty projection with no aggregate.

**Reject with a stable reason code** (our addition — legacy rejections carried only a human message): not read-only · more than one statement · empty input · column not whitelisted · unknown source · unknown operator · empty projection · row cap exceeded · result size cap exceeded · timeout.

## Fresh — write from `REBUILD_PLAN` (no legacy coverage)

Untested attack classes and caps. These are the reason this file cannot be treated as a complete acceptance set.

- **Comment-based smuggling:** `--` and `/* */` (including nested block comments) used to split a keyword, truncate a statement tail, or hide a second statement.
- **String-literal false positives:** a legitimate literal *containing* a keyword or semicolon must be accepted as data — legacy's only literal test asserts the opposite (interpolation).
- **Union semantics:** UNION/INTERSECT/EXCEPT with a write branch, or over a non-whitelisted source.
- **Subquery writes:** a write hidden in a subquery or a data-modifying CTE (a CTE that deletes and returns rows).
- **System and schema surfaces:** schema catalogues, `PRAGMA`-like tables, schema-qualified names, and a **real but forbidden application table** (legacy only rejected a *nonexistent* source).
- **`PRAGMA` / `ATTACH` / `VACUUM` / `SELECT INTO` / `COPY` / `EXPORT`** and any file-reading or file-writing function — a wide-open class with zero legacy coverage.
- **Row-cap boundary:** exactly the cap, cap + 1, and a caller limit above the cap. Legacy asserted only that truncation was *off* on three rows.
- **Result-size cap** (50 KB) and an oversized single cell.
- **Summarization caps:** the raw-row threshold, the aggregated-row cap, and 200-character string truncation (AI-8).
- **Per-tool timeout:** slow query, cancellation, timeout reason code (AI-9).
- **Parameter placeholders:** `?`, `:name`, `$1`, `@name` — and that binding is used instead of interpolation.
- **Non-string input:** none, integer, bytes, list, map where text is expected.
- **Unicode and encoding tricks:** homoglyph keywords, NUL bytes, BOM, zero-width characters, non-ASCII whitespace, case-folded write keywords.
- **Quoted-identifier breakout:** a field key or alias containing a quote character; aliases that shadow a forbidden identifier (legacy exempts aliases from the whitelist entirely).
- **Whitelist completeness:** ordering and having references outside the projection; a column that exists but belongs to a deactivated field (must be rejected — HM-22 × AI-7).
- **Path parity:** the same input is rejected identically whichever command path it enters, and every execution path validates before executing.
- **Purity:** validation is deterministic, has no side effects, and touches no data.
- **Determinism of the reason code:** same input → same reason code, always.

## Anti-patterns — never reproduce

1. **UNION accepted by accident:** the test named "rejects union" asserts only that nothing crashed; a union of two reads is silently accepted with no second whitelist pass.
2. **Wildcard bypasses the column whitelist:** `SELECT *` and count-over-wildcard extract zero references, so the whitelist is vacuous on both paths.
3. **Statement-type validation by keyword or substring matching rather than structure:** rejections are matched against fragments of an English error message. Decide on the AST.
4. **Source allowlist as a hardcoded pair of table names:** a nonexistent source is rejected but a real-but-forbidden table is never tested. The allowlist is registry-driven (identity columns + active fields + the canonical read surface).
5. **Values interpolated into query text instead of bound:** the "injection" test asserts the hostile payload appears verbatim and merely counts quote characters.
6. **Aggregate function name as an unvalidated free string** — a second injection surface behind the projection list. Allowlist the aggregate functions.
7. **Caps never exercised:** the only cap test asserts the flag is *off*; a cap that is never reached is a cap that is never proven. Size cap and timeout are absent entirely.
8. **Aliases exempt from the whitelist:** any identifier, including a forbidden one, can be introduced into the result shape unvalidated.
9. **Rejection contract carries no reason class:** every rejection is a bare exception with a human message, so agents cannot branch on *why*.
10. **Docstring/assertion drift:** one test promises "all unknown columns reported" but matches either name; another named for a filter actually exercises a limit. Names must state the boundary actually exercised.
11. **Transport concerns asserted inside the executor** (asserting the absence of a timing field), encoding a layering decision as a test.
12. **Two query languages in the tool surface** (SQL *and* a DSL): doubled validation, doubled documentation, two ways to be wrong.
13. **Legacy internal table names exposed as enum values** to the model — schema detail leaking into the model-facing contract.
14. **A test that imports a private whitelist helper** and a fixture that materialises a database view before querying — validation must be testable through the public command surface, with fixtures from `midata-testkit`.
