use leptos::{
    html::ElementType,
    prelude::{Effect, NodeRef},
    tachys::html::node_ref::NodeRefContainer,
};
use web_sys::wasm_bindgen::JsCast;

// TODO: Support ref functions? These take the node as argument.

fn compose_refs<E: ElementType>(refs: Vec<NodeRef<E>>) -> NodeRef<E>
where
    E:,
    E::Output: JsCast + 'static,
{
    let composed_ref: NodeRef<E> = NodeRef::new();

    Effect::new(move |_| {
        if let Some(node) = composed_ref.get() {
            for r#ref in &refs {
                r#ref.load(&node);
            }
        }
    });

    composed_ref
}

pub fn use_composed_refs<E: ElementType>(refs: Vec<NodeRef<E>>) -> NodeRef<E>
where
    E:,
    E::Output: JsCast + 'static,
{
    compose_refs(refs)
}
