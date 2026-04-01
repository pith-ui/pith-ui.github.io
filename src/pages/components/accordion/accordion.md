---
title: Accordion
description: A set of collapsible panels with headings.
features:
  - Accessible
  - Keyboard navigation
  - Single or multiple
  - Collapsible
  - Animated
---

# Accordion

## Demo

### Basic

A collapsible accordion with three items, allowing one open section at a time.

<!-- demo: accordion_basic -->

## Anatomy

Import the component and assemble its parts:

```rust
use pith_ui::accordion::*;

<AccordionSingle>
    <AccordionItem>
        <AccordionHeader>
            <AccordionTrigger />
        </AccordionHeader>
        <AccordionContent />
    </AccordionItem>
</AccordionSingle>
```

## Examples

### Open multiple panels

Use `~AccordionMultiple~` instead of `~AccordionSingle~` to allow multiple panels to be open at the same time.

<!-- demo: accordion_multiple -->

### Collapsible

By default, `~AccordionSingle~` keeps one item open at all times. Set `collapsible` to `true` to allow the open item to be collapsed.

```rust
<AccordionSingle collapsible=true>
    // ...
</AccordionSingle>
```

### Disabled

Disable the entire accordion or individual items.

```rust
// Disable the entire accordion
<AccordionSingle disabled=true>
    // ...
</AccordionSingle>

// Disable a single item
<AccordionItem value="item-1" disabled=true>
    // ...
</AccordionItem>
```

### Animating content

Use the `--accordion-content-height` CSS custom property with keyframe animations to animate the content height. The `data-state` attribute toggles between `"open"` and `"closed"`.

```css
@keyframes accordion-open {
    from { height: 0; }
    to { height: var(--accordion-content-height); }
}

@keyframes accordion-close {
    from { height: var(--accordion-content-height); }
    to { height: 0; }
}

.accordion-content[data-state="open"] {
    animation: accordion-open 200ms ease-out;
}

.accordion-content[data-state="closed"] {
    animation: accordion-close 200ms ease-out;
}
```

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### AccordionSingle

Root container where only one item can be open at a time.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| value | `MaybeProp<String>` | — | The controlled value of the open item. |
| default_value | `MaybeProp<String>` | — | The value of the item that should be open initially. |
| on_value_change | `Option<Callback<String>>` | — | Callback fired when the open item changes. |
| collapsible | `MaybeProp<bool>` | `false` | Whether the open item can be collapsed by clicking its trigger again. |
| disabled | `MaybeProp<bool>` | `false` | Whether all items are disabled. |
| orientation | `MaybeProp<Orientation>` | `Vertical` | The orientation of the accordion. |
| dir | `MaybeProp<Direction>` | — | The reading direction (ltr or rtl). |

| Attribute | Description |
|---|---|
| data-orientation | "horizontal" or "vertical". |

### AccordionMultiple

Root container where multiple items can be open simultaneously.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| values | `MaybeProp<Vec<String>>` | — | The controlled values of the open items. |
| default_values | `MaybeProp<Vec<String>>` | — | The values of items that should be open initially. |
| on_values_change | `Option<Callback<Vec<String>>>` | — | Callback fired when the open items change. |
| disabled | `MaybeProp<bool>` | `false` | Whether all items are disabled. |
| orientation | `MaybeProp<Orientation>` | `Vertical` | The orientation of the accordion. |
| dir | `MaybeProp<Direction>` | — | The reading direction (ltr or rtl). |

| Attribute | Description |
|---|---|
| data-orientation | "horizontal" or "vertical". |

### AccordionItem

Contains the trigger and content for a single collapsible section.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| value | `String` | — | A unique value that identifies this item (required). |
| disabled | `MaybeProp<bool>` | `false` | Whether this item is disabled. |

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |
| data-disabled | Present when the item is disabled. |
| data-orientation | "horizontal" or "vertical". |

### AccordionHeader

Wraps the trigger button.
Renders an `<h3>` element.

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |
| data-disabled | Present when the item is disabled. |
| data-orientation | "horizontal" or "vertical". |

### AccordionTrigger

The button that toggles the section open or closed.
Renders a `<button>` element.

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |
| data-disabled | Present when the item is disabled. |
| data-orientation | "horizontal" or "vertical". |

### AccordionContent

The collapsible content area.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the content mounted in the DOM when closed. |

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |
| data-disabled | Present when the item is disabled. |
| data-orientation | "horizontal" or "vertical". |

The `--accordion-content-height` and `--accordion-content-width` CSS custom properties are available for animating the content size.
