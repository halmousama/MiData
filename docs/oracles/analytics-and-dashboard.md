# Oracles — analytics, studies & dashboard hydration

> **Extracted:** 2026-10-26 · **Method:** `REBUILD_PLAN.md` Appendix D — one owner-authorized session read
> **only** the legacy test files and transcribed test **names + asserted behaviours**. No code, API shape,
> endpoint path, response key name or legacy file name is reproduced. **The firewall exception was used
> and expired with that session** (AGENTS.md rule 5; `docs/gaps.md` P0-11 closed).
>
> **Scenarios:** 51 (analytics 30 · analyze-tool 21) → **51 kept, 0 waived**; 1 legacy-envelope note.
> **Inventory ids:** `docs/inventory.md`. New rows added where none existed are marked in §Proposed rows.
>
> Scenario ids (`ANA-nn`) are the stable handle: cite them in an inventory row's `evidence` column.

## Scenarios

| id | module | legacy test | scenario (input → expected behaviour) | inventory |
|---|---|---|---|---|
| ANA-01 | analytics | `test_trend_returns_chart_data_and_stats` | Two weight readings 100 and 200 on the same date, two animals → the trend collapses each date to one point whose value is the date mean 150; summary count 2, mean 150, min 100, max 200; a regression line and outlier lists are present; insights non-empty. | AN-4, AN-1, AN-5, AN-15 |
| ANA-02 | analytics | `test_trend_with_date_filter` | Same two readings, date window starting in the future → zero points, no error, at least one deterministic insight stating the window is empty. | AN-2, AN-22, AN-5 |
| ANA-03 | analytics | `test_distribution_returns_histogram_bins` | Two weights 100 and 200 → non-empty histogram bins, each carrying a range and an integer count; summary count 2, mean 150; insights non-empty. | AN-6, AN-8, AN-1 |
| ANA-04 | analytics | `test_distribution_single_value` | Exactly one weight reading 150 → exactly one populated bin with count 1, plus a deterministic single-data-point insight. | AN-7, AN-8 |
| ANA-05 | analytics | `test_correlation_returns_paired_data_and_pearson_r` | Weight/height pairs (100, 80) and (200, 150) across two animals → paired points with numeric x and y; per-group summary carries count and Pearson r; overall r = 1.0 (±0.01); insights non-empty. | AN-9, AN-10, AN-1 |
| ANA-06 | analytics | `test_correlation_missing_x_field` | Correlation on weight against a field key that does not exist → empty chart, no crash, deterministic insight identifying that the field was not found. | AN-22, AN-10 |
| ANA-07 | analytics | `test_correlation_without_x_field` | Correlation on weight with no second field supplied → empty chart and an insight stating a second (x) field is required. | AN-15, AN-22 |
| ANA-08 | analytics | `test_comparison_without_group_data_returns_gracefully` | Weight grouped by a dimension that has no recorded values → a valid result whose groups each carry group/count/mean, plus an insight saying the grouping field does not exist; never a crash. | AN-12, AN-13, AN-22 |
| ANA-09 | analytics | `test_comparison_invalid_group_by` | Grouping by an unrecognised dimension → empty chart plus an insight saying the grouping field does not exist or that there is no data. | AN-12, AN-13, AN-22 |
| ANA-10 | analytics | `test_unknown_y_field_returns_empty_chart` | Trend on a field key that does not exist → empty chart and a deterministic "field not found" insight. | AN-22, AN-5 |
| ANA-11 | analytics | `test_unknown_analysis_type_returns_gracefully` | Unrecognised analysis type with a valid field → structured empty result, no exception, at least one insight. | AN-15, AN-22 |
| ANA-12 | analytics | `test_deep_analysis_returns_full_report` | Two weights 100 and 200 → deep-dive report: overview count 2, mean 150, min 100, max 200, sum 300; histogram bins with range and count; per-date mean series; breed and sex breakdown lists (empty for this dataset); high and low outlier lists; insights non-empty. | AN-16, AN-17, AN-18, AN-19, AN-20, AN-21, AN-1 |
| ANA-13 | analytics | `test_deep_analysis_missing_field` | Deep dive on a field key that does not exist → overview count 0 plus a "not found" insight, no crash. | AN-16, AN-22 |
| ANA-14 | analytics | `test_deep_analysis_field_with_no_measurements` | Deep dive on a field that exists but holds zero observations → overview count 0 plus a "no data" insight. | AN-16, AN-22 |
| ANA-15 | analytics | `test_correlation_matrix_returns_correct_shape` | Matrix across two numeric fields with two animals → 2×2 matrix keyed by field; both diagonals exactly 1.0; off-diagonals symmetric within 0.001. | AN-11 |
| ANA-16 | analytics | `test_correlation_matrix_too_few_fields` | Matrix requested for a single field → explicit validation error stating at least 2 fields are required. | AN-11 |
| ANA-17 | analytics | `test_correlation_matrix_empty_data` | Matrix across two field keys with zero observations → error result, never a matrix of undefined values. | AN-11, AN-22 |
| ANA-18 | analytics | `test_correlation_matrix_with_filters` | Two animals each with weight 100 and height 80, filtered by a breed matching no animal → error or empty result rather than a fabricated matrix. | AN-11, AN-2 |
| ANA-19 | analytics | `test_categorical_distribution` | Five animals, a select field Cow 2 / Sheep 1 / Goat 2 → three category rows with counts 2/1/2, three summary groups each with count and percentage, a category insight, no regression line, empty outlier lists. | AN-14, AN-6, AN-8 |
| ANA-20 | analytics | `test_categorical_trend_returns_empty` | Trend requested on a select field → empty chart plus an insight redirecting to the distribution view. | AN-14, AN-5 |
| ANA-21 | analytics | `test_categorical_deep_analysis` | Deep dive on a select field over five animals → overview count 5 with 3 categories, per-category distribution entries, empty time trend, empty outliers, non-empty insights. | AN-14, AN-16, AN-17 |
| ANA-22 | analytics | `test_categorical_text_field_deep_analysis` | Deep dive on a free-text field, Black 2 / White 2 / Brown 1 → overview count 5 with 3 categories; the categorical path is taken, no numeric maths. | AN-14, AN-16 |
| ANA-23 | analytics | `test_categorical_deep_analysis_nonexistent_field` | Deep dive on a field that does not exist → overview count 0 plus a "not found" insight. | AN-16, AN-22 |
| ANA-24 | analytics | `test_dashboard_stats_returns_counts` | Herd of 2 animals with observations → dashboard summary reports a total animal count of at least 2 and a recent-activity count. | DB-10 |
| ANA-25 | analytics | `test_dashboard_stats_with_mode` | Same herd scoped to a mode linking one field → identical structure, numbers scoped to the mode. | DB-11, DB-10 |
| ANA-26 | analytics | `test_dashboard_stats_empty_db` | Empty database → total animals exactly 0, no error. | DB-10, AN-22 |
| ANA-27 | analytics | `test_profitability_returns_structure` | Seeded herd with a numeric production field → a profitability result is returned, no crash. **No numeric or money assertion at all.** | AN-23, AN-24, DB-8 |
| ANA-28 | analytics | `test_profitability_with_empty_data` | Empty database → profitability result returned, no crash, no division by zero. | AN-23, AN-22 |
| ANA-29 | analytics | `test_dynamic_dashboard_returns_widgets` | Herd with data → dashboard hydration returns a widget collection. | DB-9, DB-10 |
| ANA-30 | analytics | `test_dynamic_dashboard_empty` | No widget configuration stored → empty widget collection, not an error. | DB-9, AN-22 |
| ANA-31 | analyze | `test_unknown_function_raises` | Analysis invoked with an unknown analysis name → rejected with an actionable error, no data returned. | AI-20, AN-15 |
| ANA-32 | analyze | `test_missing_function_raises` | Analysis invoked with no analysis name → rejected with an actionable error naming the missing parameter. | AI-20, AN-15 |
| ANA-33 | analyze | `test_returns_correct_columns_and_rows` | 12 weight readings across 8 animals → a fixed 10-bin histogram; every row carries a range, an integer count and a percentage; summary min/max/mean/count with count > 0 and min ≤ max. | AN-6, AN-1 |
| ANA-34 | analyze | `test_weights_sum_to_total_count` | Same 12 readings → the sum of bin counts equals the summary count (no reading dropped, none double-counted). | AN-6 |
| ANA-35 | analyze | `test_filter_by_species` | Filter species = Cow over the same herd → exactly 5 weight readings counted. | AN-2, AN-6 |
| ANA-36 | analyze | `test_filter_no_match` | Filter to a species with no animals in the herd → empty bin list and count 0, no error. | AN-2, AN-22 |
| ANA-37 | analyze | `test_no_weight_field` | No weight field defined at all → empty rows and count 0, no crash. | AN-6, AN-22 |
| ANA-38 | analyze | `test_single_value_produces_single_bucket_of_1` | One single weight reading of 500 → summary count 1, 10 buckets emitted, counts across buckets sum to 1. | AN-7, AN-6 |
| ANA-39 | analyze | `test_returns_expected_structure` (growth trend) | 12 readings across 8 animals and 3 species → per-period rows with period, average, min, max and count; a trend object with direction ∈ {up, down, stable} and a numeric percentage change. | AN-4, AN-5, AN-27 |
| ANA-40 | analyze | `test_period_month_default` | No grouping specified → every period is month granularity. | AN-4, AN-27 |
| ANA-41 | analyze | `test_period_day` | Grouping = day → every period is day granularity. | AN-4, AN-27 |
| ANA-42 | analyze | `test_no_weight_field` (growth trend) | Growth trend with no weight field defined → empty rows and direction reported as stable, never a fabricated trend. | AN-4, AN-22 |
| ANA-43 | analyze | `test_returns_expected_structure` (composition) | 8 animals, 3 species, 2 sexes, mixed ages → rows of species × sex × age group each with count and percentage, plus a total animal count greater than 0. | AN-28 |
| ANA-44 | analyze | `test_percentages_sum_to_100` | Same herd → group percentages sum to 100 (±0.1). | AN-28 |
| ANA-45 | analyze | `test_total_equals_sum_of_counts` | Same herd → the sum of group counts equals the reported total (each animal counted exactly once). | AN-28 |
| ANA-46 | analyze | `test_filter_by_species` (composition) | Filter species = Sheep → total 3 animals and every returned row's species is Sheep. | AN-2, AN-28 |
| ANA-47 | analyze | `test_no_entities` | Empty herd → empty rows and total 0, no error. | AN-22, AN-28 |
| ANA-48 | analyze | `test_returns_expected_structure` (mortality) | 8 animals, one recorded as dead → per-year rows with year, total, deaths and a death-rate percentage, plus an overall rate. | AN-29 |
| ANA-49 | analyze | `test_overall_rate_positive_when_deaths_exist` | One death in an 8-animal herd → overall death rate strictly greater than 0 (exact value unasserted, denominator unspecified). | AN-29 |
| ANA-50 | analyze | `test_no_death_fields` | No death data recorded anywhere → overall rate exactly 0, no division by zero, no error. | AN-29, AN-22 |
| ANA-51 | analyze | `test_filter_by_species_no_deaths` | Filter species = Cow (3 animals, no deaths) → overall rate exactly 0. | AN-2, AN-29 |

