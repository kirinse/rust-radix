use leptos::{attribute_interceptor::AttributeInterceptor, context::Provider, html, prelude::*};
use leptos_maybe_callback::MaybeCallback;
use leptos_node_ref::AnyNodeRef;
use leptos_typed_fallback_show::TypedFallbackShow;
use radix_leptos_context::create_context;
use radix_leptos_id::use_id;
use radix_leptos_primitive::Primitive;
// use radix_leptos_presence::Presence;
// ----------------------------------------
// Minimal placeholders for advanced Radix features
// ----------------------------------------

// 1) “Roving focus group” placeholder module
#[allow(unused)]
mod roving_focus_group {
    use leptos::prelude::*;

    #[component]
    pub fn Root(
        children: Children,
        // We ignore real roving focus props for now
        // #[prop(optional)] loop_: MaybeProp<bool>,
        // #[prop(optional)] orientation: MaybeProp<String>,
        // #[prop(optional)] dir: MaybeProp<String>,
    ) -> impl IntoView {
        view! { <div class="roving-focus-root">{children()}</div> }
    }

    #[component]
    pub fn Item(
        children: Children,
        // #[prop(into, optional)] focusable: MaybeProp<bool>,
        // #[prop(into, optional)] active: MaybeProp<bool>,
    ) -> impl IntoView {
        view! { <div class="roving-focus-item">{children()}</div> }
    }
}

// 2) “Presence” placeholder for mount/unmount animations
#[allow(unused)]
mod presence {
    use leptos::prelude::*;

    #[component]
    pub fn Presence(
        #[prop(into, optional)] present: MaybeProp<bool>,
        children: TypedChildrenFn<impl IntoView + 'static>,
    ) -> impl IntoView {
        // If present==true, render children; else hide them.
        // Real code might do advanced mount/unmount logic.
        let children = StoredValue::new(children.into_inner());
        view! { <Show when=move || present.get().unwrap_or_default()>{children.with_value(|c| c())}</Show> }
    }
}

// use roving_focus_group::{Item as RovingFocusItem, Root as RovingFocusRoot};
// use presence::Presence;

// ----------------------------------------
// TabsContext: track selected tab, orientation, etc.
// ----------------------------------------

#[derive(Clone)]
struct TabsContextValue {
    // This is a minimal approach that only tracks the “selected tab.”
    // Real Radix approach also tracks orientation, direction, activation, etc.
    selected: RwSignal<Option<String>>,
    on_value_change: MaybeCallback<String>,
    orientation: String,
    #[allow(dead_code)] // TODO: remove this
    dir: String,
    #[allow(dead_code)] // TODO: remove this
    activation_mode: String,
    base_id: String,
}

const TABS_NAME: &str = "Tabs";

create_context!(
    context_type: TabsContextValue,
    provider: TabsProvider,
    hook: use_tabs_context,
    root: TABS_NAME
);

// ----------------------------------------
// Root
// ----------------------------------------

#[component]
pub fn Tabs(
    /// The currently selected value (if controlled)
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// The default value if uncontrolled
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Called when the selected tab changes
    #[prop(into, optional)]
    on_value_change: MaybeCallback<String>,
    /// orientation: "horizontal" or "vertical"
    #[prop(into, optional)]
    orientation: MaybeProp<String>,
    /// direction: "ltr" or "rtl"
    #[prop(into, optional)]
    dir: MaybeProp<String>,
    /// activationMode: "automatic" or "manual"
    #[prop(into, optional)]
    activation_mode: MaybeProp<String>,
    #[prop(into, optional)] _class: MaybeProp<String>,

    children: ChildrenFn,
    node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let orientation = orientation
        .get()
        .unwrap_or_else(|| "horizontal".to_string());
    let dir = dir.get().unwrap_or_else(|| "ltr".to_string());
    let activation_mode = activation_mode
        .get()
        .unwrap_or_else(|| "automatic".to_string());

    // Local signal that stores the actual “currently selected tab”
    let fallback = default_value.get().unwrap_or_default();
    let selected = RwSignal::new(value.get().clone().or(Some(fallback)));

    // If “value” changes, we update selected
    Effect::new({
        #[cfg(debug_assertions)]
        leptos::logging::log!("{:?}", value.get());
        move |_| {
            if let Some(new_val) = value.get().clone() {
                selected.set(Some(new_val));
            }
        }
    });

    // Render the root container
    view! {
        <TabsProvider value=TabsContextValue {
            selected,
            on_value_change,
            orientation: orientation.clone(),
            dir: dir.clone(),
            activation_mode: activation_mode.clone(),
            base_id: use_id(None).get(),
        }>
            <Primitive
                element={html::div}
                node_ref={node_ref}
                {..}
                data-orientation=move || orientation.clone()
                data-dir=move || dir.clone()
                data-activation-mode=move || activation_mode.clone()
            >
                {children.with_value(|c| c())}
            </Primitive>
        </TabsProvider>
    }
}

// ----------------------------------------
// List
// ----------------------------------------
const TAB_LIST_NAME: &str = "TabsList";

