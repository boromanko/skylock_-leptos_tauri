# Skylock UI (Leptos + Tauri) — Intent & Rules

## Purpose
This repository contains the new Skylock UI built with **Leptos + Tauri**. The UI is used on a dedicated device and is fully operable via **6 hardware buttons** (Up, Down, Left, Right, Enter, Esc). Keyboard/mouse support is secondary.

## Scope
- Build the UI from scratch, using the legacy Rust prototype **only as visual reference**.
- No backend/frontend split in runtime; Tauri hosts the UI and bridges to the backend later.
- Focus on **UI structure, styles, and interaction logic**.

## Core Principles
- **SCSS-first:** styling and animations are done in SCSS; JS/Rust only for UI logic and state.
- **Keyboard-first UX:** every screen is fully usable with the 6 buttons.
- **Stable focus model:** all focusable elements have stable IDs and visible focus states.
- **No business logic in UI:** UI emits typed events; backend integration is added later.

## Working Agreement
- Use clear, readable components and predictable folder structure.
- Prefer tokens (CSS variables) over raw values.
- Keep animations and transitions in CSS.
- Update this document if we change rules or architecture decisions.

## Reference
- Legacy prototype (ignored by git): `Skylock reference prototype/`
- Design system WIP may be used as reference only.
