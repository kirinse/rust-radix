use leptos::{
    attr::{NextAttribute, custom::custom_attribute},
    prelude::*,
    svg,
};
use leptos_node_ref::AnyNodeRef;
use leptos_typed_fallback_show::TypedFallbackShow;
use radix_leptos_primitive::Primitive;

/* -------------------------------------------------------------------------------------------------
 * Arrow
 * -----------------------------------------------------------------------------------------------*/

const NAME: &str = "Arrow";

#[component]
pub fn Arrow(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into, optional, default=10.0.into())] width: MaybeProp<f64>,
    #[prop(into, optional, default=5.0.into())] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children);

    #[cfg(debug_assertions)]
    Effect::new(move |_| {
        leptos::logging::log!("[{NAME}] width: {:?}", width.get());
        leptos::logging::log!("[{NAME}] height: {:?}", height.get());
        leptos::logging::log!("[{NAME}] node_ref: {:?}", node_ref.get());
        leptos::logging::log!("[{NAME}] as_child: {:?}", as_child.get());
    });

    let attrs = custom_attribute("viewBox", "0 0 30 10")
        .add_any_attr(custom_attribute("preserveAspectRatio", "none"));

    view! {
        <Primitive
            element=svg::svg
            as_child={as_child}
            attr:width=move || width.get()
            attr:height=move || height.get()
            node_ref={node_ref}
            {..attrs}
        >
            <TypedFallbackShow
                when=move || as_child.get().unwrap_or_default()
                fallback=move || {
                    view! {
                        <polygon points="0,0 30,0 15,10" />
                    }
                }
            >
                {children.with_value(|maybe_children| maybe_children.as_ref().map(|child_fn| child_fn()))}
            </TypedFallbackShow>
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Primitive re-exports
 * -----------------------------------------------------------------------------------------------*/

pub mod primitive {
    pub use super::*;
    pub use Arrow as Root;
}
