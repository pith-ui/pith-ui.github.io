---
title: Separator
description: Visually or semantically separates content.
features:
  - Accessible
  - Horizontal & vertical
  - Decorative mode
---

# Separator

## Demo

### Basic

Horizontal and vertical separators used to divide content sections. The vertical separators use `decorative=true` since they are purely visual.

<!-- demo: separator_basic -->

## Anatomy

Import the component and use it as a single part:

```rust
use pith_ui::separator::*;

<Separator />
```

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### Separator

A horizontal or vertical divider line.
Renders a `<div>` element with `role="separator"` (or `role="none"` when `decorative` is true).

| Prop | Type | Default | Description |
|---|---|---|---|
| orientation | `MaybeProp<Orientation>` | `Horizontal` | The orientation of the separator. |
| decorative | `MaybeProp<bool>` | `false` | When true, renders with `role="none"` and hides from screen readers. |

| Attribute | Description |
|---|---|
| data-orientation | "horizontal" or "vertical". |
