# Nested NavigationMenu submenu content does not render visibly

**Component:** `NavigationMenu`
**Severity:** major
**Date:** 2026-03-31

## Summary

When a `NavigationMenu` is nested inside another `NavigationMenu`'s content panel (e.g., a "Handbook" flyout inside an "Overview" dropdown), the nested menu's viewport content is not visible when the nested trigger is activated. The nested viewport either doesn't render or is clipped by the parent viewport.

## Expected behavior

Clicking the nested "Handbook" trigger should open a side panel (positioned via `left-full top-0` on the nested viewport) showing the nested menu's content links (Styling, Animation, Composition).

## Actual behavior

Clicking the nested "Handbook" trigger produces no visible change. The nested content panel does not appear. The parent viewport's content remains unchanged.

## Steps to reproduce

1. Open the NavigationMenu nested demo
2. Hover/click "Overview" to open the parent dropdown
3. Click "Handbook" inside the dropdown — no nested flyout appears

## Analysis

The nested demo (`navigation_menu_nested.rs`) uses a full `NavigationMenu` root component (not `NavigationMenuSub`) inside the parent's `NavigationMenuContent`:

```rust
// Line 40: nested NavigationMenu inside parent content
<NavigationMenu orientation=Orientation::Vertical attr:class="relative">
    <NavigationMenuItem>
        <NavigationMenuTrigger>...</NavigationMenuTrigger>
        <NavigationMenuContent>...</NavigationMenuContent>
    </NavigationMenuItem>
    <NavigationMenuViewport attr:class="... absolute left-full top-0 ..." />
</NavigationMenu>
```

Two issues contribute to the problem:

### 1. Parent viewport clips nested viewport (CSS interaction)

The parent `NavigationMenuViewport` has `overflow-hidden` (required for its own open/close animations). The nested viewport is positioned `absolute left-full top-0`, which extends it to the RIGHT of its positioned ancestor — outside the parent viewport's bounds. `overflow-hidden` clips this content entirely.

The parent viewport class:
```
nav-viewport absolute left-0 top-full z-50 mt-2.5 rounded-lg bg-white shadow-lg
shadow-slate-200 outline outline-1 outline-slate-200 overflow-hidden
```

### 2. Missing NavigationMenuList in nested menu

The nested `NavigationMenu` contains `NavigationMenuItem` directly without a wrapping `NavigationMenuList`. The `NavigationMenuList` component creates the `CollectionProvider` and `ViewportContentContextValue` that content components rely on to register themselves for viewport rendering. Without it, nested content may fail to register with the nested viewport.

### Which layer owns the bug

This is a **library architectural limitation**. The viewport-based rendering model assumes viewports don't nest. When content renders inside a parent viewport, any nested viewport is subject to the parent's `overflow-hidden`. The library would need either:
- Portal support for nested viewports (rendering them outside the parent viewport's DOM)
- A `NavigationMenuSub` variant designed for this nesting pattern that renders content differently

## Workarounds

The nested demo has been updated to use `NavigationMenuSub` with inline content rendering (no viewport). The sub-menu content expands below the "Handbook" trigger within the parent dropdown instead of flying out to the side. This avoids the parent viewport's `overflow-hidden` clipping issue. The chevron now points right (closed) and rotates to point down (open) to indicate inline expansion.

## References

- `pith-ui.github.io/src/pages/components/navigation_menu/demos/navigation_menu_nested.rs` — nested demo
- `pith-ui/packages/primitives/leptos/src/components/navigation_menu/navigation_menu_viewport.rs` — viewport rendering
- `pith-ui/packages/primitives/leptos/src/components/navigation_menu/navigation_menu_list.rs` — NavigationMenuProvider (context setup)
- `pith-ui/packages/primitives/leptos/src/components/navigation_menu/navigation_menu.rs:172-231` — NavigationMenuSub component

## Library Team Response

Bug 1: Nested NavigationMenu submenu content does not render — REAL BUG (demo/architectural)
                                                                                                                                                    
This is a real issue in the nested demo, but it's an architectural limitation that also exists in React Radix, not a code defect. Two problems
compound:

1. Missing NavigationMenuList — The nested NavigationMenu wraps NavigationMenuItem directly without a NavigationMenuList, so the CollectionProvider
and ViewportContentContextValue that content relies on to register with the viewport are never created.
2. Parent overflow-hidden clips nested viewport — Even if the list wrapper were added, the parent viewport's overflow-hidden (needed for its own
animations) clips any nested viewport positioned outside its bounds via absolute left-full.

React Radix has the same limitation. Their NavigationMenu viewport model was not designed for nesting viewports inside viewports. There's no portal
support for nested viewports in upstream Radix.

Workaround: Use NavigationMenuSub for nested menus (it has its own state management without viewport rendering), or render nested content inline
without a separate viewport. 