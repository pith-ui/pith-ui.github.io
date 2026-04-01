---
title: Progress
description: Displays the status of a task that takes a long time.
features:
  - Accessible
  - Determinate & indeterminate
  - Custom max value
---

# Progress

## Demo

### Basic

A progress bar that auto-increments to simulate an export task, with a label and value display.

<!-- demo: progress_basic -->

## Anatomy

Import the component and assemble its parts:

```rust
use pith_ui::progress::*;

<Progress>
    <ProgressIndicator />
</Progress>
```

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### Progress

Root progress container. Sets ARIA attributes and provides context.
Renders a `<div>` element with `role="progressbar"`.

| Prop | Type | Default | Description |
|---|---|---|---|
| value | `MaybeProp<f64>` | — | The current value. Omit for an indeterminate progress bar. |
| max | `MaybeProp<f64>` | `100.0` | The maximum value. |
| get_value_label | `Option<Callback<(f64, f64), String>>` | — | Custom accessible label function receiving (value, max). |

| Attribute | Description |
|---|---|
| data-state | "indeterminate", "loading", or "complete". |
| data-value | Current value (absent when indeterminate). |
| data-max | Maximum value. |

### ProgressIndicator

The visual indicator showing completion.
Renders a `<div>` element.

| Attribute | Description |
|---|---|
| data-state | "indeterminate", "loading", or "complete". |
| data-value | Current value (absent when indeterminate). |
| data-max | Maximum value. |
