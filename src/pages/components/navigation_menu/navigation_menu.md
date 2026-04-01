---
title: Navigation Menu
description: A collection of links and menus for website navigation.
features:
  - Accessible
  - Keyboard navigation
  - Submenus with viewport
  - Active indicator
  - RTL support
---

# Navigation Menu

## Demo

### Basic

A navigation menu with expandable content panels and a plain link.

<!-- demo: navigation_menu_basic -->

## Anatomy

Import the component and assemble its parts:

```rust
use pith_ui::navigation_menu::*;

<NavigationMenu>
    <NavigationMenuList>
        <NavigationMenuItem>
            <NavigationMenuTrigger />
            <NavigationMenuContent>
                <NavigationMenuLink />
            </NavigationMenuContent>
        </NavigationMenuItem>
    </NavigationMenuList>
    <NavigationMenuIndicator />
    <NavigationMenuViewport />
</NavigationMenu>
```

## Examples

### Nested submenus

`~NavigationMenu~` can be nested within a higher-level `~NavigationMenuContent~` to create a multi-level navigation menu.

<!-- demo: navigation_menu_nested -->

### Nested inline submenus

For second-level navigation that should stay in the same panel, nest a `~NavigationMenu~` inside the content and render only `~NavigationMenuList~` + `~NavigationMenuViewport~` with a `default_value`.

<!-- demo: navigation_menu_inline -->

### Custom links

`~NavigationMenuLink~` renders an `<a>` element. Use the `as_child` prop to compose it with your router's link component for client-side routing.

```rust
<NavigationMenuLink as_child=true>
    <A href="/docs">"Documentation"</A>
</NavigationMenuLink>
```

### Large menus

When menu content doesn't fit in the viewport, you have two options:

**Compress the content** — change the layout to be more compact, or use `max-height` to limit the panel height and let it compress itself while preventing overflow.

```css
.content,
.popup {
    max-height: var(--available-height);
}
```

**Make it scrollable** — constrain the height and add `overflow-y: auto` to the content.

```css
.content,
.popup {
    max-height: var(--available-height);
}

.content {
    overflow-y: auto;
}
```

Native scrollbars are visible while transitioning content, so consider using a `~ScrollArea~` component instead to keep them hidden.

## API Reference

All components also accept `as_child`, `node_ref`, and `children` props.

### NavigationMenu

Groups all parts of the navigation menu. Manages open state and coordinates hover delays.
Renders a `<nav>` element.

| Prop                | Type                       | Default      | Description                                                                            |
| ------------------- | -------------------------- | ------------ | -------------------------------------------------------------------------------------- |
| value               | `MaybeProp<String>`        | —            | The controlled value of the active menu item.                                          |
| default_value       | `MaybeProp<String>`        | —            | The value of the item that should be active initially.                                 |
| on_value_change     | `Option<Callback<String>>` | —            | Callback fired when the active value changes.                                          |
| orientation         | `MaybeProp<Orientation>`   | `Horizontal` | The orientation of the navigation menu.                                                |
| dir                 | `MaybeProp<Direction>`     | —            | The reading direction of the menu (ltr or rtl).                                        |
| delay_duration      | `MaybeProp<f64>`           | `200.0`      | How long to wait before opening a menu on hover, in milliseconds.                      |
| skip_delay_duration | `MaybeProp<f64>`           | `300.0`      | How long to wait before skipping the delay when moving between items, in milliseconds. |

| Attribute        | Description                    |
| ---------------- | ------------------------------ |
| data-orientation | "horizontal" or "vertical".    |

### NavigationMenuSub

A sub-navigation for secondary menu lists. Use inside the content of a parent item.
Renders a `<div>` element.

| Prop            | Type                       | Default      | Description                                                |
| --------------- | -------------------------- | ------------ | ---------------------------------------------------------- |
| value           | `MaybeProp<String>`        | —            | The controlled value of the active sub-menu item.          |
| default_value   | `MaybeProp<String>`        | —            | The value of the sub-item that should be active initially. |
| on_value_change | `Option<Callback<String>>` | —            | Callback fired when the active sub-menu value changes.     |
| orientation     | `MaybeProp<Orientation>`   | `Horizontal` | The orientation of the sub-navigation.                     |

| Attribute        | Description                    |
| ---------------- | ------------------------------ |
| data-orientation | "horizontal" or "vertical".    |

### NavigationMenuList

Contains the list of navigation menu items.
Renders a `<ul>` element.

| Attribute        | Description                    |
| ---------------- | ------------------------------ |
| data-orientation | "horizontal" or "vertical".    |

### NavigationMenuItem

An individual navigation menu item.
Renders a `<li>` element.

| Prop  | Type                | Default | Description                                                               |
| ----- | ------------------- | ------- | ------------------------------------------------------------------------- |
| value | `MaybeProp<String>` | —       | A unique value that identifies this item. Auto-generated if not provided. |

### NavigationMenuTrigger

Opens the navigation menu content when hovered or clicked.
Renders a `<button>` element.

| Prop     | Type              | Default | Description                                         |
| -------- | ----------------- | ------- | --------------------------------------------------- |
| disabled | `MaybeProp<bool>` | `false` | Whether the trigger should ignore user interaction. |

| Attribute     | Description                                                           |
| ------------- | --------------------------------------------------------------------- |
| data-state    | "open" or "closed" depending on whether the content panel is visible. |
| data-disabled | Present when the trigger is disabled.                                 |

### NavigationMenuContent

The expandable content panel displayed when the parent item is active.
Renders a `<div>` element.

| Prop        | Type              | Default | Description                                                 |
| ----------- | ----------------- | ------- | ----------------------------------------------------------- |
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the content mounted in the DOM when closed. |

| Attribute        | Description                                                                             |
| ---------------- | --------------------------------------------------------------------------------------- |
| data-state       | "open" or "closed" depending on visibility.                                             |
| data-motion      | "from-start", "from-end", "to-start", or "to-end" indicating the transition direction. |
| data-orientation | "horizontal" or "vertical".                                                             |

### NavigationMenuLink

A link within the navigation menu. Closes the menu when clicked.
Renders an `<a>` element.

| Prop      | Type                          | Default | Description                                    |
| --------- | ----------------------------- | ------- | ---------------------------------------------- |
| active    | `MaybeProp<bool>`             | `false` | Whether the link is the currently active page. |
| on_select | `Option<Callback<ev::Event>>` | —       | Called when the link is selected.              |

| Attribute   | Description                                         |
| ----------- | --------------------------------------------------- |
| data-active | Present when the link is the currently active page. |

### NavigationMenuIndicator

An animated indicator that follows the currently active trigger.
Renders a `<div>` element.

| Prop        | Type              | Default | Description                                                   |
| ----------- | ----------------- | ------- | ------------------------------------------------------------- |
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the indicator mounted when no item is active. |

| Attribute        | Description                                                   |
| ---------------- | ------------------------------------------------------------- |
| data-state       | "visible" or "hidden" depending on whether an item is active. |
| data-orientation | "horizontal" or "vertical".                                   |

### NavigationMenuViewport

A shared viewport that displays the active item's content with animated transitions.
Renders a `<div>` element.

| Prop        | Type              | Default | Description                                                     |
| ----------- | ----------------- | ------- | --------------------------------------------------------------- |
| force_mount | `MaybeProp<bool>` | `false` | Whether to keep the viewport mounted when no content is active. |

| Attribute        | Description                                                 |
| ---------------- | ----------------------------------------------------------- |
| data-state       | "open" or "closed" depending on whether content is visible. |
| data-orientation | "horizontal" or "vertical".                                 |
