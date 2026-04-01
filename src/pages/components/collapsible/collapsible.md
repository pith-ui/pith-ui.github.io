---
title: Collapsible
description: A component that expands and collapses a panel of content.
features:
  - Accessible
  - Animated
  - Controlled & uncontrolled
---

# Collapsible

## Demo

### Basic

A controlled collapsible that reveals additional list items when toggled.

<!-- demo: collapsible_basic -->

## Anatomy

Import the component and assemble its parts:

```rust
use pith_ui::collapsible::*;

<Collapsible>
    <CollapsibleTrigger />
    <CollapsibleContent />
</Collapsible>
```

## Examples

### Animating content

Use the `--collapsible-content-height` CSS custom property with keyframe animations to animate the content height. The `data-state` attribute toggles between `"open"` and `"closed"`.

```css
@keyframes collapsible-open {
    from { height: 0; }
    to { height: var(--collapsible-content-height); }
}

@keyframes collapsible-close {
    from { height: var(--collapsible-content-height); }
    to { height: 0; }
}

.collapsible-content[data-state="open"] {
    animation: collapsible-open 200ms ease-out;
}

.collapsible-content[data-state="closed"] {
    animation: collapsible-close 200ms ease-out;
}
```

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### Collapsible

Root component that manages the open/close state.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| open | `MaybeProp<bool>` | — | The controlled open state. |
| default_open | `MaybeProp<bool>` | `false` | Whether the collapsible should be open initially. |
| on_open_change | `Option<Callback<bool>>` | — | Callback fired when the open state changes. |
| disabled | `MaybeProp<bool>` | `false` | Whether the collapsible is disabled. |

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |
| data-disabled | Present when the collapsible is disabled. |

### CollapsibleTrigger

A button that toggles the content visibility.
Renders a `<button>` element.

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |
| data-disabled | Present when the collapsible is disabled. |

### CollapsibleContent

The content area that expands and collapses.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the content mounted in the DOM when closed. |

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |
| data-disabled | Present when the collapsible is disabled. |

The `--collapsible-content-height` and `--collapsible-content-width` CSS custom properties are available for animating the content size.
