# Oracles — AI assistant, provider settings, conversations & widget proposals

> **Extracted:** 2026-10-26 · **Method:** `REBUILD_PLAN.md` Appendix D — one owner-authorized session read
> **only** the legacy test files and transcribed test **names + asserted behaviours**. No code, API shape,
> endpoint path, event framing, tool-name spelling or legacy file name is reproduced. **The firewall
> exception was used and expired with that session** (AGENTS.md rule 5; `docs/gaps.md` P0-11 closed).
>
> **Scenarios:** 75 (chat 24 · settings 20 · conversations 19 · widget 12) → **73 kept, 2 waived**;
> 4 legacy-mechanism notes.
> **Inventory ids:** `docs/inventory.md`.
>
> Scenario ids (`AST-nn`) are the stable handle: cite them in an inventory row's `evidence` column.
> **Three rebuild differences govern every mapping here:** (a) AI writes are **staged in SQLite** with a
> single-use token and 5-minute expiry, surviving restart, and execute only on explicit confirmation;
> (b) the API key lives in the **OS keyring**, never a settings file or the database; (c) the assistant
> **mirrors the user's language** — nothing is forced.

## Turn events & tool dispatch

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| AST-01 | chat | `test_chunk_event` | Assistant emits a text delta → exactly one streaming chunk event carrying that text reaches the panel, appended in order. | AI-1, AI-2 |
| AST-02 | chat | `test_chunk_event_arabic` | The assistant delta is non-Latin (Arabic) → the chunk arrives intact: no transliteration, no dropped characters, no split at a non-ASCII boundary. | AI-1, AI-2 |
| AST-03 | chat | `test_tool_call_event` | The model requests a tool → a tool-call event with the call id, the tool id and its arguments is emitted before any result for that call. | AI-2, AI-5 |
| AST-04 | chat | `test_tool_status_event` | A tool call begins executing → a status event for that call id reports a running state and carries no error. | AI-2 |
| AST-05 | chat | `test_tool_status_error` | A tool call fails → a status event for that call id reports failure with an i18n'd reason and the turn continues instead of aborting. | AI-2, AI-20 |
| AST-06 | chat | `test_tool_result_event` | A tool returns data → a result event carries a one-line summary, a preview row count and the preview payload. | AI-2, AI-8 |
| AST-07 | chat | `test_confirmation_event` | A write tool stages a plan → a confirmation event carries an opaque token plus a human-readable description of the pending change, and **nothing executes during the turn**. | AI-2, AI-10, AI-11 |
| AST-08 | chat | `test_done_event` | The turn completes normally → exactly one terminal done event with an empty payload closes the stream. | AI-2 |
| AST-09 | chat | `test_error_event` | The turn fails → exactly one terminal error event carries an i18n'd message, never raw exception text or stack detail. | AI-2, AI-20 |
| AST-10 | chat | `test_all_events_have_correct_format` | Any turn, any outcome → every event is well-formed and the turn terminates with exactly one terminal event (done xor error) — never both, never neither. | AI-2 |
| AST-11 | chat | `test_has_all_tools` | The assistant is offered its tools → the offerable set is **exactly the command-registry projection**; adding a command makes it offerable with no other change. | AI-5 |
| AST-12 | chat | `test_query_tool_has_type_enum` | The read-only query tool is described to the model → it declares which query language it accepts. | AI-6 |
| AST-13 | chat | `test_modify_tool_has_changes` | The write tool is described → its argument is a **list** of proposed changes, so a write can never be one free-form blob. | AI-10 |
| AST-14 | chat | `test_batch_tool_has_operations` | The batch tool is described → it declares a list of operations plus an all-or-nothing versus per-item mode. | AI-13 |
| AST-15 | chat | `test_ui_tool_has_action_enum` | The navigation tool is described → it declares a closed set of navigation actions plus a params object, dispatched by command id, carrying no domain logic. | AI-23 |
| AST-16 | chat | `test_query_tool_has_sql_param` | The query tool is described → it accepts a single validated statement as a string parameter. | AI-6 |
| AST-19 | chat | `test_has_query_executor` | A read-only query tool is registered → dispatching that tool id reaches the validated query path and returns data. | AI-5, AI-6 |
| AST-20 | chat | `test_unknown_tool_raises` | The model calls a tool id not in the registry → the turn reports an i18n'd "unknown tool" status error, records it in the transcript, executes nothing, and does not crash. | AI-5, AI-20 |
| AST-21 | chat | `test_generate_summary_with_rows` | A tool returns 5 rows over 3 columns → the result summary states the row count and names the columns. | AI-8 |
| AST-22 | chat | `test_generate_summary_singular` | A tool returns exactly 1 row → the summary uses the singular form, i18n'd, correct in both EN and FR. | AI-8, SY-5 |
| AST-23 | chat | `test_generate_summary_truncated` | A tool result hits the row cap and is flagged truncated → the summary says so and elides the column list after a fixed count with "and N more". | AI-8 |
| AST-24 | chat | `test_generate_summary_no_columns` | A tool returns 0 rows and no columns → the summary states zero rows and omits the column clause entirely (no dangling separator). | AI-8 |

