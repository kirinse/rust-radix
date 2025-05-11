use leptos::{prelude::*, tachys::view::any_view::AnyViewWithAttrs};
use leptos_node_ref::AnyNodeRef;
// use leptos_dom::{ComponentRepr, Text};

fn remove_nameless_component(child: &View) -> &View {
    match child {
        View::Component(c) if c.name().is_empty() && c.children.len() == 1 => {
            remove_nameless_component(&c.children[0])
        }
        _ => child,
    }
}

#[component]
pub fn Slot(
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(attrs)] attrs: Vec<(&'static str, impl IntoAttributeValue)>,
    children: ChildrenFragment,
) -> impl IntoView {
    let children_fragment = children();
    let slottable = children_fragment
        // .as_children()
        .nodes
        .iter()
        .find(|child| is_slottable(child));

    if let Some(View::Component(slottable)) = slottable {
        if slottable.children.len() == 1 {
            let new_children = children_fragment
                .as_children()
                .iter()
                .flat_map(|child| match child {
                    View::Component(component) if component.name() == "Slottable" => {
                        let children = component
                            .children
                            .iter()
                            .map(remove_nameless_component)
                            .collect::<Vec<_>>();
                        log::info!("children of slottable {:?}", children);

                        if children.len() != 1 {
                            vec![]
                        } else {
                            match children[0] {
                                View::Element(element) => {
                                    // TODO: Leptos does not allow access to element children, so we can't properly take out the children.
                                    //       Currently, this is a hack to at least extract text elements.

                                    let mut views = vec![];

                                    let node_list =
                                        element.clone().into_html_element().child_nodes();
                                    for n in 0..node_list.length() {
                                        if let Some(node) = node_list.item(n) {
                                            log::info!("{:?}", node);

                                            // Node.TEXT_NODE
                                            if node.node_type() == 3 {
                                                if let Some(text) = node.text_content() {
                                                    views.push(View::Text(Text::new(text.into())));
                                                }
                                            }
                                        }
                                    }

                                    views
                                }
                                View::Component(component) => component.children.clone(),
                                _ => vec![],
                            }
                        }
                    }
                    _ => vec![child.clone()],
                })
                .collect::<Vec<_>>();
            log::info!("new children {:?}", new_children);

            let slottable_child = remove_nameless_component(&slottable.children[0]).clone();
            let new_element = StoredValue::new(match slottable_child {
                View::Element(_element) => {
                    log::info!("old is element");

                    // TODO: Actually use the tag name, instead of hardcoding the story example.
                    // element.into_html_element().tag_name();

                    let mut html_element = html::b();
                    for child in new_children {
                        html_element = html_element.child(child);
                    }

                    // TODO: copy attrs from element

                    Some(html_element.into_view())
                }
                View::Component(component) => {
                    log::info!("old is component");

                    let mut repr = ComponentRepr::new(component.name().to_string());
                    repr.children.extend(new_children);

                    log::info!("{:?}", repr);
                    Some(View::Component(repr))
                }
                v => {
                    log::info!("old is other {:?}", v);
                    None
                }
            });

            log::info!("new element {:?}", new_element.get_value());
            return view! {
                <SlotClone node_ref=node_ref attrs=attrs>
                   {new_element.get_value()}
                </SlotClone>
            };
        }

        view! {
            <SlotClone node_ref=node_ref attrs=attrs />
        }
    } else {
        view! {
            <SlotClone node_ref=node_ref attrs=attrs>
                {children_fragment.as_children().collect_view()}
            </SlotClone>
        }
    }
}

#[component]
pub fn SlotClone(
    #[prop(optional)] node_ref: AnyNodeRef,
    // #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] children: Option<ChildrenFragment>,
) -> impl IntoView {
    if let Some(children) = children {
        let children_fragment = children();
        if children_fragment.nodes.iter().count() == 1 {
            return map_children(children_fragment.nodes.into_iter().as_ref(), node_ref);
        }
    }

    return view! {}.into_any();
}

#[component]
pub fn Slottable(children: Children) -> impl IntoView {
    children()
}

fn is_slottable(child: &AnyViewWithAttrs) -> bool {
    child.into_render().name();
    matches!(child, View::Component(c) if c.name() == "Slottable")
}

fn map_children(
    children: &[AnyView],
    node_ref: AnyNodeRef,
    // attrs: Vec<(&'static str, Attribute)>,
) -> AnyView {
    children
        .iter()
        .map(|child| match child {
            View::Element(element) => element
                .clone()
                .into_html_element()
                .node_ref(node_ref)
                .attrs(attrs.clone())
                .into_view(),
            View::Component(component) => {
                map_children(&component.children, node_ref, attrs.clone())
            }
            View::CoreComponent(component) => {
                leptos::logging::log!("core component {:?}", component);

                // TODO: handle core components
                // match component {
                //     CoreComponent::Unit(_) => todo!("slot unit"),
                //     CoreComponent::DynChild(dyn_child) => todo!("slot dyn child"),
                //     CoreComponent::Each(_) => todo!("slot each"),
                // }

                child.into_view()
            }
            _ => child.into_view(),
        })
        .collect_view()
}
