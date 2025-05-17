use leptos::{html::H1, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_slot::slot3::*;

#[component]
pub fn WithoutSlottable() -> impl IntoView {
    let node_ref = NodeRef::<H1>::new();
    let any_node_ref = AnyNodeRef::new();

    Effect::new(move || {
        leptos::logging::log!("stories::slot::node_ref={:?}", node_ref.get());
    });

    view! {
        <Slot owner_name="Slot_WithoutSlottable" node_ref=any_node_ref on:click=move |_| leptos::logging::log!("clicked")>
            <h1>ÀÁÂÂ</h1>
            <h2>BbbbbBBBb</h2>
        </Slot>
    }
}

#[component]
pub fn WithSlottable() -> impl IntoView {
    // let node_ref = NodeRef::<H1>::new();
    let any_node_ref = AnyNodeRef::new();

    view! {
        <Slot owner_name="Slot_WithSlottable" node_ref=any_node_ref on:click=move |_| leptos::logging::log!("clicked")>
            Hello
            <a href="https://radix-ui.com" data-slot=true>
                Button <em>text</em>
            </a>
            // <Slottable slot><h1>"slottable"</h1></Slottable>
            <span>world</span>
        </Slot>
    }
}
