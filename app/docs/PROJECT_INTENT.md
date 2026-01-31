# Skylock UI (Leptos + Tauri) — Project Intent & Rules

## What this is
Skylock UI is a **keyboard‑first** interface for a dedicated device. The app is built in **Leptos + Tauri**, runs as a single desktop app, and is fully operable with **6 hardware buttons** (Up, Down, Left, Right, Enter, Esc). Mouse/keyboard support is secondary.

## Scope
- Build the UI from scratch, using the legacy Rust prototype **only as visual reference**.
- No separate frontend/backend at runtime; Tauri hosts the UI and later bridges to backend.
- Focus on **UI structure, styling, and interaction logic** (no business logic here).

## Architecture at a glance
- **UI framework:** Leptos
- **Shell:** Tauri
- **Styling:** SCSS + CSS variables
- **State:** UI state lives in Leptos; backend integration later
- **Docs:** kept in `app/docs/` and must be kept current

## Folder map (where to look)
- `app/src/screens/` — app screens (Home, Radial, Settings, Wizard)
- `app/src/components/` — reusable UI components
- `app/src/state/` — navigation, focus, UI state
- `app/src/styles/` — tokens + base + components SCSS
- `app/public/fonts/` — local fonts (Inter + Material Symbols)
- `app/docs/` — project docs, tokens sources, icon list
- `reference prototype/` — legacy reference only (not committed)

## Non‑negotiable rules (must do)
- **SCSS‑first:** all styling & animations in SCSS.
- **Token‑first:** use CSS variables; no raw values in components.
- **Keyboard‑first:** every screen works with 6 hardware buttons.
- **Stable focus:** every focusable element has a stable ID and visible focus state.
- **Event‑driven UI:** components emit events; screens/controllers decide effects.
- **Docs stay current:** update `app/docs/` on every structural or rules change.

## Things you MUST NOT do
- Do **not** add business logic or backend calls to UI components.
- Do **not** hardcode colors, sizes, or fonts in component styles.
- Do **not** rely on mouse/hover for required actions.
- Do **not** introduce uncontrolled focus changes (no stealing focus every frame).
- Do **not** modify the legacy prototype; treat it as read‑only.
- Do **not** introduce SCSS complexity (deep nesting, huge mixins).

## How we implement UI (simple workflow)
1) Check reference prototype for visuals only.
2) Add/adjust tokens in `app/src/styles/tokens.scss` or `typography.scss`.
3) Implement component styles in SCSS using tokens.
4) Build a gallery/demo screen with plain HTML + SCSS to validate visuals fast.
5) After styles are stable, **extract real Leptos components** with props/API.
6) Implement UI logic in Leptos with keyboard navigation + focus.
7) Update docs in `app/docs/` if anything changes.

## Current phase (important)
We are **intentionally** styling components as HTML + SCSS inside the gallery first.
This is not a shortcut — it is a deliberate phase to lock down visuals and tokens.
Once visuals are stable, every component will be refactored into a **real Leptos
component** with a clean API (props), reusability, and keyboard-first behavior.

## References
- Legacy prototype: `reference prototype/`
- Icon list: `app/docs/icons.json`