## Provider settings & secret handling

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| AST-25 | settings | `test_round_trip_normal_key` | A normal API key is saved, then read back for a provider call → the identical key value is used. | AI-16 |
| AST-26 | settings | `test_round_trip_with_special_chars` | A key containing punctuation and symbols is saved and read back → returned byte-identical, no escaping, trimming or mangling. | AI-16 |
| AST-27 | settings | `test_round_trip_long_key` | A very long key (~500 characters) is saved and read back → unchanged and not truncated. | AI-16 |
| AST-28 | settings | `test_encrypt_empty_string_returns_empty` | The user saves an empty key field → the stored secret stays unset; nothing is written. | AI-16 |
| AST-29 | settings | `test_encrypt_none_becomes_empty` | The key field is absent from the submitted settings → treated as unset, no crash, no placeholder stored. | AI-16 |
| AST-30 | settings | `test_decrypt_empty_string_returns_empty` | The key is read while none is configured → an unset result, no error shown. | AI-16 |
| AST-31 | settings | `test_decrypt_none_becomes_empty` | The key was never configured and the reader reports absence → the caller sees "not configured", never a crash. | AI-16 |
| AST-32 | settings | `test_fernet_key_is_deterministic` | The stored key is read twice in one session → both reads resolve the same secret; no per-read re-keying. | AI-16 |
| AST-33 | settings | `test_encrypt_deterministic_round_trip` | The same key is saved twice → both reads still return the original plaintext; re-save is idempotent. | AI-16 |
| AST-34 | settings | `test_corrupted_ciphertext_returns_empty` | The stored key material is unreadable garbage → the app reports "not configured", does not crash, and never surfaces the raw stored bytes. | AI-16, AI-20 |
| AST-35 | settings | `test_tampered_ciphertext_returns_empty` | The stored key material was altered and fails its integrity check → the same graceful "not configured" outcome; the next provider call fails with an i18n'd auth error, not a panic. | AI-16, AI-20 |
| AST-36 | settings | `test_sentinel_keeps_existing_key` | The provider settings form is re-saved without the user retyping the key → the existing key is preserved, not overwritten. | AI-15, AI-16 |
| AST-37 | settings | `test_sentinel_discarded_when_row_missing` | The form is submitted without a key while no key exists → no key is created and no placeholder becomes a stored secret. | AI-16 |
| AST-38 | settings | `test_sentinel_discarded_when_stored_value_empty` | The form is submitted without a key while the stored key is empty → the key stays empty. | AI-16 |
| AST-39 | settings | `test_new_key_replaces_existing` | A real new key is saved over an existing one → subsequent provider calls use the new key and the old one is gone. | AI-16 |
| AST-40 | settings | `test_empty_string_clears_key` | The user explicitly clears the key field and saves → the stored key is removed and the assistant reports "not configured" until a new key is entered. | AI-16 |
| AST-41 | settings | `test_get_decrypted_all_returns_plaintext` | All provider settings configured → base URL, model, temperature and key all resolve with their exact saved values, ready to build a request. | AI-15, AI-16 |
| AST-42 | settings | `test_get_decrypted_all_empty_db` | No provider settings have ever been saved → every setting resolves to empty or its default with no error (clean first-run state). | AI-15, AI-16 |
| AST-43 | settings | `test_get_all_settings_masks_secret` | The settings screen is opened with a key configured → the key is reported as configured; its plaintext never crosses the command boundary. | AI-16 |
| AST-44 | settings | `test_get_all_settings_empty_secret_stays_empty` | The settings screen is opened with no key configured → the field reads as unset, not as a masked value (no false "a key exists" signal). | AI-16 |

