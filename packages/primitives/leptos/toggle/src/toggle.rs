use leptos::{ev::MouseEvent, html, prelude::*};
use leptos_maybe_callback::MaybeCallback;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_primitive::{Primitive, compose_callbacks};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};

// We define a name for the “Root” like in your Tabs code
// const TOGGLE_NAME: &str = "Toggle";

#[component]
pub fn Toggle(
    #[prop(into, optional)] pressed: MaybeProp<bool>,
    #[prop(into, optional, default = false)] default_pressed: bool,
    #[prop(into, optional)] on_pressed_change: MaybeCallback<bool>,
    #[prop(into, optional, default = false)] disabled: bool,
    #[prop(into, optional)] on_click: MaybeCallback<MouseEvent>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    // #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let disabled = Signal::derive(move || disabled);

    let (pressed, set_pressed) = use_controllable_state(UseControllableStateParams {
        prop: pressed,
        default_prop: default_pressed,
        on_change: on_pressed_change,
    });

    let handle_press = Callback::new(move |_| {
        if !disabled.get() {
            let new_val = !pressed.get();
            set_pressed.run(new_val);
            // if is_form_control.get() {
            //     // If switch is in a form, stop propagation from the button, so that we only propagate
            //     // one click event (from the input). We propagate changes from an input so that native
            //     // form validation works and form events reflect switch updates.
            //     ev.stop_propagation();
            // }
        }
    });

    let attrs = view! {
        <{..}
            disabled=disabled
            data-state=move || if pressed.get() { "on" } else { "off" }
            data-disabled=move || disabled.get().then_some("")
            aria-pressed=move || pressed.with(|p| p.to_string())
        />
    };

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref={node_ref}
            {..attrs}
            on:click=compose_callbacks(on_click.into(), Some(handle_press), None)
        >
            {children()}
        </Primitive>
    }
}
