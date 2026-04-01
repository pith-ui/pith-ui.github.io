# AccordionMultiple: opening a sibling item replays animation on already-open items

**Component:** `AccordionMultiple`, `CollapsibleContent`
**Severity:** major
**Date:** 2026-03-31

## Summary

When using `AccordionMultiple`, opening a second accordion item causes the already-open first item's content to replay its open animation (visually collapsing to height:0 and re-expanding). This makes the multi-panel accordion unusable with CSS animations. The same issue affects any consumer of `CollapsibleContent` within a shared reactive context where sibling state changes re-trigger effects on unchanged items.

## Expected behavior

Opening item B while item A is already open should only animate item B's content. Item A's content should remain visually stable with no animation replay.

## Actual behavior

Opening item B causes item A's content to visually collapse to height:0 and re-expand (the `slide-down` animation replays from the beginning). This continues to compound — opening a third item replays the animation on all previously-open items simultaneously.

The issue also manifests in any component that uses `AccordionMultiple` with animated `AccordionContent`, including the API reference prop tables on the docs site (which use `AccordionMultiple` internally in `src/components/demo.rs`).

## Steps to reproduce

1. Render an `AccordionMultiple` with 3+ items, each with `AccordionContent` using a CSS class that applies a keyframe animation on `[data-state='open']`
2. Click to open item 1 — it animates open correctly
3. Click to open item 2 — item 2 animates open, but item 1 also replays its open animation
4. Click to open item 3 — items 1 and 2 both replay their open animations

## Analysis

**The bug is in the pith-ui library's `CollapsibleContent` implementation**, not in the consuming CSS or component usage.

### Source location

`packages/primitives/leptos/src/components/collapsible/mod.rs`, lines 250-317 (in git revision `854ff63`).

### Root cause chain

1. `CollapsibleContentImpl` contains a Leptos `Effect` (line 250) that measures the content's dimensions and manages animation blocking:

   ```rust
   Effect::new(move |_| {
       let _open = context.open.get();   // subscribes to open signal
       let _present = is_present.get();  // subscribes to presence signal
       // ...
   });
   ```

2. This effect subscribes to `context.open` by calling `.get()`. In the `AccordionMultiple` context, each item's `open` state is a derived signal that reads from the parent accordion's `values: RwSignal<Vec<String>>` to determine whether its own value is in the set.

3. When any item is opened or closed, the parent `values` signal changes. This causes **every** item's derived `open` signal to notify its subscribers — even items whose boolean `open` value hasn't actually changed (`true` → `true` for already-open items).

4. When the effect re-runs on an already-open item, it executes this sequence (lines 273-314):
   - Sets `animation-name: none` on the element (to freeze it for measurement)
   - Measures `getBoundingClientRect()` and updates `--collapsible-content-height`
   - Restores the original `animation-name` value

5. The browser interprets the restoration of `animation-name` (from `none` back to e.g. `accordion-open`) as a new animation assignment and **replays the animation from the beginning**.

### Why this doesn't affect AccordionSingle

In `AccordionSingle`, only one item can be open. When a new item opens, the previously-open item's `open` state changes from `true` to `false` (legitimate state change — it should animate closed). The newly-open item changes from `false` to `true` (legitimate — it should animate open). There are no items that stay `true` across the transition, so the replay issue never surfaces.

### Layer

This is a **library-level bug** in the reactivity/effect design of `CollapsibleContent`. The CSS animations are behaving correctly — they're replaying because the component's JavaScript is explicitly removing and re-applying the `animation-name` style property on elements whose state hasn't changed.

## Workarounds

The docs site currently uses `force_mount=true` on `AccordionContent` combined with CSS `transition` on `height` instead of keyframe animations:

```css
.accordion-content {
    overflow: hidden;
    height: 0;
    transition: height 200ms ease-out;
}
.accordion-content[data-state='open'] {
    height: var(--accordion-content-height);
}
```

This works because:
- `force_mount` keeps the DOM element stable (no mount/unmount cycles)
- CSS transitions only fire when the property value actually changes, so the effect's animation-name manipulation doesn't cause a visible replay (transitions don't use `animation-name`)
- The tradeoff is that closed content stays in the DOM (accessible to screen readers, increases DOM size)

**Files using this workaround** (remove `force_mount` and revert CSS to keyframes once the library fix lands):
- `src/components/demo.rs` (API reference accordion)
- `src/pages/components/accordion/demos/accordion_basic.rs`
- `src/pages/components/accordion/demos/accordion_multiple.rs`
- `style/input.css` (accordion-content and collapsible-content rules)

## References

- CollapsibleContent source: `packages/primitives/leptos/src/components/collapsible/mod.rs:162-337` (rev `854ff63`)
- AccordionContent delegates to CollapsibleContent: `packages/primitives/leptos/src/components/accordion/accordion_item.rs:134-160`
- The measurement effect that causes the replay: `collapsible/mod.rs:250-317`
- CSS variable naming: pith-ui uses `--accordion-content-height` and `--collapsible-content-height` (no `--radix-` prefix as of rev `854ff63`)