## Conversations

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| AST-45 | conversations | `test_create_conversation` | A conversation is created with a title → it persists with that title and a zero message count. | AI-3 |
| AST-46 | conversations | `test_create_conversation_default_title` | A conversation is created with no title → it receives an i18n'd default title in EN and FR, never an empty title. | AI-3, SY-5 |
| AST-47 | conversations | `test_list_conversations` | Two conversations exist → both are listed, most recently updated first. | AI-3 |
| AST-48 | conversations | `test_delete_conversation` | A conversation is deleted → it is absent from subsequent lists. | AI-3 |
| AST-49 | conversations | `test_get_messages_empty` | A conversation with no messages is opened → an empty transcript, no error, no placeholder message. | AI-3 |
| AST-50 | conversations | `test_get_messages_with_data` | A conversation holds user, assistant and user messages → all three are returned in send order with roles and content intact. | AI-3 |
| AST-51 | conversations | `test_cascade_delete_messages` | A conversation holding messages is deleted → its messages are deleted with it, nothing orphaned. | AI-3 |
| AST-52 | conversations | `test_message_optional_fields` | An assistant message that made no tool calls is stored → its tool-call and tool-result metadata is absent and the message still renders. *(Legacy asserted nothing here — see anti-pattern 5.)* | AI-3, AI-2 |
| AST-53 | conversations | `test_nonexistent_conversation_messages_404` | Messages are requested for a conversation id that does not exist → an explicit not-found outcome, never a crash; separately, a message that did use tools round-trips its tool-call and tool-result metadata across a reload. | AI-3, AI-2 |
| AST-54 | conversations | `test_list_conversations_api` | The conversation list is requested → success with a list, empty when nothing exists. | AI-3 |
| AST-55 | conversations | `test_create_conversation_api` | A conversation is created with a title → created and returned with a stable id and the given title. | AI-3 |
| AST-56 | conversations | `test_create_conversation_default_title_api` | A conversation is created with no title → created with the i18n'd default title. | AI-3, SY-5 |
| AST-57 | conversations | `test_update_conversation_api` | An existing conversation is renamed → the new title is returned and persisted. | AI-3 |
| AST-58 | conversations | `test_update_nonexistent_conversation_api` | Renaming a conversation id that does not exist → a clean not-found error, no partial write, no crash. | AI-3, AI-20 |
| AST-59 | conversations | `test_delete_conversation_api` | An existing conversation is deleted → confirmed gone from the subsequent list. | AI-3 |
| AST-60 | conversations | `test_delete_nonexistent_conversation_api` | Deleting a conversation id that does not exist → a clean not-found error, not a crash. | AI-3, AI-20 |
| AST-61 | conversations | `test_get_messages_api` | An existing empty conversation's messages are requested → success with an empty transcript. | AI-3 |
| AST-62 | conversations | `test_get_messages_nonexistent_conv_api` | Messages requested for a non-existent conversation → an explicit not-found error, not a silent empty success that hides the mistake. | AI-3 |
| AST-63 | conversations | `test_chat_without_conversation_id_creates_new` | A chat turn is sent with no conversation id → a conversation is created first and its id is announced before any content event; if the provider then fails, an i18n'd error event ends the turn and the conversation is still persisted. | AI-3, AI-1, AI-20 |

