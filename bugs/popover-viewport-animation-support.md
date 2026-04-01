# Feature request: Popover Viewport for direction-aware content animations

**Component:** `Popover`
**Severity:** minor
**Date:** 2026-03-31

## Summary

Base-ui's Popover has a `Popover.Viewport` component that enables direction-aware content transitions when a popover moves between multiple trigger elements. Pith-ui's Popover does not have an equivalent, making it impossible to replicate this example in the documentation.

## Expected behavior

A `PopoverViewport` component (or equivalent mechanism) that:

1. Wraps popover content and tracks when the active trigger changes.
2. Exposes a `data-activation-direction` attribute (`left`, `right`, `up`, `down`) indicating the new trigger's spatial position relative to the previous one.
3. Wraps content in inner elements with `data-current` (incoming content) and `data-previous` (outgoing content during a transition) attributes for CSS-driven enter/exit animations.

This would allow CSS transitions that slide content in the direction of the new trigger, creating a smooth navigational feel when a popover moves between toolbar icons or navigation items.

## Actual behavior

No `PopoverViewport` or equivalent exists. Popover content renders statically — when the popover reopens at a different anchor, there is no transition infrastructure and no way to know the spatial relationship between the previous and current trigger.

## Analysis

**Base-ui's implementation** (from their docs):

- `<Popover.Viewport>` wraps the content inside `<Popover.Popup>`.
- It renders a `<div>` with `data-activation-direction` computed from the bounding rects of the previous and current trigger elements.
- Inside the viewport, content is double-buffered: `data-current` shows the incoming content, `data-previous` shows the outgoing content during the transition, then is removed.
- CSS `@keyframes` reference these data attributes for directional slide animations.

**What pith-ui would need:**

- A new `PopoverViewport` component that sits inside `PopoverContent`.
- Internal state tracking the previous trigger element (or its position) when `trigger_id` changes.
- Spatial comparison logic: compare bounding rects of old vs new trigger to determine direction.
- DOM structure with `data-current`/`data-previous` wrappers for CSS transition hooks.

**Layer:** Library (pith-ui core). This cannot be implemented in userland because it requires access to the popover's internal trigger tracking and the popup's DOM structure.

## Workarounds

None. The "Animating the Popover" example from base-ui is omitted from the pith-ui documentation.

## References

- Base-ui Popover docs: `Popover.Viewport` section and "Animating the Popover" example
- Base-ui reference file: `.claude/reference/base-ui/popover.md` lines 1688-1712
- Pith-ui Popover source: `packages/primitives/leptos/src/components/popover/`
