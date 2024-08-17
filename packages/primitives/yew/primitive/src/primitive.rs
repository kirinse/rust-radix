use yew::{prelude::*, virtual_dom::VTag};
use yew_attrs::Attrs;

#[derive(PartialEq, Properties)]
pub struct PrimitiveProps {
    pub element: String,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Primitive(props: &PrimitiveProps) -> Html {
    let tag = VTag::__new_other(
        props.element.clone().into(),
        props.node_ref.clone(),
        Default::default(),
        props.attrs.attributes.clone(),
        props.attrs.listeners.clone(),
        props.children.clone(),
    );

    tag.into()
}
