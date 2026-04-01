---
title: Toggle
description: A two-state button that can be on or off.
features:
  - Accessible
  - Keyboard toggle
  - Controlled & uncontrolled
---

# Toggle

## Demo

### Basic

A favorite toggle button that swaps between a filled and outline heart icon based on its pressed state.

<!-- demo: toggle_basic -->

## Anatomy

Import the component and use it as a single part:

```rust
use pith_ui::toggle::Toggle;

<Toggle>"Label"</Toggle>
```

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### Toggle

A two-state button.
Renders a `<button>` element with `aria-pressed`.

| Prop | Type | Default | Description |
|---|---|---|---|
| pressed | `MaybeProp<bool>` | — | The controlled pressed state. |
| default_pressed | `MaybeProp<bool>` | `false` | Whether the toggle should be pressed initially. |
| on_pressed_change | `Option<Callback<bool>>` | — | Callback fired when the pressed state changes. |
| disabled | `MaybeProp<bool>` | `false` | Whether the toggle is disabled. |

| Attribute | Description |
|---|---|
| data-state | "on" or "off". |
| data-disabled | Present when the toggle is disabled. |
