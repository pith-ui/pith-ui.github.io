---
title: Tooltip
description: A popup that appears when an element is hovered or focused, showing a hint for sighted users.
features:
  - Accessible
  - Delayed open
  - Keyboard support
  - Collision-aware positioning
---

# Tooltip

## Demo

### Basic

Tooltips on hover and focus, with a shared provider for coordinated open delays.

<!-- demo: tooltip_basic -->

## Anatomy

Import the component and assemble its parts:

```rust
use pith_ui::tooltip::*;

<TooltipProvider>
    <Tooltip>
        <TooltipTrigger />
        <TooltipPortal>
            <TooltipContent>
                <TooltipArrow />
            </TooltipContent>
        </TooltipPortal>
    </Tooltip>
</TooltipProvider>
```

## Examples

### Custom positioning

Use the `side`, `side_offset`, and `align` props on `~TooltipContent~` to control where the tooltip appears relative to the trigger.

```rust
<TooltipContent side=Side::Right side_offset=10.0 align=Align::Start>
    "Right-aligned tooltip"
    <TooltipArrow />
</TooltipContent>
```

### Custom delay

Override the provider's delay on a per-tooltip basis with the `delay_duration` prop.

```rust
<Tooltip delay_duration=0.0>
    <TooltipTrigger>"Instant"</TooltipTrigger>
    <TooltipPortal>
        <TooltipContent>"No delay"</TooltipContent>
    </TooltipPortal>
</Tooltip>
```

### Composing with other components

Use `as_child` on `~TooltipTrigger~` to compose the tooltip trigger with another component, such as a toolbar button.

```rust
<TooltipTrigger as_child=true>
    <button>"Save"</button>
</TooltipTrigger>
```

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### TooltipProvider

Wraps your app to provide shared tooltip delay behavior. All `~Tooltip~` instances inside a provider coordinate so that moving between triggers skips the open delay.

| Prop | Type | Default | Description |
|---|---|---|---|
| delay_duration | `MaybeProp<f64>` | `700.0` | How long to wait before opening a tooltip on hover, in milliseconds. |
| skip_delay_duration | `MaybeProp<f64>` | `300.0` | How long to skip the delay when moving between triggers, in milliseconds. |
| disable_hoverable_content | `MaybeProp<bool>` | `false` | Whether to disable the grace area that keeps the tooltip open when moving to its content. |

### Tooltip

Root component that manages the open/close state for a single tooltip.

| Prop | Type | Default | Description |
|---|---|---|---|
| open | `MaybeProp<bool>` | — | The controlled open state. |
| default_open | `MaybeProp<bool>` | `false` | Whether the tooltip should be open initially. |
| on_open_change | `Option<Callback<bool>>` | — | Callback fired when the open state changes. |
| delay_duration | `MaybeProp<f64>` | — | Override the provider's delay for this tooltip. |
| disable_hoverable_content | `MaybeProp<bool>` | — | Override the provider's hoverable content setting. |

### TooltipTrigger

The element that triggers the tooltip on hover or focus.
Renders a `<button>` element.

| Attribute | Description |
|---|---|
| data-state | "delayed-open", "instant-open", or "closed". |

### TooltipPortal

Portals the tooltip content to the document body.

| Prop | Type | Default | Description |
|---|---|---|---|
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the content mounted in the DOM when closed. |

### TooltipContent

The floating tooltip content. Positioned relative to the trigger using collision-aware anchoring.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| side | `Signal<Side>` | `Top` | Which side of the trigger to place the tooltip. |
| side_offset | `Signal<f64>` | `0.0` | Distance from the trigger in pixels. |
| align | `Signal<Align>` | `Center` | Alignment along the side axis. |
| align_offset | `Signal<f64>` | `0.0` | Offset along the alignment axis in pixels. |
| avoid_collisions | `Signal<bool>` | `true` | Whether to flip sides when the tooltip would overflow. |
| collision_padding | `Signal<Padding>` | `0.0` | Space to maintain from the edge of the collision boundary. |
| arrow_padding | `Signal<f64>` | `0.0` | Minimum distance between the arrow and the tooltip edges. |
| sticky | `Signal<Sticky>` | `Partial` | Whether the tooltip stays in view when scrolling. |
| hide_when_detached | `Signal<bool>` | `false` | Whether to hide the tooltip when the trigger scrolls out of view. |
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the content mounted when closed. |
| aria_label | `MaybeProp<String>` | — | Accessible label for the tooltip content. |

| Attribute | Description |
|---|---|
| data-state | "delayed-open", "instant-open", or "closed". |
| data-side | "top", "right", "bottom", or "left". |
| data-align | "start", "center", or "end". |

### TooltipArrow

An optional arrow pointing from the tooltip toward the trigger.
Renders an `<svg>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| width | `MaybeProp<f64>` | — | The width of the arrow in pixels. |
| height | `MaybeProp<f64>` | — | The height of the arrow in pixels. |
