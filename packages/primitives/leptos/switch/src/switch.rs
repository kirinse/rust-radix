use leptos::{
    context::Provider,
    ev::{Event, MouseEvent},
    html::{self, Input},
    prelude::*,
};
use leptos_maybe_callback::MaybeCallback;
use leptos_node_ref::AnyNodeRef;

use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_context::create_context;
use radix_leptos_primitive::{Primitive, compose_callbacks};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use radix_leptos_use_previous::use_previous;
use radix_leptos_use_size::use_size;

/// A context holding the switch state (checked/disabled) + callback for toggling.
#[derive(Clone)]
struct SwitchContextValue {
    checked: Signal<bool>,
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
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional, default = false)] default_checked: bool,
    #[prop(into, optional)] on_checked_change: MaybeCallback<bool>,
    #[prop(optional, default = false)] required: bool,
    #[prop(optional, default = false)] disabled: bool,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into, optional)] on_click: MaybeCallback<MouseEvent>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let name = Signal::derive(move || name.get());
    let value = Signal::derive(move || value.get().unwrap_or("on".into()));

    let children = StoredValue::new(children.into_inner());
    let button_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs([node_ref, button_ref]);

    let is_form_control = Signal::derive(move || {
        button_ref
            .get()
            .and_then(|button| button.closest("form").ok())
            .flatten()
            .is_some()
    });

    let (checked, set_checked) = use_controllable_state(UseControllableStateParams {
        prop: checked,
        default_prop: default_checked,
        on_change: on_checked_change,
    });
    
    let handle_click = Callback::new(move |ev: MouseEvent| {
        if !disabled {
            let new_val = !checked.get();
            set_checked.run(new_val);
            if is_form_control.get() {
                // If switch is in a form, stop propagation from the button, so that we only propagate
                // one click event (from the input). We propagate changes from an input so that native
                // form validation works and form events reflect switch updates.
                ev.stop_propagation();
            }
        }
    });

    let attrs = view! {
        <{..}
            type="button"
            role="switch"
            class=move || format!("radix-leptos-switch-root {}", class.get().unwrap_or_default())
            value=move || value.get()
            name=move || name.get()
            disabled=disabled
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
            data-disabled=move || disabled.then_some("")
            aria-checked=move || checked.with(|c| c.to_string())
            aria-required=move || required.to_string()
        />
    };

    // 5) Render the `Root` as a <button>
    view! {
        <SwitchProvider value=SwitchContextValue {
            checked,
            disabled,
        }>
            <Primitive
                element=html::button
                node_ref=composed_refs
                as_child={as_child}
                {..attrs}
                on:click=compose_callbacks(on_click.into(), Some(handle_click), None)
            >
                {children.with_value(|c| c())}
            </Primitive>
            <Show when=move || is_form_control.get()>
                <BubbleInput
                    attr:name=name
                    control_ref=button_ref
                    bubbles=Signal::derive(|| true)
                    value=value
                    checked=checked
                    required=required
                    disabled=disabled
                />
            </Show>
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
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
            data-disabled=move || disabled.then_some("")
            class=move || format!("radix-leptos-switch-thumb {}", class.get().unwrap_or_default())
        />
    }
}

#[component]
fn BubbleInput(
    #[prop(into)] control_ref: AnyNodeRef,
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] bubbles: Signal<bool>,
    #[prop(into)] required: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] value: Signal<String>,
    // #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let node_ref: NodeRef<Input> = NodeRef::new();
    let prev_checked = use_previous(checked);
    let control_size = use_size(control_ref);

    // Bubble checked change to parents
    Effect::new(move || {
        if let Some(input) = node_ref.get() {
            if prev_checked.get() != checked.get() {
                let init = web_sys::EventInit::new();
                init.set_bubbles(bubbles.get());

                let event = Event::new_with_event_init_dict("click", &init)
                    .expect("Click event should be instantiated.");

                input.set_checked(checked.get());

                input
                    .dispatch_event(&event)
                    .expect("Click event should be dispatched.");
            }
        }
    });

    let attrs = view! {
        <{..}
            type="checkbox"
            aria-hidden="true"
            checked=move || checked.get().then_some("")
            required=move || required.get().then_some("")
            disabled=move || disabled.get().then_some("")
            value=value
            tab-index="-1"
            // We transform because the input is absolutely positioned, but we have
            // rendered it **after** the button. This pulls it back to sit on top
            // of the button.
            style:transform="translateX(-100%)"
            style:width=move || control_size.get().map(|size| format!("{}px", size.width))
            style:height=move || control_size.get().map(|size| format!("{}px", size.height))
            style:position="absolute"
            style:pointer-events="none"
            style:opacity="0"
            style:margin="0px"
        />
    };

    view! {
        <input
            node_ref={node_ref}
            {..attrs}
        />
    }
}

// Re-export them under “primitive::Root” and “primitive::Thumb”
pub mod primitive {
    pub use super::Switch as Root;
    pub use super::SwitchThumb as Thumb;
}
