# NavigationMenu viewport lacks collision-aware positioning

**Component:** `NavigationMenu`
**Severity:** major
**Date:** 2026-03-31

## Summary

The `NavigationMenuViewport` component does not implement any collision-aware (popper/floating) positioning logic. It only exposes `--navigation-menu-viewport-width` and `--navigation-menu-viewport-height` CSS custom properties, leaving all positioning to consumer CSS. This means the viewport always opens in a fixed direction (e.g., below the trigger via `top-full`) regardless of available viewport space.

## Expected behavior

When the navigation menu is near the bottom of the viewport and there isn't enough room below, the viewport should flip above the trigger (or reposition) to remain visible — similar to how `Tooltip` and `Popover` use the shared `Popper` module with floating-ui middleware for collision detection.

## Actual behavior

The viewport always renders in the direction specified by the consumer's CSS classes (e.g., `top-full` places it below). If there isn't enough room below the trigger, the viewport overflows off-screen or gets clipped, with no automatic repositioning.

## Steps to reproduce

1. Open the NavigationMenu demo page
2. Scroll the page so the navigation menu is near the bottom of the viewport
3. Open a menu trigger — the viewport renders below, extending past the viewport edge

## Analysis

The `NavigationMenuViewport` component (`navigation_menu_viewport.rs`) does not use the shared `Popper` module that `Tooltip`, `Popover`, and other floating components use. Its positioning logic is limited to:

- **Lines 94–111** (`NavigationMenuViewportImpl`): Sets `--navigation-menu-viewport-width` and `--navigation-menu-viewport-height` CSS custom properties based on `ResizeObserver` measurements of the active content
- **Lines 113–215**: Renders a `<div>` with `data-state` and `data-orientation` attributes, plus pointer event handlers

No `getBoundingClientRect()` calculations, no floating-ui middleware, and no dynamic repositioning occur. The Radix React equivalent also delegates positioning to CSS, so this matches upstream behavior — but consumers expect smart positioning similar to other pith-ui floating components that use the `Popper` module.

### Contrast with Tooltip/Popover

The shared `Popper` module (`support/popper/mod.rs`) uses `floating-ui-leptos` with `Offset`, `Shift`, `Flip`, and `Size` middleware for collision-aware positioning. `Tooltip` and `Popover` both wrap their content in `PopperContent`, gaining automatic side-flipping. `NavigationMenu` does not use `Popper` at all.

## Workarounds

None currently in use. The docs site positions the viewport with `top-full left-0 mt-2.5`, which works when the menu is near the top of the page.

## References

- `packages/primitives/leptos/src/components/navigation_menu/navigation_menu_viewport.rs` — viewport implementation
- `packages/primitives/leptos/src/support/popper/mod.rs` — shared popper module (not used by NavigationMenu)
- `packages/primitives/leptos/src/components/tooltip/tooltip_content.rs` — example of a component using Popper
