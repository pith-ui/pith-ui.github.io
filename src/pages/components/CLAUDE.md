# Writing Component Documentation (Mirroring base-ui)

Pith UI documentation mirrors [base-ui](https://base-ui.com) in structure, examples, and explanatory text — transliterated from React/TSX to Leptos/Rust.

**Golden example**: `src/pages/components/navigation_menu/` — follow this as the reference for all other components.

## Reference material

Base-ui markdown docs are downloaded to `.claude/reference/base-ui/`. The URL pattern is stable:

```
https://base-ui.com/react/components/<kebab-case-name>.md
```

To refresh or add new references, run:

```bash
bash .claude/scripts/fetch-base-ui-docs.sh
```

**Always read the corresponding base-ui markdown before writing or updating a component page.** It is the source of truth for what sections, examples, and API details to include.

## Markdown-driven pages

Each component page is driven by a single markdown file. The page's `mod.rs` is a thin wrapper that calls the markdown parser with a demo lookup closure.

### File structure

```
src/pages/components/<component>/
├── mod.rs                      # Thin wrapper: calls md_page::render_md_page()
├── <component>.md              # All page content (frontmatter, prose, API ref)
└── demos/
    ├── mod.rs                  # Exports demo components only (no Section wrappers)
    ├── <component>_basic.rs    # Primary demo
    ├── <component>_nested.rs   # Example demos...
    └── ...
```

The markdown parser lives at `src/components/md_page.rs` and is shared by all component pages.

### mod.rs pattern

```rust
use leptos::prelude::*;
use crate::components::extract_demo;

mod demos;
use demos::*;
use crate::components::md_page::{self, DemoEntry};

#[component]
pub fn ComponentNamePage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("<component>.md"),
        |name| match name {
            "<component>_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <ComponentBasic /> }),
                source: extract_demo(include_str!("demos/<component>_basic.rs")),
            }),
            // ... one entry per demo
            _ => None,
        },
    )
}
```

### Markdown format

See `navigation_menu/navigation_menu.md` for the canonical example. Key conventions:

**Frontmatter** — title, description, features array:
```yaml
---
title: Navigation Menu
description: A collection of links and menus for website navigation.
features:
  - Accessible
  - Keyboard navigation
---
```

**Demo markers** — `<!-- demo: name -->` inserts a `DemoTabs` component:
```markdown
### Basic

Description of the demo.

<!-- demo: navigation_menu_basic -->
```

**Inline code** — two flavors:
- `` `~ComponentName~` `` → accent-colored code (component references)
- `` `code` `` → standard code (HTML elements, props, CSS properties)

**Code fences** — rendered with Prism.js highlighting and a copy button:
````markdown
```rust
<NavigationMenu>
    <NavigationMenuList />
</NavigationMenu>
```
````

**API Reference** — structured markdown tables that the parser converts to accordion-based prop tables:
```markdown
## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### ComponentName

Description of what it does.
Renders a `<nav>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| value | `MaybeProp<String>` | — | The controlled value. |

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |
```

## Demo files

Demo files contain **only** the demo component — no Section wrapper, no heading, no description. Those come from the markdown. The `#region demo` markers define what appears in the Code tab.

```rust
// #region demo
use leptos::prelude::*;
use pith_ui::navigation_menu::*;

#[component]
pub fn NavigationMenuBasic() -> impl IntoView {
    view! { ... }
}
// #endregion demo
```

### demos/mod.rs

Export only demo components (not Sections):
```rust
mod navigation_menu_basic;
pub use navigation_menu_basic::NavigationMenuBasic;
```

## Transliterating from base-ui

### Component mapping

| React (base-ui) | Leptos (Pith UI) |
|---|---|
| `<Component.Part>` | `<ComponentPart>` (PascalCase, no dot notation) |
| `className="..."` | `attr:class="..."` |
| `defaultValue` | `default_value` (snake_case props) |
| `onValueChange={handler}` | `on_value_change=handler` |
| `sideOffset={10}` | `side_offset=10.0` (floats for numeric props) |
| String content | `"String content"` (quoted in `view!` macro) |
| `import { X } from '@base-ui/react/x'` | `use pith_ui::x::*;` |

### Components that don't exist in pith-ui

Base-ui has components that pith-ui doesn't. These need equivalent patterns:

| Base-ui | Pith-ui equivalent |
|---|---|
| `NavigationMenu.Portal > Positioner > Popup` | `NavigationMenuViewport` with `absolute` positioning |
| `NavigationMenu.Arrow` | Not available — omit (library gap) |
| `NavigationMenu.Backdrop` | Not available — omit |
| `NavigationMenu.Icon` | Manual chevron SVG |

### Data attribute differences

| Base-ui | Pith-ui |
|---|---|
| `data-[popup-open]` | `data-[state=open]` |
| `data-[starting-style]` / `data-[ending-style]` | `data-[motion=from-start]` / `data-[motion=to-end]` etc. |

### CSS animations

Base-ui uses `data-starting-style` and `data-ending-style` for CSS transition triggers. Pith-ui uses `data-motion` and `data-state`. Animations are defined in `style/input.css` under `@layer components`:

- `nav-content` class → keyframe animations for `data-motion` (slide enter/exit)
- `nav-viewport` class → scale+fade for `data-state` (open/close)

### Demo styling alignment

Match base-ui's Tailwind styles precisely, translating `gray-*` → `slate-*`. Key patterns:

- **Root**: `relative min-w-max rounded-lg bg-slate-50 p-1 text-slate-900` (`relative` is required for viewport positioning)
- **Triggers**: `box-border flex items-center justify-center gap-1.5 h-10 px-3.5 ...`
- **Link cards**: `block rounded-md p-3 no-underline text-inherit hover:bg-slate-100 ...`
- **Link titles**: `m-0 mb-1 text-base leading-5 font-medium`
- **Link descriptions**: `m-0 text-sm leading-5 text-slate-500`
- **Viewport**: `nav-viewport absolute left-0 top-full z-50 mt-2.5 rounded-lg bg-white shadow-lg shadow-slate-200 outline outline-1 outline-slate-200 overflow-hidden`

### Critical pith-ui patterns

1. **`NavigationMenuViewport` is required** for popups to render correctly. Without it, content renders inline and gets clipped. Place it as a sibling of `NavigationMenuList` inside the root `NavigationMenu`.

2. **`relative` on the root** `NavigationMenu` is required since the viewport uses `absolute` positioning. Base-ui doesn't need this because it portals to `<body>`.

3. **`z-50` on the viewport** ensures popups layer above subsequent page content (code blocks, other sections).

4. **`overflow-hidden` on the viewport** clips animated content during transitions.

5. **Inline nested menus** (like the Product panel with audience tabs): wrap `NavigationMenuList` in a plain `<div>` for grid item sizing, since pith-ui's wrapper elements break `self-stretch`. The `NavigationMenuViewport` goes as a sibling in the grid.

## Prism.js code highlighting

Code blocks use Prism.js. The theme (`prism-tomorrow`) uses attribute selectors that override Tailwind utilities. Our `style/input.css` has a specificity override — see the `pre[class*='language-']` rule. Our CSS must load **after** the Prism CDN stylesheet in `index.html`.

## Checklist for a new component page

1. Read `.claude/reference/base-ui/<component>.md` end to end
2. Read the Pith UI component source for prop names, types, and data attributes
3. Create `<component>.md` following the markdown format above
4. Create demo files — one per base-ui example, matching content/styling precisely
5. Create `mod.rs` with the parser wrapper and demo lookup
6. Verify with `trunk serve` — compare rendered output against base-ui's live site
