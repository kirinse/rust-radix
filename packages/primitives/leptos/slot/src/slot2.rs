use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;

#[slot]
pub struct Slot {
    #[prop(optional, into)]
    display_name: String,
    #[prop(optional_no_strip)]
    node_ref: AnyNodeRef,
    #[prop(optional)]
    children: Option<ChildrenFn>,
}

#[slot]
pub struct SlotClone {
    #[prop(optional, into)]
    display_name: String,
    children: Children,
}

#[allow(unused_variables)]
#[component]
pub fn Slottable(
    #[prop(optional, into)] display_name: String,
    #[prop(optional, into)] __radix_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    children()
}

// fn create_slot_clone(owner_name: String, children: Children) -> SlotClone {
//     SlotClone::builder()
//         .display_name(format!("{owner_name}.SlotClone"))
//         .children(children)
//         .build()
// }

pub fn get_element_ref(element: Slot) -> AnyNodeRef {
    element.node_ref
}

#[allow(unused_variables)]
#[component]
pub fn SlotWrapper(
    #[prop(into)] owner_name: String,
    slot: Slot,
    node_ref: AnyNodeRef,
) -> impl IntoView {
    if let Some(child) = slot.children {
        child()
    } else {
        ().into_any()
    }
}
