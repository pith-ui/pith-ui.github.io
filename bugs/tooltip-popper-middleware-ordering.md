# Tooltip does not flip to opposite side when overflowing viewport

**Component:** `Tooltip` (via shared `Popper` module)
**Severity:** major
**Date:** 2026-03-31

## Summary

The tooltip always renders on its default side (`Top`) even when there is insufficient room in the viewport. The `avoid_collisions` prop is `true` by default and the `Flip` middleware is present, but the middleware chain ordering prevents it from working correctly. This affects all components using the shared `Popper` module (Tooltip, Popover, etc.).

## Expected behavior

When a tooltip with `side=Top` (default) would overflow the top of the viewport, the `Flip` middleware should automatically reposition it to the `Bottom` side. The tooltip documentation claims "collision-aware positioning" and the `avoid_collisions` prop description says "Whether to flip sides when the tooltip would overflow."

## Actual behavior

The tooltip renders above the trigger regardless of available space. When the trigger is near the top of the viewport, the tooltip overflows off-screen.

## Steps to reproduce

1. Open the Tooltip demo page
2. Scroll the page so a tooltip trigger is near the top of the viewport
3. Hover the trigger — the tooltip renders above, extending past the viewport edge
4. It does not flip to below the trigger

## Analysis

The root cause is in the shared `Popper` module's middleware chain ordering.

**File:** `packages/primitives/leptos/src/support/popper/mod.rs`, lines 247–277

The middleware is assembled in this order:
1. `Offset`
2. **`Shift`** (main_axis: true, cross_axis: false)
3. **`Flip`**
4. `Size`

The floating-ui documentation recommends the order: `Offset` → **`Flip`** → **`Shift`** → `Size`.

### Why the order matters

When `Shift` runs before `Flip`:

1. **Shift** with `main_axis: true` adjusts the tooltip along the main axis (vertical for `Top`/`Bottom` placements). It slides the tooltip to try to keep it within the viewport boundary.
2. After Shift has adjusted the position, **Flip** evaluates whether the tooltip overflows. But Shift may have already moved the tooltip enough that Flip's overflow detection no longer triggers — even though the tooltip is in a suboptimal position.

When `Flip` runs before `Shift` (correct order):

1. **Flip** evaluates whether the original side has enough space. If not, it switches to the opposite side (e.g., `Top` → `Bottom`).
2. **Shift** then fine-tunes the position on the chosen side, sliding it along the cross axis to avoid edge overflow.

### Code

```rust
// popper/mod.rs lines 254-277
if avoid_collisions.get() {
    let mut shift_options = ShiftOptions::default()
        .detect_overflow(detect_overflow_options.clone())
        .main_axis(true)    // ← slides along main axis
        .cross_axis(false);

    // ... limiter setup ...

    middleware.push(Box::new(Shift::new(shift_options)));  // ← runs BEFORE Flip

    middleware.push(Box::new(Flip::new(
        FlipOptions::default().detect_overflow(detect_overflow_options.clone()),
    )));  // ← runs AFTER Shift
}
```

### Scope of impact

This affects every component that uses the `Popper` module with `avoid_collisions=true`:
- `Tooltip` (default side: `Top`)
- `Popover`
- Any future components using `PopperContent`

## Workarounds

None currently. The docs site tooltips use the default `side=Top` with no workaround.

## References

- `packages/primitives/leptos/src/support/popper/mod.rs:247-277` — middleware chain assembly
- `packages/primitives/leptos/src/components/tooltip/tooltip_content.rs:49-54` — tooltip default props
- floating-ui middleware ordering documentation: Offset → Flip → Shift is the recommended order
