use leptos::context::Provider;
use leptos::html;
use leptos::prelude::*;
use leptos_maybe_callback::MaybeCallback;
use leptos_node_ref::AnyNodeRef;

use radix_leptos_context::create_context;
use radix_leptos_primitive::Primitive;
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};

/// A context holding the switch state (checked/disabled) + callback for toggling.
#[derive(Clone)]
struct SwitchContextValue {
    checked: Signal<Option<bool>>,
    disabled: bool,
}

// We define a name for the “Root” like in your Tabs code
const SWITCH_NAME: &str = "Switch";

create_context!(
    context_type: SwitchContextValue,
    provider: SwitchProvider,
    hook: use_switch_context,
    root: SWITCH_NAME
);

/// The main “Radix Switch” (Root). If `checked` is Some(...), it’s controlled;
/// otherwise it’s uncontrolled using `default_checked`. We pass down a context
/// so the `<Thumb>` can read `checked`, `disabled`, etc.
#[component]
pub fn Switch(
    /// If fully controlled, pass `checked=Some(...)`
    #[prop(into, optional)]
    checked: MaybeProp<bool>,
    /// If uncontrolled, pass `default_checked`
    #[prop(into, optional, default = false.into())]
    default_checked: MaybeProp<bool>,
    /// Fired when toggled
    #[prop(into, optional)]
    on_checked_change: MaybeCallback<bool>,
    /// If true, switch is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// If true, switch is required
    #[prop(optional, default = false)]
    required: bool,
    /// Extra classes, e.g. "bg-gray-500", etc.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// The child elements, typically a single <Thumb>
    children: TypedChildrenFn<impl IntoView + 'static>,
    /// If you want a NodeRef
    #[prop(optional, into)]
    node_ref: AnyNodeRef,
) -> impl IntoView {
    // 1) figure out initial state
    // let on_checked_change = on_checked_change.as_callback();
    let children = StoredValue::new(children.into_inner());
    // let initial = checked
    //     .get()
    //     .unwrap_or(default_checked.get().unwrap_or(false));
    // let checked_signal = RwSignal::new(initial);

    let (checked, set_checked) = use_controllable_state(UseControllableStateParams {
        prop: checked,
        default_prop: default_checked,
        on_change: Some(Callback::new(move |value| {
            if let Some(value) = value {
                on_checked_change.run(value);
            }
        }))
        .into(),
    });

    // 2) If external “checked” changes, override local
    // Effect::new({
    //     let checked_signal = checked_signal.clone();
    //     move |_| {
    //         if let Some(external_val) = checked.get() {
    //             checked_signal.set(external_val);
    //         }
    //     }
    // });

    // 3) The on_click toggles if not disabled
    let handle_click = move |_| {
        if !disabled {
            let new_val = checked.with(|c| c.map(|c| !c));
            set_checked.run(new_val);
        }
    };
    let attrs = view! {
        <{..}
            type="button"
            role="switch"
            disabled=disabled
            data-state=move || if checked.with(|c| c == &Some(true)) { "checked" } else { "unchecked" }
            data-disabled=move || if disabled { Some("") } else { None }
            aria-checked=move || checked.with(|c| c.unwrap_or_default().to_string())
            aria-required=move || required.to_string()
        />
    };

    // 5) Render the “Root” as a <button>
    view! {
        <SwitchProvider value=SwitchContextValue {
            checked,
            disabled,
        }>
            <Primitive
                element={html::button}
                {..attrs}
                node_ref={node_ref}
                attr:class=move || {
                    format!("radix-leptos-switch-root {}", class.get().unwrap_or_default())
                }
                on:click=handle_click
            >
                {children.with_value(|c| c())}
            </Primitive>
        </SwitchProvider>
    }
}

// We define a name for the “Thumb”
const THUMB_NAME: &str = "SwitchThumb";

/// The “Thumb” that shows ON/OFF visually. Reads from `SwitchContextValue`.
#[component]
pub fn SwitchThumb(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let SwitchContextValue {
        checked, disabled, ..
    } = use_switch_context(THUMB_NAME);

    view! {
        <span
            role="presentation"
            data-state=move || if checked.with(|c| c == &Some(true)) { "checked" } else { "unchecked" }
            data-disabled=move || if disabled { Some("") } else { None }
            class=move || format!("radix-leptos-switch-thumb {}", class.get().unwrap_or_default())
        />
    }
}

// Re-export them under “primitive::Root” and “primitive::Thumb”
pub mod primitive {
    pub use super::Switch as Root;
    pub use super::SwitchThumb as Thumb;
}
