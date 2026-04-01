---
title: Popover
description: An accessible popup anchored to a button.
features:
  - Accessible
  - Focus management
  - Collision-aware positioning
  - Dismissable
  - Arrow support
---

# Popover

## Demo

### Basic

A notification popover anchored to a bell icon trigger with an arrow.

<!-- demo: popover_basic -->

## Anatomy

Import the component and assemble its parts:

```rust
use pith_ui::popover::*;

<Popover>
    <PopoverTrigger />
    <PopoverPortal>
        <PopoverContent>
            <PopoverArrow />
            <PopoverClose />
        </PopoverContent>
    </PopoverPortal>
</Popover>
```

## Examples

### Opening on hover

Use controlled state with pointer events to open the popover on hover. Pointer handlers on both the trigger and the content keep the popup open while the cursor moves between them.

<!-- demo: popover_hover -->

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### Popover

Root component that manages the popover's open/close state.

| Prop | Type | Default | Description |
|---|---|---|---|
| open | `MaybeProp<bool>` | — | The controlled open state. |
| default_open | `MaybeProp<bool>` | `false` | Whether the popover should be open initially. |
| on_open_change | `Option<Callback<bool>>` | — | Callback fired when the open state changes. |
| modal | `MaybeProp<bool>` | `false` | Whether the popover is modal (traps focus). |

### PopoverTrigger

A button that toggles the popover.
Renders a `<button>` element.

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |

### PopoverAnchor

An optional custom anchor for positioning the popover. When present, the popover positions itself relative to this element instead of the trigger.

### PopoverPortal

Portals the popover content to the document body.

| Prop | Type | Default | Description |
|---|---|---|---|
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the portal mounted when the popover is closed. |

### PopoverContent

The floating popover panel with focus management and keyboard dismissal.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the content mounted when the popover is closed. |
| on_open_auto_focus | `Option<Callback<ev::Event>>` | — | Called when focus moves into the popover on open. Call `prevent_default` to override. |
| on_close_auto_focus | `Option<Callback<ev::Event>>` | — | Called when focus returns to the trigger on close. Call `prevent_default` to override. |
| on_escape_key_down | `Option<Callback<ev::KeyboardEvent>>` | — | Called when Escape is pressed. Call `prevent_default` to prevent closing. |
| on_pointer_down_outside | `Option<Callback<ev::CustomEvent>>` | — | Called when clicking outside the content. Call `prevent_default` to prevent closing. |
| on_focus_outside | `Option<Callback<ev::CustomEvent>>` | — | Called when focus moves outside the content. |
| on_interact_outside | `Option<Callback<ev::CustomEvent>>` | — | Called on any interaction outside the content. |
| side | `Signal<Side>` | `Bottom` | Which side of the trigger to place the popover. |
| side_offset | `Signal<f64>` | `0.0` | Distance from the trigger in pixels. |
| align | `Signal<Align>` | `Center` | Alignment along the side axis. |
| align_offset | `Signal<f64>` | `0.0` | Offset along the alignment axis in pixels. |
| avoid_collisions | `Signal<bool>` | `true` | Whether to flip sides when the popover would overflow. |
| collision_padding | `Signal<Padding>` | `0.0` | Space to maintain from the edge of the collision boundary. |
| arrow_padding | `Signal<f64>` | `0.0` | Minimum distance between the arrow and the popover edges. |
| sticky | `Signal<Sticky>` | `Partial` | Whether the popover stays in view when scrolling. |
| hide_when_detached | `Signal<bool>` | `false` | Whether to hide the popover when the trigger scrolls out of view. |

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |
| data-side | "top", "right", "bottom", or "left". |
| data-align | "start", "center", or "end". |

### PopoverClose

A button that closes the popover.
Renders a `<button>` element.

### PopoverArrow

An optional arrow pointing from the popover toward the trigger.
Renders an `<svg>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| width | `MaybeProp<f64>` | — | The width of the arrow in pixels. |
| height | `MaybeProp<f64>` | — | The height of the arrow in pixels. |
