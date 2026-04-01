---
title: Dialog
description: A popup that opens on top of the entire page.
features:
  - Accessible
  - Focus trapping
  - Keyboard dismiss
  - Modal & non-modal
  - Animated
---

# Dialog

## Demo

### Basic

A modal dialog with a form, demonstrating overlay, focus trapping, and close buttons.

<!-- demo: dialog_basic -->

## Anatomy

Import the component and assemble its parts:

```rust
use pith_ui::dialog::*;

<Dialog>
    <DialogTrigger />
    <DialogPortal>
        <DialogOverlay />
        <DialogContent>
            <DialogTitle />
            <DialogDescription />
            <DialogClose />
        </DialogContent>
    </DialogPortal>
</Dialog>
```

## Examples

### Nested dialogs

You can nest dialogs within one another normally. Backdrops of child dialogs layer over the parent to present a clean visual stack.

<!-- demo: dialog_nested -->

### Controlled open state

Use `open` and `on_open_change` to control the dialog's open state programmatically. This is useful for opening the dialog imperatively from elsewhere in your app.

```rust
let (open, set_open) = signal(false);

<Dialog open=open on_open_change=set_open>
    <DialogTrigger>"Open"</DialogTrigger>
    <DialogPortal>
        <DialogOverlay />
        <DialogContent>
            <DialogTitle>"Controlled"</DialogTitle>
        </DialogContent>
    </DialogPortal>
</Dialog>
```

### Non-modal

Set `modal` to `false` to render a non-modal dialog that does not trap focus or add a backdrop. The user can still interact with the rest of the page while the dialog is open.

```rust
<Dialog modal=false>
    <DialogTrigger>"Open"</DialogTrigger>
    <DialogPortal>
        <DialogContent>
            <DialogTitle>"Settings"</DialogTitle>
            <DialogClose>"Close"</DialogClose>
        </DialogContent>
    </DialogPortal>
</Dialog>
```

### Custom focus management

Use `on_open_auto_focus` on `~DialogContent~` to redirect initial focus to a specific element when the dialog opens, or `on_close_auto_focus` to control where focus returns when it closes.

```rust
<DialogContent on_open_auto_focus=move |e: ev::Event| {
    e.prevent_default();
    // Focus a specific element instead of the first focusable one
}>
    // ...
</DialogContent>
```

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### Dialog

Root component that manages the dialog's open/close state.

| Prop | Type | Default | Description |
|---|---|---|---|
| open | `MaybeProp<bool>` | — | The controlled open state. |
| default_open | `MaybeProp<bool>` | `false` | Whether the dialog should be open initially. |
| on_open_change | `Option<Callback<bool>>` | — | Callback fired when the open state changes. |
| modal | `MaybeProp<bool>` | `true` | Whether the dialog is modal (traps focus, renders backdrop). |

### DialogTrigger

A button that opens the dialog.
Renders a `<button>` element.

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |

### DialogPortal

Portals the overlay and content to the document body.

| Prop | Type | Default | Description |
|---|---|---|---|
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the portal mounted when the dialog is closed. |

### DialogOverlay

A backdrop behind the dialog that covers the page.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the overlay mounted when the dialog is closed. |

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |

### DialogContent

The dialog panel with focus trapping and keyboard dismissal.
Renders a `<div>` element.

| Prop | Type | Default | Description |
|---|---|---|---|
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the content mounted when the dialog is closed. |
| on_open_auto_focus | `Option<Callback<ev::Event>>` | — | Called when focus moves into the dialog on open. Call `prevent_default` to override. |
| on_close_auto_focus | `Option<Callback<ev::Event>>` | — | Called when focus returns to the trigger on close. Call `prevent_default` to override. |
| on_escape_key_down | `Option<Callback<ev::KeyboardEvent>>` | — | Called when Escape is pressed. Call `prevent_default` to prevent closing. |
| on_pointer_down_outside | `Option<Callback<ev::CustomEvent>>` | — | Called when clicking outside the content. Call `prevent_default` to prevent closing. |
| on_interact_outside | `Option<Callback<ev::CustomEvent>>` | — | Called on any interaction outside the content. |

| Attribute | Description |
|---|---|
| data-state | "open" or "closed". |

### DialogTitle

An accessible title for the dialog. Linked via `aria-labelledby`.
Renders an `<h2>` element.

### DialogDescription

An accessible description for the dialog. Linked via `aria-describedby`.
Renders a `<p>` element.

### DialogClose

A button that closes the dialog.
Renders a `<button>` element.
