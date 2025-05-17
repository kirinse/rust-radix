use leptos::{
    html::{ElementType, HtmlElement},
    prelude::*,
    tachys::html::node_ref::NodeRefContainer,
};
use leptos_node_ref::{AnyNodeRef, AnyNodeRefAttr, IntoAnyNodeRef, any_node_ref};
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone)]
#[slot]
pub struct Slottable {
    #[prop(optional, into)]
    display_name: String,
    #[prop(optional, into)]
    __radix_id: String,
    children: ChildrenFn,
}

#[component]
pub fn Slot(
    #[prop(optional, into, default = "Slot".into())] owner_name: String,
    #[prop(optional)] slottable: Option<Slottable>,
    #[prop(optional_no_strip)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
// where
//     El: ElementType,
//     <El as ElementType>::Output: JsCast,
{
    let children_value = StoredValue::new(children);
    // any_node_ref(node_ref);
    let attrs = view! {<{..}/>};
    let fragment: Fragment = (children_value.get_value())().into();
    let nodes_count = fragment.nodes.iter().count();

    if slottable
        .as_ref()
        .is_some_and(|st| Fragment::from((st.children)()).nodes.iter().count() != 1)
    {
        return ().into_any();
    }

    if let Some(slottable) = &slottable {
        // TODO: Slot.children become slottable's children' children
        let slottable_children_value = StoredValue::new(slottable.children.clone());
        let slottable_fragment: Fragment = (slottable_children_value.get_value())().into();

        // nodes = nodes
        //     .into_iter()
        //     .chain((new_element_value.get_value())().nodes)
        //     .collect::<Vec<AnyView>>()
        //     .into();
        // return view! {
        //     <SlotClone {..attrs}>
        //         {Box::new(move || {
        //             slottable_fragment
        //                 .nodes
        //                 .into_iter()
        //                 .next()
        //                 .unwrap_or(().into_any())
        //                 // .children(view! {<p/>})
        //                 // .chain((new_element_value.get_value())().nodes).collect_view()
        //         })}
        //         // {children().nodes.into_iter().chain((slottable.children)().nodes.into_iter()).collect_view()}
        //     </SlotClone>
        // }
        // .into_any();

        // return p()
        //     .child(new_element().nodes.into_iter().next().unwrap())
        //     .into_any();
    }

    Effect::new(move || {
        leptos::logging::log!(
            "Slot<El>.owner_name={:?}, nodes: {}, slottable: {}, slottable nodes: {:?}",
            &owner_name,
            nodes_count,
            slottable.is_some(),
            slottable
                .clone()
                .map(|s| Fragment::from((s.children)()).nodes.iter().count()),
        );
    });

    return view! {
        <SlotClone {..attrs}>
            {Box::new(move || {
                Fragment::from((children_value.get_value())())
                    .nodes
                    .into_iter()
                    .next()
                    .unwrap_or(().into_any())
                    // .children(view! {<p/>})
                    // .chain((new_element_value.get_value())().nodes).collect_view()
            })}
            // {children().nodes.into_iter().chain((slottable.children)().nodes.into_iter()).collect_view()}
        </SlotClone>
    }
    .into_any();
}

#[component]
fn SlotClone(children: Children) -> impl IntoView {
    children()
}

pub struct SlottableReturn<E> {
    pub display_name: String,
    __radix_id: &'static str,
    pub element: fn() -> HtmlElement<E, (), ()>,
}

#[component(transparent)]
pub fn Slottable2<E, C>(
    owner_name: &'static str,
    element: fn() -> HtmlElement<E, (), ()>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> SlottableReturn<E>
where
    E: ElementType + 'static,
    C: IntoView + 'static,
    View<C>: RenderHtml,
    HtmlElement<E, (), ()>: ElementChild<View<C>>,
    <HtmlElement<E, (), ()> as ElementChild<View<C>>>::Output: IntoView,
    <E as ElementType>::Output: JsCast,
    AnyNodeRef: NodeRefContainer<E>,
{
    SlottableReturn {
        display_name: format!("{owner_name}.Slottable"),
        __radix_id: "radix.slottable",
        element,
    }
}