## Numeric oracles (become our known-value tests)

| dataset | asserted outcome |
|---|---|
| weights {100, 200} same date | 1 trend point, mean 150; summary count 2, mean 150, min 100, max 200 |
| weights {100, 200} | histogram summary count 2, mean 150; integer bin counts |
| single weight {150} | 1 populated bin, count 1 |
| pairs (100, 80) and (200, 150) | Pearson r = 1.0 ± 0.01 over 2 pairs |
| weights {100, 200} deep dive | count 2, mean 150, min 100, max 200, sum 300 |
| non-existent field / zero observations | overview count 0 |
| 2×2 matrix | diagonal exactly 1.0; off-diagonals equal within 0.001 |
| matrix with 1 field / 0 observations / filter matching nothing | error, never a fabricated matrix |
| select Cow 2 / Sheep 1 / Goat 2 | 3 category rows, counts 2/1/2, no regression line, empty outliers |
| select deep dive / text deep dive | count 5, 3 categories, empty trend, empty outliers |
| dashboard, 2 animals / empty DB | total ≥ 2 / total = 0 |
| 12 weight readings | exactly 10 buckets; Σ counts = summary count; min ≤ max |
| filter species = Cow / absent species | count 5 / 0 rows, count 0 |
| single reading 500 | count 1, 10 buckets, Σ = 1 |
| growth trend default / day | month labels (2 periods) / day labels (3 periods); direction ∈ {up, down, stable}; numeric change |
| composition over 8 animals | total > 0; Σ % = 100 ± 0.1; Σ counts = total; Sheep → total 3 |
| mortality, 1 death in 8 | overall rate > 0 (~12.5 % if the herd total is the denominator) |
| no death data / Cow filter | 0 / 0 |
| no widget configuration | widget collection empty |

