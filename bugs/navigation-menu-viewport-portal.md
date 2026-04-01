# Feature Request: NavigationMenuViewportPortal for nested menus

**Component:** `NavigationMenu`
**Priority:** high
**Date:** 2026-03-31

## Problem

Nested `NavigationMenu` viewports cannot render as floating elements because the parent viewport's `overflow-hidden` (required for its slide/scale animations) clips the nested viewport. This makes nested flyout submenus — a common navigation pattern — non-functional.

The nested demo currently uses `NavigationMenuSub` with inline content rendering as a workaround, but this produces an inferior UX: the submenu renders inside the parent dropdown instead of as a separate floating panel, and lacks proper dismiss-on-blur behavior.

## Proposed solution

Add a `NavigationMenuViewportPortal` component that portals the nested viewport outside the parent viewport's DOM tree, following the same pattern already used by `DialogPortal`, `TooltipPortal`, `SelectPortal`, `PopoverPortal`, and `ComboboxPortal`.

### Usage

```rust
<NavigationMenu>
    <NavigationMenuList>
        <NavigationMenuItem>
            <NavigationMenuTrigger>"Overview"</NavigationMenuTrigger>
            <NavigationMenuContent>
                // Nested sub-navigation with portaled viewport
                <NavigationMenuSub orientation=Orientation::Vertical>
                    <NavigationMenuList>
                        <NavigationMenuItem>
                            <NavigationMenuTrigger>"Handbook"</NavigationMenuTrigger>
                            <NavigationMenuViewportPortal>
                                <NavigationMenuContent>
                                    // Nested links...
                                </NavigationMenuContent>
                            </NavigationMenuViewportPortal>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                    <NavigationMenuViewport />
                </NavigationMenuSub>
            </NavigationMenuContent>
        </NavigationMenuItem>
    </NavigationMenuList>
    <NavigationMenuViewport />
</NavigationMenu>
```

## Implementation guide

### Existing infrastructure to reuse

All of the following are already in the library and proven across multiple components:

| Infrastructure | Location | Used by |
|---|---|---|
| `ScopedPortal` | `support/portal/mod.rs` | Dialog, Tooltip, Select, Combobox, Popover |
| Context bridge pattern | `context_bridge` callback on `ScopedPortal` | All portal components |
| Collection scope capture | `use_collection_scope()` / `provide_collection_scope()` | Select, Combobox |
| Popper scope capture | `use_popper_scope()` / `provide_popper_scope()` | Select, Combobox |

### New code needed

**1. Scope capture type (~15 lines)**

Follow the `PopperScope` pattern (`support/popper/mod.rs:81-98`):

```rust
#[derive(Clone, Copy)]
pub struct NavigationMenuViewportScope {
    context: NavigationMenuContextValue,
    viewport_content: ViewportContentContextValue,
}

pub fn use_navigation_menu_viewport_scope() -> Option<NavigationMenuViewportScope> { ... }
pub fn provide_navigation_menu_viewport_scope(scope: NavigationMenuViewportScope) { ... }
```

**2. `NavigationMenuViewportPortal` component (~40 lines)**

Follow the `SelectPortal` pattern (`components/select/select_portal.rs`):

```rust
#[component]
pub fn NavigationMenuViewportPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let nav_context = expect_context::<NavigationMenuContextValue>();
    let viewport_content = expect_context::<ViewportContentContextValue>();
    let collection_scope = use_collection_scope::<FocusGroupItemData>();

    view! {
        <ScopedPortal
            container=container
            container_ref=container_ref
            force_mount=force_mount
            context_bridge=Callback::new(move |_| {
                provide_context(nav_context);
                provide_context(viewport_content);
                if let Some(scope) = collection_scope {
                    provide_collection_scope(scope);
                }
            })
        >
            {children()}
        </ScopedPortal>
    }
}
```

**3. Export the new component**

Add to `navigation_menu/mod.rs`:
- New file or inline in existing file
- `pub use` from the module root

### Contexts that must cross the portal boundary

| Context | Why |
|---|---|
| `NavigationMenuContextValue` | Menu state (value, orientation, callbacks, viewport ref) |
| `ViewportContentContextValue` | Content registration map — viewport reads this to render items |
| Collection scope (`FocusGroupItemData`) | Keyboard navigation within content |

### What does NOT need to change

- `NavigationMenuViewport` — continues to work as-is; it reads from context which is re-provided through the portal
- `NavigationMenuContent` — registration via `on_viewport_content_change` works through re-provided context
- `NavigationMenuSub` — already creates its own `NavigationMenuProvider`; the portal wraps its viewport output
- `Presence` / animation system — unaffected

## Testing considerations

Focus testing on areas where portal boundaries can break assumptions:

- **Keyboard navigation**: Arrow keys between parent and nested triggers, Tab/Escape behavior
- **Focus management**: `FocusGroup` / `FocusGroupItem` across the portal boundary
- **Hover events**: Pointer grace area between parent content and portaled nested viewport
- **Dismiss behavior**: Nested viewport should close when focus/pointer leaves both the trigger and the viewport
- **Animation**: `data-state` and `data-motion` transitions on the portaled viewport
- **Attribute forwarding**: `ForwardedAttrs` (spread attributes like `attr:class`) through the portal

## Scope estimate

~150-200 lines of new code. No new dependencies. No Leptos limitations — `mount_to()` owner boundary crossing is handled by the existing `context_bridge` pattern. The implementation closely follows `SelectPortal` as a template.

Optional follow-up: integrate `PopperContent` for collision-aware positioning of the nested viewport (separate from this feature, which uses CSS positioning).
