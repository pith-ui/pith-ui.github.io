---
title: Tabs
description: A component for toggling between related panels on the same page.
features:
  - Accessible
  - Keyboard navigation
  - Automatic & manual activation
  - Horizontal & vertical
  - RTL support
---

# Tabs

## Demo

### Basic

A tabbed interface with three panels and keyboard navigation.

<!-- demo: tabs_basic -->

## Anatomy

Import the component and assemble its parts:

```rust
use pith_ui::tabs::*;

<Tabs>
    <TabsList>
        <TabsTrigger />
    </TabsList>
    <TabsContent />
</Tabs>
```

## Examples

### Vertical orientation

Use the `orientation` prop to arrange tabs vertically.

```rust
<Tabs orientation=Orientation::Vertical>
    <TabsList>
        <TabsTrigger value="one">"One"</TabsTrigger>
        <TabsTrigger value="two">"Two"</TabsTrigger>
    </TabsList>
    <TabsContent value="one">"Panel one"</TabsContent>
    <TabsContent value="two">"Panel two"</TabsContent>
</Tabs>
```

### Manual activation

By default, tabs activate when they receive focus (automatic mode). Set `activation_mode` to `Manual` so tabs only activate on click or <kbd>Enter</kbd>/<kbd>Space</kbd>.

```rust
<Tabs activation_mode=ActivationMode::Manual>
    // ...
</Tabs>
```

### Disabled tabs

Individual triggers can be disabled. Disabled tabs are skipped during keyboard navigation.

```rust
<TabsTrigger value="drafts" disabled=true>
    "Drafts"
</TabsTrigger>
```

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### Tabs

Groups the tab triggers and their corresponding panels. Manages the active tab state.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| value | `MaybeProp<String>` | — | The controlled value of the active tab. |
| default_value | `MaybeProp<String>` | — | The value of the tab that should be active initially. |
| on_value_change | `Option<Callback<String>>` | — | Callback fired when the active tab changes. |
| orientation | `MaybeProp<Orientation>` | `Horizontal` | The orientation of the tabs. |
| dir | `MaybeProp<Direction>` | — | The reading direction (ltr or rtl). |
| activation_mode | `MaybeProp<ActivationMode>` | `Automatic` | Whether tabs activate on focus (Automatic) or on click/Enter (Manual). |

| Attribute | Description |
|---|---|
| data-orientation | "horizontal" or "vertical". |

### TabsList

Contains the tab triggers.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| loop | `MaybeProp<bool>` | `true` | Whether keyboard navigation loops back to the first tab at the end. |

### TabsTrigger

An interactive button that activates its associated content panel.
Renders a `<button>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| value | `String` | — | A unique value that identifies this tab (required). |
| disabled | `MaybeProp<bool>` | `false` | Whether the trigger is disabled. |

| Attribute | Description |
|---|---|
| data-state | "active" or "inactive". |
| data-disabled | Present when the trigger is disabled. |
| data-orientation | "horizontal" or "vertical". |

### TabsContent

A panel displayed when the corresponding tab is active.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| value | `String` | — | The value matching the associated trigger (required). |
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the content mounted in the DOM when inactive. |

| Attribute | Description |
|---|---|
| data-state | "active" or "inactive". |
| data-orientation | "horizontal" or "vertical". |