## Waived scenario (1)

| id | legacy test | why waived |
|---|---|---|
| note · ANA-39 | legacy tabular column-name and response-key envelope assertions (`range_label`, `avg_weight`, `rate_pct`, `totalEntities`, `recentActivities`) | Envelope shapes are a legacy transport detail; we assert typed command output fields, and formatting (range labels, percentages) belongs at the UI edge |

## Fresh — write from `REBUILD_PLAN` (no legacy coverage)

- **Central tendency beyond the mean:** median, percentiles/IQR, mode, and the sample-vs-population std-dev choice are never asserted numerically — only that a std value exists (AN-1, AN-3).
- **Regression numbers:** slope, intercept and r² are never value-asserted; no positive/negative/flat slope oracle, no r² known value (AN-4).
- **Pearson r on non-degenerate data:** every legacy oracle uses 2 points, where r is ±1 by construction. Need ≥3 points, a negative r, a weak r, and a constant series where r is undefined and must be *reported*, never NaN (AN-9).
- **Group statistics contents:** median/std/min/max per group never checked; by-breed and by-sex breakdowns are always empty in the fixtures (AN-12, AN-19, AN-20).
- **Outlier rule:** IQR fence vs z-score vs fixed top-5 is never defined or asserted, and the top-5 cap is untested (AN-21).
- **Filter coverage:** breed/sex/date filters exist only for trend and one matrix case; distribution, comparison and deep dive filters are untested (AN-2).
- **Dirty data inside a numeric series:** nulls, text in a numeric field, values in the wrong storage slot, duplicated same-day readings, and the effect of latest-value semantics on analytics (DE-7 × AN).
- **Degenerate statistics:** all-equal values (std 0), a single-member group, division by zero inside correlation and percentage maths.
- **Profitability in full:** production-field resolution by purpose tag, prices and feed cost in integer minor units, currency from settings (TND default), best/worst performers — legacy asserts structure only (AN-23…AN-25).
- **Dashboard widget surface:** the seven widget types, configuration hydration from the database, enable/disable and reorder, alert count in the summary, and the 10-widget/1k-animal hydration budget (DB-1…DB-13).
- **i18n:** nothing runs in French or asserts insight text comes from a catalogue; every insight must be a template key + params, not prose (SY-5, AN-26).
- **Determinism:** same input twice → identical output; and the analytics path never touches the LLM (AN-26).
- **Ordering:** bin order, group order and period order (chronological) are never asserted.
- **Age computation and age-group boundaries:** thresholds and the missing-birth-date case are untested, though two fixture animals have no birth date (HM-13, AN-28).
- **Units and locale formatting** travel with a numeric result at the UI edge only.
- **Read-only posture:** an analysis must not write an audit row (SY-6 scope).
- **Performance budget** for any analysis on a large herd.

