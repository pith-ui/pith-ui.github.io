---
title: Switch
description: A control that indicates whether a setting is on or off.
features:
  - Accessible
  - Keyboard toggle
  - Form integration
  - Controlled & uncontrolled
---

# Switch

## Demo

### Basic

Controlled toggles for airplane mode and Wi-Fi, plus a disabled switch.

<!-- demo: switch_basic -->

## Anatomy

Import the component and assemble its parts:

```rust
use pith_ui::switch::*;

<Switch>
    <SwitchThumb />
</Switch>
```

## Examples

### With a label

Wrap with a `<label>` or use `~Label~` from `pith_ui::label` to associate an accessible label. Use the `for` attribute pointing to the switch's `id`.

```rust
use pith_ui::label::Label;

<Label r#for="notifications">"Notifications"</Label>
<Switch attr:id="notifications">
    <SwitchThumb />
</Switch>
```

### Form usage

Use the `name` and `value` props to include the switch in form submissions. The hidden input will submit the `value` when checked.

```rust
<Switch name="marketing" value="yes" default_checked=true>
    <SwitchThumb />
</Switch>
```

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### Switch

The root switch control.
Renders a `<button>` element with a hidden `<input>` for form integration.

| Prop | Type | Default | Description |
|---|---|---|---|
| checked | `MaybeProp<bool>` | — | The controlled checked state. |
| default_checked | `MaybeProp<bool>` | `false` | Whether the switch should be checked initially. |
| on_checked_change | `Option<Callback<bool>>` | — | Callback fired when the checked state changes. |
| disabled | `MaybeProp<bool>` | `false` | Whether the switch is disabled. |
| required | `MaybeProp<bool>` | `false` | Whether the switch is required in a form. |
| name | `MaybeProp<String>` | — | The name of the hidden input for form submission. |
| value | `MaybeProp<String>` | `"on"` | The value submitted with the form when checked. |
| form | `MaybeProp<String>` | — | The form element to associate with. |

| Attribute | Description |
|---|---|
| data-state | "checked" or "unchecked". |
| data-disabled | Present when the switch is disabled. |

### SwitchThumb

The sliding thumb indicator.
Renders a `<span>` element.

| Attribute | Description |
|---|---|
| data-state | "checked" or "unchecked". |
| data-disabled | Present when the switch is disabled. |
