# UI design — tokens, themes, widget kit

> **Status:** target design — nothing is implemented yet. This file is the contract a UI task is
> reviewed against; it is not a mood board.
> **Source:** `REBUILD_PLAN.md` §1.4 (honest positioning), §7.1–§7.5, §13.1 (legacy palette);
> AGENTS.md rules 7–8.

## 1. What egui is, and is not

egui yields a clean, professional **tool aesthetic** — not Tailwind/React gloss (`REBUILD_PLAN.md`
§1.4). The legacy
earthy/olive design language is reproduced through **design tokens**, not through copied component
code. Large data tables are real work in egui: they are budgeted once, in Phase 1, as a reusable kit
component (§7.3, risk register §18). Expectation-setting is part of the job: a screen that looks like a
careful desktop tool is on-brief; a screen that looks like a web app is not.

## 2. Tokens are the only styling source

`crates/midata-app/src/theme.rs` owns every visual constant:

| Token group | Contents |
|---|---|
| Palette | full earth/olive scales — light: warm sand background `#f5f0ea`, olive primary `#759048`, white cards, warm-brown shadows; dark: the `#261d14` family |
| Semantic | success · warning · destructive · info, each with a background/foreground pair |
| Shape | card radius 16–28 px, control radius 12 px, borders and dividers |
| Spacing | one spacing scale used by every layout (no ad-hoc pixel literals) |
| Typography | Inter (bundled, OFL-attributed) with **tabular numerals** for data columns |

Rules:

- **No hard-coded colours, radii or spacing in screens or widgets** — everything resolves through a
  token. A literal hex in a screen is a review failure.
- Light **and** dark themes are first-class; both must work for every screen, and a token/layout change
  is verified in both.
- Theme choice persists under one key (`midata_theme`), and every persistence key lives in the single
  constants module, prefixed `midata_` (the legacy key-mismatch bug class is eliminated by construction).

## 3. Screens

`Screen` is a central enum router (no URLs, no history stack to desync). Core navigation is always
visible: **Dashboard · Animals · Entry · Analytics · Assistant** (Assistant from Phase 5). Advanced
surfaces — Packs, the full Settings area, mode-switcher prominence, Deep Analysis, Pedigree — are gated
behind **Developer Mode** (persisted flag; progressive disclosure implemented for real in Phase 6, flag
plumbed from Phase 1).

One file per screen under `crates/midata-app/src/screens/`, each thin: dispatch commands, render state,
hold no domain logic. Target set: `onboarding`, `dashboard`, `animal_list`, `animal_detail`, `entry`
(single / batch / mass tabs), `analytics_studio`, `deep_analysis`, `pedigree`, `packs`, `settings`
(Fields, Species, Modes, Formulas, Rules, AI Provider, Data & Backup, Appearance & Language, Developer),
`assistant` (docked panel, pinnable).

## 4. Widget kit (built once, Phase 1)

| Component | Contract |
|---|---|
| `DataTable` | built on `egui_extras::TableBuilder`: sortable columns, filter bar, pagination (25/50/100), multi-select, column visibility from the active mode's fields + identity columns, row context menu, keyboard navigation. The highest-effort component — it gets its own budget and snapshots. |
| `DynamicFieldRenderer` | one input per `field_definitions` row (number / text / select / boolean / multi-select / date via `egui_extras` date picker), driven by mode + cardinality + unit + inline min/max |
| Dialogs | modal with focus trap and ESC, confirm dialog for destructive operations, toasts, empty states, skeletons |
| Dashboard grid | bento-style responsive layout; cards hydrate from the widget's data configuration |
| Charts | `egui_plot` for line / bar / histogram / scatter / box; **donut/pie via custom painter** in Phase 3, with a documented fallback to horizontal bars (§16 O3) |

The kit is one-way: `widgets/` never imports from `screens/`. That rule is what keeps a future
`midata-widgets` extraction mechanical (§4.8).

## 5. States every screen must design

Empty (with the action that fills it), loading (skeleton, not a spinner on top of stale data), error
(i18n message + retry where meaningful), and — for destructive actions — an explicit confirmation.
Empty states are audited screen by screen in Phase 6. Every string is an i18n key; a Literal in a screen
is a review failure (see `docs/i18n.md`).

## 6. Verification ("look at the PNG")

Offscreen `egui_kittest` snapshots per screen and state, no window and no focus stealing:
`cargo test -p midata-app -- snapshot`. Goldens are committed; regeneration is intentional and travels
with the change. A UI task is **not** done until an agent opened the rendered PNG and described what it
saw; both themes when tokens/layout change; a French spot check when strings change. Demo imagery and
demo data are seeded only — never personal or farm data.

## 7. Accessibility and restraint

- Keyboard navigation for tables and dialogs; visible focus.
- Contrast checked per theme using the semantic token pairs rather than ad-hoc colours.
- Progressive disclosure over configuration density: the legacy 14-surface sprawl is not rebuilt behind a
  hamburger menu; advanced surfaces live behind Developer Mode (D21, §7.1).