## Anti-patterns — never reproduce

> **Feeds `REBUILD_PLAN.md` §3 (one authoritative home — this list is provenance only):**
> 1 → `analytics.structured-insights` · 2 → `analytics.numeric-bounds` ·
> 3 → `analytics.typed-results` · 4 → `analytics.one-empty-contract` ·
> 5 → `analytics.one-empty-contract` · 6 → `identity.columns-not-eav` ·
> 7 → `analytics.single-engine` · 8 → `schema.no-sentinels` ·
> 9 → `test-integrity.no-vacuous-assertions` · 10 → `test-integrity.assert-values` ·
> 11 → `test-integrity.assert-values` · 12 → `analytics.numeric-bounds` ·
> 13 → `analytics.single-engine` + (D16) · 14 → D9 (pig policy — not a register row)

1. **Insights as free-form prose asserted by English substring** ("not found", "one data point", "requires"). Insights must be structured (template key + params + severity) resolved through i18n; tests assert the key and params.
2. **Presentation formatting inside the analytics engine:** histogram bins arrive as pre-built range *strings*; counts are the truth and bins must expose numeric bounds an agent can consume.
3. **Polymorphic null-filled result envelope:** every analysis returns the same key set with `trend_line: null` and `outliers: {high: [], low: []}` for categorical results. Use typed per-analysis result variants — absence is a variant, not a null.
4. **Contradictory empty/error signalling:** the same "grouping field missing" condition returns a populated structure in one test and an empty chart in the next. One rule: typed error for invalid input, structured empty result with a reason code for valid input with no data.
5. **Silent "Unknown" bucket** for a missing grouping dimension instead of an actionable error naming it.
6. **Identity facts stored as observations** (species, sex, breed, birth date, death as a text flag), with age groups derived by parsing a birth-date string. Identity is columns (D7); age is computed with `chrono` (HM-12, HM-13).
7. **A second dispatch surface with its own output shape:** a separate analysis-function executor with string dispatch and its own column lists duplicates the command registry.
8. **Magic sentinel for scope** (`mode_id = 0` meaning "no mode") — the scope is an optional value.
9. **Statistically vacuous oracles:** correlation asserted only on 2 points; the matrix diagonal/symmetry checked on the same degenerate data.
10. **Unspecified metric definitions accepted as green:** mortality's denominator is never defined and year rows are never value-checked. Document the formula and assert the number.
11. **Structure-only acceptance tests:** profitability and the dynamic dashboard assert key presence only — no numbers, no money, no minor units.
12. **Histogram edge case left accidental:** a single distinct value yields 10 buckets of implicit width 1 with 9 empty. Define the min = max case explicitly.
13. **A banned second analytics engine** (D16) and any test that requires one.
14. **A fixture species that the pig policy forbids** used as the "no match" case.