## Widget proposals

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| AST-64 | widget | `test_bar_chart_with_valid_sql` | A proposal for a bar chart over a validated aggregate query (average weight by breed) → the proposal returns title, chart kind, result columns and rows, a non-zero row count, the validated query and the axis mapping. | AI-18, DB-3 |
| AST-65 | widget | `test_kpi_card_widget` | A proposal for a single-value KPI card → at least one row is returned and the value column is mapped. | AI-18, DB-1 |
| AST-66 | widget | `test_table_widget` | A proposal for a table of identifiers plus a numeric observation → multiple rows and exactly the requested columns. | AI-18, DB-5 |
| AST-67 | widget | `test_line_chart_widget` | A proposal for a line chart → rows are returned and both axes are mapped. | AI-18, DB-2 |
| AST-68 | widget | `test_pie_chart_widget` | A proposal for a pie chart over grouped counts → grouped rows returned and label/value keys mapped. | AI-18, DB-4 |
| AST-69 | widget | `test_missing_title_raises` | A proposal with no title → rejected with an i18n'd "missing title" error naming the field; nothing is saved. | AI-18, AI-20 |
| AST-70 | widget | `test_missing_template_type_raises` | A proposal with no chart kind → rejected with an i18n'd "missing chart kind" error; nothing is saved. | AI-18, AI-20 |
| AST-71 | widget | `test_missing_sql_raises` | A proposal with no query → rejected with an i18n'd "missing query" error; nothing is saved. | AI-18, AI-20 |
| AST-72 | widget | `test_missing_mapping_raises` | A proposal with no field mapping → rejected with an i18n'd "missing mapping" error; nothing is saved. | AI-18, AI-20 |
| AST-73 | widget | `test_invalid_template_type_raises` | A proposal naming a chart kind that does not exist → rejected with an i18n'd "unknown chart kind" error listing the valid kinds; nothing is saved. | AI-18, AI-20 |
| AST-74 | widget | `test_invalid_sql_not_select_raises` | A proposal whose query changes data → rejected by the validation lab before execution with an i18n'd "read-only queries only" error; the database is untouched. | AI-6, AI-18, AI-19 |
| AST-75 | widget | `test_invalid_sql_syntax` | A proposal whose query does not parse → rejected with an i18n'd validation error carrying no raw engine text; nothing is saved or executed. | AI-6, AI-18, AI-19 |

## Waived (9)

| id | legacy test | why waived |
|---|---|---|
| AST-17 | `test_query_tool_has_dsl_param` | The rebuild has no DSL layer — one validated SQL path (AI-6) |
| AST-18 | `test_query_tool_requires_type` | The query-language discriminator exists only because of the legacy SQL/DSL duality |
| note · AST-25…33 | encryption round-trip mechanics (deterministic key derivation, re-encrypt idempotency) | The **behaviour** (a saved key round-trips byte-identically; re-save is idempotent) is kept in AI-16, but a secret encrypted into the application database with a key derived from a hard-coded constant is waived — the keyring is the only secret store (rule 14) |
| note · AST-36…38 | the masked-sentinel round-trip protocol | Kept as behaviour (re-saving without retyping preserves the key; a placeholder never becomes a secret), but a magic mask string travelling UI → settings → "keep the old value" is waived: we render explicit configured / not-configured state and never echo a secret |
| note · AST-08/10 | server-sent-event framing (`event:`/`data:` lines, blank-line terminator) | In-process event stream instead; the *terminal-event invariant* (AST-10) is kept |
| note · AST-63 | "HTTP 200 carrying an error event" | The conversation-still-persisted behaviour is kept; the transport status semantics are waived |

## Failure-mode oracles (the set our rebuild must keep)

- **Tool failure** → a failed status event for that call id with an i18n'd reason; the turn does not abort.
- **Unknown tool id from the model** → i18n'd error, nothing executed, no crash.
- **Provider failure mid-turn** → exactly one terminal error event, i18n'd, no raw exception text; the conversation already created is still persisted.
- **Terminal-event invariant** → every turn ends with exactly one of done/error, never both, never neither.
- **Key absent / never configured** → not-configured state on read and on screen; no placeholder persisted; no crash.
- **Key explicitly cleared** → key removed, assistant reports not configured until re-entered.
- **Stored key unreadable (corrupt)** → degrade to not configured, never raise, never print stored bytes.
- **Stored key tampered (integrity failure)** → same graceful degradation; use fails with an i18n'd auth error.
- **Placeholder re-submitted with no key present** → discarded, no secret created.
- **Empty provider settings store** → all settings resolve to empty or default, no error.
- **Empty conversation transcript** → empty list, no error, no placeholder message.
- **Unknown conversation id — read messages / rename / delete** → explicit not-found, no partial write, no crash.
- **Conversation deleted while holding messages** → messages cascade away, nothing orphaned.
- **Widget proposal missing title / chart kind / query / mapping** → i18n'd field-specific rejection, nothing saved.
- **Widget proposal with an unknown chart kind** → i18n'd rejection listing valid kinds.
- **Widget query is not read-only, or does not parse** → rejected pre-execution, database untouched, no raw engine text.

## Fresh — write from `REBUILD_PLAN` (no legacy coverage)

The highest-value fresh work in the whole extraction — this is where the rebuild's safety model lives.