#[component]
pub fn TabsList(
    /// If we replicate roving focus props, we can define them here
    #[prop(optional, default = true)]
    _loop: bool,
    children: TypedChildrenFn<impl IntoView + 'static>,
    node_ref: AnyNodeRef,
) -> impl IntoView {
    let TabsContextValue {
        orientation: _,
        dir: _,
        ..
    } = use_tabs_context(TAB_LIST_NAME);
    let children = StoredValue::new(children.into_inner());
    view! {
        <AttributeInterceptor let:attr>
            // <RovingFocusRoot
            // // loop_=loop_
            // // orientation=orientation
            // // dir=dir
            // >
            <Primitive element={html::div} node_ref={node_ref} {..attr} attr:role="tablist">
                {children.with_value(|c| c())}
            </Primitive>
        // </RovingFocusRoot>
        </AttributeInterceptor>
    }
}

// ----------------------------------------
// Trigger
// ----------------------------------------
const TAB_TRIGGER_NAME: &str = "TabsTrigger";

#[component]
pub fn TabsTrigger(
    #[prop(into)] value: String,
    #[prop(optional, default = false)] disabled: bool,
    children: TypedChildrenFn<impl IntoView + 'static>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let TabsContextValue {
        selected,
        base_id,
        on_value_change,
        ..
    } = use_tabs_context(TAB_TRIGGER_NAME);
    // Check if selected
    let value = StoredValue::new(value);
    let children = StoredValue::new(children.into_inner());
    let is_selected = Signal::derive(move || {
        selected
            .get()
            .as_ref()
            .map(|s| s == &value.with_value(|v| v.clone()))
            .unwrap_or(false)
    });

    // On click, we update selected and call on_value_change
    let on_click = Callback::<()>::new(move |()| {
        if !disabled {
            #[cfg(debug_assertions)]
            leptos::logging::log!("clicked");
            #[cfg(debug_assertions)]
            leptos::logging::log!("selected {:?}", selected.get());
            selected.set(Some(value.with_value(|v| v.clone())));
            on_value_change.run(value.with_value(|v| v.clone()));
        }
    });

    // Effect to print current
    #[cfg(debug_assertions)]
    Effect::new({
        let value = value.with_value(|c| c.clone());
        move |_| {
            leptos::logging::log!("value: {:?}", value);
            leptos::logging::log!("selected {:?}", selected.get());
        }
    });
    let trigger_id = StoredValue::new(make_trigger_id(&base_id, &value.with_value(|v| v.clone())));
    let content_id = StoredValue::new(make_content_id(&base_id, &value.with_value(|v| v.clone())));

    // If we do real roving focus, we wrap in <RovingFocusItem> too
    view! {
        <AttributeInterceptor let:attr>
            // <RovingFocusItem
            // // focusable=move || !disabled
            // // active=move || is_selected.get()
            // >
            <Primitive
                element={html::button}
                node_ref={node_ref}
                {..attr}
                attr:r#type="button"
                attr:role="tab"
                attr:aria-selected=move || is_selected.get().to_string()
                attr:aria-controls=move || content_id.with_value(|v| v.clone())
                attr:data-state=move || if is_selected.get() { "active" } else { "inactive" }
                attr:data-disabled=move || disabled
                attr:disabled=disabled
                attr:id=move || trigger_id.with_value(|v| v.clone())
                on:click=move |_| on_click.run(())
            >
                {children.with_value(|c| c())}
            </Primitive>
        // </RovingFocusItem>
        </AttributeInterceptor>
    }
}

// ----------------------------------------
// Content
// ----------------------------------------
const TAB_CONTENT_NAME: &str = "TabsContent";

#[component]
pub fn TabsContent(
    value: String,
    #[prop(optional, default = false)] force_mount: bool,
    children: TypedChildrenFn<impl IntoView + 'static>,
    node_ref: AnyNodeRef,
) -> impl IntoView {
    let TabsContextValue {
        selected,
        orientation,
        dir: _,
        base_id,
        ..
    } = use_tabs_context(TAB_CONTENT_NAME);
    let value = StoredValue::new(value);
    let children = StoredValue::new(children.into_inner());
    let trigger_id = make_trigger_id(&base_id, &value.with_value(|v| v.clone()));
    let content_id = make_content_id(&base_id, &value.with_value(|v| v.clone()));
    let is_selected = Signal::derive(move || {
        selected
            .get()
            .is_some_and(|s| s == value.with_value(|v| v.clone()))
    });

    // Effect to print current
    #[cfg(debug_assertions)]
    Effect::new({
        move |_| {
            leptos::logging::log!("content value: {:?}", value.with_value(|v| v.clone()));
            leptos::logging::log!("content selected {:?}", selected.get());
        }
    });

    // Basic presence approach
    view! {
        <Primitive
            element={html::div}
            node_ref={node_ref}
            {..}
            id=content_id
            role="tabpanel"
            data-state=move || if is_selected.get() { "active" } else { "inactive" }
            data-orientation=move || orientation.clone()
            aria-hidden=move || (!is_selected.get()).to_string()
            aria-labelledby=trigger_id
        >
            <TypedFallbackShow when=move || is_selected.get() || force_mount fallback=|| ()>
                {children.with_value(|c| c())}
            </TypedFallbackShow>
        </Primitive>
    }
}

// ----------------------------------------
// Utilities
// ----------------------------------------
fn make_trigger_id(base_id: &str, value: &str) -> String {
    format!("{}-trigger-{}", base_id, value)
}

fn make_content_id(base_id: &str, value: &str) -> String {
    format!("{}-content-{}", base_id, value)
}

// ----------------------------------------
// Re-exports
// ----------------------------------------
pub mod primitive {
    pub use super::*;
    pub use Tabs as Root;
    pub use TabsContent as Content;
    pub use TabsList as List;
    pub use TabsTrigger as Trigger;
}
