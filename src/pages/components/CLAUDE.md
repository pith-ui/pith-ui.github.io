# Writing Component Documentation (Mirroring base-ui)

Pith UI documentation mirrors [base-ui](https://base-ui.com) in structure, examples, and explanatory text — transliterated from React/TSX to Leptos/Rust.

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

## What to mirror

The base-ui markdown for each component has these sections. Mirror all of them:

| Base-ui section | Pith UI equivalent |
|---|---|
| `# Title` + frontmatter description | `ComponentPageHeader` title + description |
| `## Demo` (Tailwind example) | `DocSection title="Demo"` with our demo system (see root CLAUDE.md) |
| `## Usage guidelines` | `DocSection title="Usage guidelines"` — keep the same bullet points, adapted for Leptos where needed |
| `## Anatomy` | `DocSection title="Anatomy"` — transliterate the JSX composition example to `view!` macro syntax |
| `## Examples` (subsections) | Additional `DocSection`s or demo sections — one per example. Match their example names and scenarios |
| `## API reference` | `DocSection title="API Reference"` — see API reference section below |

### Sections to skip or adapt

- **CSS Modules examples**: base-ui shows both Tailwind and CSS Modules demos. We only show the Tailwind version (our demos use Tailwind classes in `const` strings).
- **React-specific patterns**: e.g., `render` prop, `className` callbacks, `ref` forwarding. Omit these or replace with Leptos equivalents (`attr:class`, `NodeRef`, etc.).
- **`<Meta>` tags**: base-ui includes `<Meta name="description">` for SEO. Skip — we don't use these.

## Transliterating code examples

When converting base-ui React/TSX to Pith UI Leptos/Rust:

| React (base-ui) | Leptos (Pith UI) |
|---|---|
| `<Tooltip.Root>` | `<Tooltip>` |
| `<Tooltip.Trigger>` | `<TooltipTrigger>` |
| `<Component.Part>` | `<ComponentPart>` (PascalCase, no dot notation) |
| `className="..."` | `attr:class=SOME_CONST` (in demos) or `attr:class="..."` |
| `defaultChecked` | `default_checked` (snake_case props) |
| `onCheckedChange={handler}` | `on_checked_change=handler` |
| `sideOffset={10}` | `side_offset=10.0` (note: floats for numeric props) |
| `{children}` | `{children()}` |
| `<label htmlFor="x">` | `<label for="x">` |
| String content | `"String content"` (quoted in `view!` macro) |
| `import { X } from '@base-ui/react/x'` | `use pith_ui::x::*;` |
| `useState` / controlled state | `RwSignal` / `create_signal` |

## Demo examples

Each base-ui `## Demo` and `## Examples` subsection that includes a code sample should become a demo file in the component's `demos/` directory, following the demo system documented in the root CLAUDE.md.

Match the base-ui example names where possible:

- Base-ui "Detached triggers" example → `tooltip_detached_triggers.rs` → `TooltipDetachedTriggersSection`
- Base-ui "Form integration" example → `checkbox_form.rs` → `CheckboxFormSection`

For the **primary demo** (the first `## Demo` in the base-ui markdown), name it `<component>_basic.rs`.

## Explanatory text

Mirror the base-ui prose closely. The usage guidelines, anatomy descriptions, and example explanations should read naturally for our context:

- Replace "React" with "Leptos" where it appears in descriptive text.
- Replace package references (`@base-ui/react/tooltip`) with Pith UI crate paths (`pith_ui::tooltip`).
- Keep accessibility guidance verbatim — it applies equally.
- Keep conceptual explanations (e.g., "Alternatives to tooltips") — these are framework-agnostic.

## API reference

Base-ui's API reference has, for each part:

1. **Description** — what the part does and what element it renders
2. **Props table** — Prop, Type, Default, Description
3. **Data Attributes table** — Attribute, Type, Description
4. **CSS Variables table** (some parts) — Variable, Type, Default, Description

### Components (`src/components/demo.rs`)

The API reference uses these components:

- **`ApiPart`** — wraps each component part with name, description, and rendered element. Takes `children` containing `ApiPartProps` and/or `ApiPartDataAttrs`.
- **`ApiPartProps`** — an accordion-based props table. Header row shows Prop/Type/Default. Each child `Prop` renders as an accordion item (collapsed: table row; expanded: detail panel with Name, Description, Type, Default).
- **`ApiPartDataAttrs`** — a simple 2-column table (Attribute, Description). Each child `DataAttr` renders as a table row.
- **`Prop`** — a single prop definition (name, type, default, description). Renders as an `AccordionItem`.
- **`DataAttr`** — a single data attribute definition (name, description). Renders as a `<tr>`.

### Usage pattern

```rust
<ApiPart name="ComponentName" description="What it does." renders="<div>">
    <ApiPartProps>
        <Prop name="value" r#type="MaybeProp<String>" description="The controlled value." />
        <Prop name="disabled" r#type="MaybeProp<bool>" default="false" description="Whether to disable." />
    </ApiPartProps>
    <ApiPartDataAttrs>
        <DataAttr name="data-state" description="\"open\" or \"closed\"" />
    </ApiPartDataAttrs>
</ApiPart>
```

Parts with no props or data attributes omit the sub-components:

```rust
<ApiPart name="ComponentNameList" description="Container for items." renders="<ul>" />
```

### Deriving prop information

- **Prop names and Rust types**: read from the Pith UI component source (snake_case, `MaybeProp<T>`, `Option<Callback<T>>`, etc.)
- **Descriptions**: adapt from the base-ui markdown
- **Data attributes**: mirror base-ui's `data-*` attributes since Pith UI uses the same conventions
- **Common props** (`as_child`, `node_ref`, `children`): omit from individual tables — mention once at the top of the API Reference section

## Page structure template

```rust
use leptos::prelude::*;
use crate::components::*;

mod demos;
use demos::*;

#[component]
pub fn ComponentNamePage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            // Title + description from base-ui frontmatter
            <ComponentPageHeader
                title="Component Name"
                description="Description from base-ui subtitle/description."
                features=&["Accessible", "Keyboard support", ...]
            />

            // Primary demo
            <DocSection title="Demo">
                <ComponentNameBasicSection />
            </DocSection>

            // Usage guidelines — mirror base-ui bullet points
            <DocSection title="Usage guidelines">
                // ...
            </DocSection>

            // Anatomy — transliterated composition example
            <DocSection title="Anatomy">
                // ...
            </DocSection>

            // Additional examples — one DocSection per base-ui example
            <DocSection title="Example Name">
                <ComponentNameExampleSection />
            </DocSection>

            // API reference — accordion-based props tables
            <DocSection title="API Reference">
                <ApiPart name="ComponentName" description="..." renders="<div>">
                    <ApiPartProps>
                        <Prop name="value" r#type="MaybeProp<String>" description="..." />
                    </ApiPartProps>
                    <ApiPartDataAttrs>
                        <DataAttr name="data-state" description="..." />
                    </ApiPartDataAttrs>
                </ApiPart>
            </DocSection>
        </div>
    }
}
```

## Checklist for a new or updated component page

1. Read `.claude/reference/base-ui/<component>.md` end to end
2. Read the Pith UI component source to understand actual prop names and types
3. Create/update demo files for each base-ui example
4. Write the page following the structure template above
5. Mirror all base-ui sections: usage guidelines, anatomy, examples, API reference
6. Verify the page compiles (`trunk serve`)
