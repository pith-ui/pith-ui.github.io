# CLAUDE.md — pith-ui-site

Documentation website for [Pith UI](https://github.com/pith-ui/pith-ui), built with Leptos + Trunk + Tailwind CSS v4.

## Build & Development

```bash
# Serve locally (port 8080)
trunk serve

# Build for production
trunk build --release

# Rebuild Tailwind CSS only
npx @tailwindcss/cli -i style/input.css -o style/output.css --minify
```

Trunk's pre-build hook runs the Tailwind CLI automatically (see `Trunk.toml`).

## Demo System

Demos are the primary way component usage is shown on the site. Each demo is **self-contained in its own file** — one file per demo, with all necessary imports, the rendered component, and a section wrapper.

### File location

```
src/pages/components/demos/
├── mod.rs
├── checkbox_basic.rs
├── checkbox_form.rs
└── ...
```

### Demo file structure

Every demo file follows this pattern:

```rust
use leptos::prelude::*;
use pith_ui::checkbox::*;
use crate::components::{extract_demo, DemoTabs};

// Tailwind class strings — outside the region, so they don't appear in the code tab.
// These are site-specific styling; readers should substitute their own classes.
const ROOT: &str = "flex h-5 w-5 items-center justify-center rounded ...";

// Helper functions also go outside the region.
fn check_icon() -> impl IntoView { ... }

// #region demo
#[component]
pub fn CheckboxBasic() -> impl IntoView {
    // All demo logic and view here
}
// #endregion demo

#[component]
pub fn CheckboxBasicSection() -> impl IntoView {
    view! {
        <h3 class="...">"Title"</h3>
        <p class="...">"Description of what this demo shows."</p>

        <DemoTabs source=extract_demo(include_str!("checkbox_basic.rs"))>
            <CheckboxBasic />
        </DemoTabs>

        // Optional follow-up text
    }
}
```

### Key rules

1. **One demo per file.** Each file has all imports needed to compile the demo independently. No shared preludes.
2. **Region markers define the code tab.** Only content between `// #region demo` and `// #endregion demo` appears in the Code tab. Put `const` class strings, helper functions, and the Section wrapper outside the region.
3. **Tailwind classes in `const` strings.** Keep `view!` markup clean — define long class strings as `const` values above the region. These are site-specific styling that readers would replace, so hiding them from the code tab is intentional.
4. **`include_str!` is the mechanism.** The `extract_demo` function + `include_str!` preserves exact source formatting. No proc macros — this avoids `TokenStream::to_string()` destroying `view!` formatting.
5. **Naming convention.** File: `<component>_<variant>.rs`. Demo component: `<PascalComponent><PascalVariant>`. Section: `<PascalComponent><PascalVariant>Section`.

### DemoTabs component

`DemoTabs` (`src/components/demo.rs`) renders a two-tab viewer using Pith UI's own `Tabs` and `ScrollArea` components:

- **Preview tab** — renders the live demo (the children)
- **Code tab** — shows syntax-highlighted source via Prism.js, in a `ScrollArea` capped at ~30 lines (`max-h-[680px]`)

The code tab uses `force_mount=true` so Prism highlights once on initial mount rather than re-highlighting on every tab switch.

Usage:
```rust
<DemoTabs source=extract_demo(include_str!("my_demo.rs"))>
    <MyDemo />
</DemoTabs>
```

### Wiring a new demo

1. Create `src/pages/components/demos/<component>_<variant>.rs` following the structure above.
2. Add `mod <component>_<variant>;` and `pub use <component>_<variant>::*;` to `demos/mod.rs`.
3. Use `<ComponentVariantSection />` in the component's page file (`src/pages/components/<component>.rs`).

## Tailwind CSS v4

This project uses **Tailwind v4** with CSS-native configuration. There is **no `tailwind.config.js`** — all configuration lives in `style/input.css`.

### Configuration patterns

**Imports and source scanning:**
```css
@import "tailwindcss";

@source "../src/**/*.rs";
@source "../index.html";
```

The `@source` directives tell Tailwind to scan `.rs` files for class names, since Rust isn't a default content source.

**Theme customization via `@theme`:**
```css
@theme {
    --font-sans: 'Inter', system-ui, sans-serif;
    --color-accent-600: #4f46e5;
    --color-code: #2d2d2d;
    --animate-slide-down: slide-down 200ms ease-out;
}
```

These generate standard utilities (`font-sans`, `bg-accent-600`, `bg-code`, `animate-slide-down`). To add a new design token, add it to the `@theme` block — never create a JS config file.

**Custom component styles** use `@layer components` for animation classes tied to `data-state` attributes (accordion, dialog, tooltip, etc.).

**Keyframes** are defined at the top level in CSS, outside `@theme`. The `@theme` block only maps them to `--animate-*` tokens.

### Conventions

- **Semantic color names** — use `accent-*` for the brand palette, `code` for code block backgrounds, and Tailwind's built-in `slate-*` for neutrals.
- **`bg-code` must match the Prism theme.** If the Prism theme changes, update `--color-code` in `@theme` to match. Currently using `prism-tomorrow` (`#2d2d2d`).
- **No `@apply`** — use Tailwind classes directly in markup. Long class strings go in `const` values for readability.

## Formatting

- Prettier (width 120, tab width 4, single quotes, no bracket spacing) for CSS/JS files.