- **Staging semantics (AI-10):** no legacy test asserts that a write tool executes *nothing* during the turn; only the confirmation event's shape is checked.
- **Staged-plan persistence and restart survival (AI-11, AI-12):** no restart, no reload, no "the plan is still there after the app is closed and reopened" oracle anywhere.
- **Single-use token:** no test asserts that a second confirmation with the same token is rejected.
- **Token expiry:** no test advances a clock past 5 minutes; expired-token confirmation is entirely unspecified.
- **Confirmation outcomes (AI-13):** transactional versus per-item commit and the cancel path are untested; no partial-failure behaviour (one item failing mid-batch) is defined.
- **Audit provenance (AI-14):** no test asserts an audit row per confirmed change, that it carries the batch token, or that the actor is recorded as AI (SY-7).
- **Undo of AI-applied writes.**
- **Keyring specifics (AI-16):** keyring unavailable, keyring locked, permission denied, entry removed outside the app.
- **Connectivity test command (AI-16):** unreachable base URL, wrong model name, 401 on an invalid key, TLS failure, slow endpoint and timeout.
- **Provider request parameters:** temperature out of range, negative or non-numeric; a model name that does not exist.
- **Context governance (AI-17):** the last-20-message window, summarisation of older turns, the 32k budget, the 24k warning — nothing at all.
- **Per-tool timeout (AI-9):** a tool that hangs forever.
- **Cancellation mid-stream:** is the partial assistant message kept, dropped or marked interrupted?
- **Malformed model output:** tool arguments that are not valid JSON, a call with missing required arguments, a call for the wrong tool, two calls in one delta.
- **Column whitelist (AI-7):** legacy blocks only non-read statements and syntax errors; no oracle for a read that touches a non-whitelisted column.
- **Injection attempts (AI-19):** multi-statement queries, comment and union tricks, and prompt-injected SQL.
- **Caps enforcement (AI-8):** only summary *formatting* is tested; nothing proves the 5,000-row / 50 KB / 200-character truncation actually happens.
- **Widget proposal lifecycle (AI-18):** preview-then-save, the saved widget being marked AI-sourced, rejecting a proposal, and the widget subsequently hydrating from its stored data source (DB-9, DB-13).
- **Language mirroring:** no test asserts the assistant answers in the user's language.
- **i18n of assistant output:** error and summary strings are free-form English literals in legacy; FR coverage and the no-raw-exception rule are unasserted.
- **Concurrent turns:** two conversations streaming at once, or a second send while one is running.
- **Empty or whitespace-only user message.**
- **Conversation deleted during an active turn:** dangling writes, orphan messages.

## Anti-patterns — never reproduce

1. **Secret-encryption key derived from a hard-coded application constant.** The keyring is the only secret store.
2. **API key stored as an encrypted column in the application database**, alongside farm data and backups.
3. **The masked-sentinel round-trip protocol:** a magic mask string travelling UI → settings → "keep the old value", leaking "a value exists" into a string comparison.
4. **Vacuous or tautological assertions:** list ordering asserted as "first is A *or* B"; a sentinel test whose clause is always true; "None bypass" tests passing an empty string and therefore asserting nothing about absence.
5. **Empty test body:** `test_message_optional_fields` has a docstring and no assertions — it can never fail.
6. **Misleading test name:** a test named for a not-found outcome asserts an empty list, then appends two unrelated assertions about tool metadata.
7. **Hard-coded tool count and hand-listed tool names:** directly contradicts a registry-derived tool list (AI-5); a pinned number blocks every future command.
8. **Test coupled to implementation identity** (asserting a specific executor is the registered handler).
9. **Module-level global cache patched by tests**, plus a fixture dance to work around the analytics engine reading the database out-of-band.
10. **Two query languages in the tool surface** (SQL *and* a DSL) — doubled validation, two ways to be wrong.
11. **Legacy internal table names exposed as model-facing enum values.**
12. **Non-deterministic streaming test:** asserts a conversation id and then shrugs that "there might be an error event (no LLM running)". Stub the provider and assert the terminal event deterministically.
13. **English literals for user-facing strings** (default conversation title, tool summaries, error text) — all must be i18n keys.
14. **Error payloads carrying free-form exception text.**
15. **A confirmation token with no persistence, expiry or single-use contract** — the token exists only as a stream-event field.
16. **Settings as stringly-typed key/value rows** (temperature stored as text) — invites type drift and locale-dependent parsing; persisted settings are typed and serde-defaulted.
17. **Float casts per query** (`CAST(weight AS DOUBLE)` in every test query) — a symptom of inconsistent storage typing. Observations are typed at write time; money stays integer minor units.
