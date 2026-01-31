# CSS/SCSS Guide (Skylock UI)

This project uses SCSS with a strict token-first approach. Follow these rules to keep styles consistent and easy to maintain.

## 1) Folder structure

All SCSS lives under `app/src/styles/` and is split by purpose:

- `tokens/` — design tokens only (colors, sizing, typography, icon glyphs).
- `base/` — global base styles (html/body defaults, resets).
- `utilities/` — reusable utilities (animations, spacing helpers, layout helpers).
- `components/` — component styles (List Item, Toggle, Checkbox, etc).
- `pages/` — page-specific styles (Gallery).

Entry point: `app/styles.scss` (imports everything in order).

## 2) Token-first rules (non-negotiable)

- Do not use raw colors, sizes, radii, or font values in component styles.
- Always use CSS variables from `tokens/` or semantic tokens.
- If a needed token is missing, add it to the appropriate token file first.

## 3) Naming and structure

- Components use BEM-style classes: `.component`, `.component__part`, `.component--variant`.
- Utilities are simple, generic, and reusable (`.centered`, `.stack`, `.pressable`).
- Avoid deep nesting; keep selectors flat and explicit.

## 4) Transitions and animations

- All transitions/animation tokens live in `utilities/animations.scss`.
- Components must use transition variables (e.g. `--transition-toggle-bg`).
- If you need a new transition, add it in `animations.scss` first.

## 5) Where to put new styles

- New component → `app/src/styles/components/<component>.scss`
- New utility → `app/src/styles/utilities/utilities.scss`
- New token → `app/src/styles/tokens/<file>.scss`
- Page-specific → `app/src/styles/pages/<page>.scss`

Then add the import in `app/styles.scss` in the correct section.

## 6) Examples

```scss
/* Component style (token-first) */
.toggle {
  width: var(--size-56);
  height: var(--size-28);
  background: var(--switch-track-off);
  border-radius: var(--radius-full);
  transition: var(--transition-toggle-bg);
}
```

```scss
/* Utility usage */
.list-item__trailing-arrow {
  @extend .centered;
  color: var(--text-secondary);
}
```

## 7) Quick checklist

- [ ] Uses tokens (no raw values)
- [ ] Correct folder and import in `app/styles.scss`
- [ ] BEM naming
- [ ] Uses shared transitions/utilities
- [ ] No deep nesting
