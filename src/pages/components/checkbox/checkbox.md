---
title: Checkbox
description: A control that allows the user to toggle between checked and unchecked states.
features:
  - Accessible
  - Indeterminate state
  - Form integration
  - Controlled & uncontrolled
---

# Checkbox

## Demo

### Basic

A controlled checkbox, an uncontrolled checkbox, and a disabled checkbox with associated labels.

<!-- demo: checkbox_basic -->

## Anatomy

Import the component and assemble its parts:

```rust
use pith_ui::checkbox::*;

<Checkbox>
    <CheckboxIndicator />
</Checkbox>
```

## Examples

### Indeterminate

Use `CheckedState::Indeterminate` to render the checkbox in a mixed state. This is useful for "select all" checkboxes that control a group of child checkboxes.

<!-- demo: checkbox_indeterminate -->

### Form integration

Use the `name` and `value` props to include the checkbox in form submissions. The hidden input will submit the `value` when checked.

<!-- demo: checkbox_form -->

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### Checkbox

The root checkbox control.
Renders a `<button>` element with a hidden `<input>` for form integration.

| Prop | Type | Default | Description |
|---|---|---|---|
| name | `MaybeProp<String>` | — | Identifies the field when a form is submitted. |
| checked | `MaybeProp<CheckedState>` | — | The controlled checked state. |
| default_checked | `MaybeProp<CheckedState>` | `False` | The checked state when initially rendered. |
| on_checked_change | `Option<Callback<CheckedState>>` | — | Callback fired when the checked state changes. |
| required | `MaybeProp<bool>` | `false` | Whether the checkbox is required in a form. |
| disabled | `MaybeProp<bool>` | `false` | Whether the checkbox is disabled. |
| value | `MaybeProp<String>` | `"on"` | The value submitted with the form when checked. |
| form | `MaybeProp<String>` | — | The form element to associate with. |

| Attribute | Description |
|---|---|
| data-state | "checked", "unchecked", or "indeterminate". |
| data-disabled | Present when the checkbox is disabled. |

### CheckboxIndicator

Renders when the checkbox is checked or indeterminate. Place your check/minus icon inside.
Renders a `<span>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the indicator mounted when unchecked. |

| Attribute | Description |
|---|---|
| data-state | "checked", "unchecked", or "indeterminate". |
| data-disabled | Present when the checkbox is disabled. |
