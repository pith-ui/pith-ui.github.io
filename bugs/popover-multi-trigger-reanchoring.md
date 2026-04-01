# Feature request: Popover multi-trigger re-anchoring

**Component:** `Popover`
**Severity:** minor
**Date:** 2026-03-31

## Summary

When multiple `PopoverTrigger` components are placed inside a single `Popover` root, the popup does not re-anchor to the trigger that was clicked. It always anchors to a single position regardless of which trigger opened it. Base-ui supports this via `triggerId` on `Popover.Root` and `id` on each `Popover.Trigger`.

## Expected behavior

Clicking different triggers should reposition the popover to anchor to the clicked trigger. A `trigger_id` prop (or equivalent mechanism) on `Popover` would control which trigger is the active anchor, and `on_open_change` would report which trigger initiated the state change.

## Actual behavior

`PopoverTrigger` composes with a single `context.trigger_ref` (`AnyNodeRef`) at mount time. All triggers write to the same ref, so the popper anchors to whichever trigger element the ref last resolved to — effectively the last one in DOM order. Clicking a different trigger toggles the open state but does not change the anchor position.

## Analysis

**Root cause:** `PopoverContextValue` stores one `trigger_ref: AnyNodeRef`. Each `PopoverTrigger` calls `use_composed_refs(vec![node_ref, context.trigger_ref])`, meaning they all share the same underlying ref. The Popper reads this ref once for positioning. There is no mechanism to update the ref to a different DOM element after mount.

**What base-ui does:** Base-ui's `Popover.Root` accepts a `triggerId` string prop. When it changes, the internal positioning logic looks up the trigger DOM element matching that id and re-anchors the popup. `onOpenChange` receives `eventDetails.trigger` so the consumer can track which trigger fired.

**What pith-ui would need:**
1. A `trigger_id` prop on `Popover` (or `PopoverTrigger` registering itself with a unique id).
2. Internal state mapping trigger ids to their DOM elements.
3. When `trigger_id` changes, update the popper's anchor reference and recompute position.
4. `on_open_change` callback should include the trigger element or id that initiated the change.

**Layer:** Library (pith-ui core). The `trigger_ref` is internal to the popover context and cannot be swapped from userland.

## Additional: PopoverAnchor positioning not working

Using `PopoverAnchor` with a plain button (no `PopoverTrigger`) and controlled state also fails to position the popover correctly. The popover does not anchor to the `PopoverAnchor` element as expected. This blocks the "detached triggers" pattern where the trigger is a plain element outside the normal `PopoverTrigger` flow. The root cause may be the same ref-chain issue, or a separate bug in how `PopperAnchor` connects the ref when no `PopoverTrigger` is present.

## Workarounds

Separate `Popover` instances per trigger with a shared signal ensure only one is open at a time. Each instance has its own trigger and anchors correctly, but the approach duplicates portal/content and doesn't match the "one popover, many triggers" pattern.

## References

- Base-ui Popover docs: "Multiple triggers" and "Controlled mode with multiple triggers" examples
- Base-ui reference file: `.claude/reference/base-ui/popover.md` lines 1145-1326
- Pith-ui PopoverTrigger source: `packages/primitives/leptos/src/components/popover/popover.rs` lines 88-166
- Related: `popover-viewport-animation-support.md` (Viewport also depends on multi-trigger infrastructure)
