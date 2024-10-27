use leptos::{html::Div, *};
use leptos_dom::Unit;
use radix_leptos_direction::{Direction, DirectionProvider};

use crate::{
    components::theme_props::{Appearance, PanelBackground, Scaling},
    props::{
        color_prop::{AccentColor, GrayColor},
        radius_prop::Radius,
    },
};

#[derive(Clone)]
struct ThemeContextValue {
    appearance: Appearance,
    accent_color: AccentColor,
    gray_color: GrayColor,
    resolved_gray_color: GrayColor,
    panel_background: PanelBackground,
    radius: Radius,
    scaling: Scaling,
    on_appearance_change: Callback<Appearance>,
    on_accent_color_change: Callback<AccentColor>,
    on_gray_color_change: Callback<GrayColor>,
    on_panel_background_change: Callback<PanelBackground>,
    on_radius_change: Callback<Radius>,
    on_scaling_change: Callback<Scaling>,
}

pub fn use_theme_context() -> ThemeContextValue {
    // use_context::<ThemeContextValue>().expect("`use_theme_context` must be used within a `Theme`")
    todo!()
}

#[component]
// pub fn Theme<AsChild, AsChildView>(
pub fn Theme(
    #[prop(into, optional)] has_background: MaybeProp<bool>,
    #[prop(into, optional)] appearance: MaybeProp<Appearance>,
    #[prop(into, optional)] accent_color: MaybeProp<AccentColor>,
    #[prop(into, optional)] gray_color: MaybeProp<GrayColor>,
    #[prop(into, optional)] panel_background: MaybeProp<PanelBackground>,
    #[prop(into, optional)] radius: MaybeProp<Radius>,
    #[prop(into, optional)] scaling: MaybeProp<Scaling>,

    #[prop(optional)] node_ref: NodeRef<Div>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    // TODO: style struct
    #[prop(into, optional)] style: MaybeProp<String>,

    #[prop(optional)] as_child: Option<AsChildTest<ThemeChildProps>>,
    // #[prop(optional)] as_child: Option<AsChild>,
    children: ChildrenFn,
) -> impl IntoView
// where
//     AsChild: Fn(ThemeChildProps) -> AsChildView,
//     AsChildView: IntoView,
{
    let context = use_context::<ThemeContextValue>();
    let is_root = || context.is_none();

    let children = StoredValue::new(children);

    view! {
        <Show
            when=|| is_root()
            fallback=|| view! {
                <ThemeImpl
                    has_background=has_background
                    appearance=appearance
                    accent_color=accent_color
                    gray_color=gray_color
                    panel_background=panel_background
                    radius=radius
                    scaling=scaling

                    node_ref=node_ref
                    id=id.clone()
                    class=class.clone()
                    style=style.clone()
                    // as_child=as_child
                >
                    {children.with_value(|children| children())}
                </ThemeImpl>
            }

        >
            // TODO: TooltipProvider
            <DirectionProvider direction={Direction::Ltr}>
                <ThemeRoot
                    has_background=has_background
                    appearance=appearance
                    accent_color=accent_color
                    gray_color=gray_color
                    panel_background=panel_background
                    radius=radius
                    scaling=scaling

                    node_ref=node_ref
                    // id=id.clone()
                    // class=class.clone()
                    // style=style.clone()
                    as_child=as_child
                >
                    {children.with_value(|children| children())}
                </ThemeRoot>
            </DirectionProvider>
        </Show>
    }
}

struct AsChildTest<P, IV = Unit>(Box<dyn Fn(P) -> IV>);

#[component]
pub fn ThemeRoot(
    #[prop(into, optional)] has_background: MaybeProp<bool>,
    #[prop(into, optional)] appearance: MaybeProp<Appearance>,
    #[prop(into, optional)] accent_color: MaybeProp<AccentColor>,
    #[prop(into, optional)] gray_color: MaybeProp<GrayColor>,
    #[prop(into, optional)] panel_background: MaybeProp<PanelBackground>,
    #[prop(into, optional)] radius: MaybeProp<Radius>,
    #[prop(into, optional)] scaling: MaybeProp<Scaling>,

    #[prop(optional)] node_ref: NodeRef<Div>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    // TODO: style struct
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] as_child: Option<AsChildTest<ThemeChildProps>>,
    children: Children,
) -> impl IntoView
// where
//     AsChild: Fn(ThemeChildProps) -> AsChildView,
//     AsChildView: IntoView,
{
    view! {
        // TODO
    }
}

#[derive(Clone)]
pub struct ThemeChildProps {
    pub node_ref: NodeRef<Div>,
    pub id: Option<String>,
    pub class: String,
    // TODO: style struct
    pub style: String,
    pub data_is_root_theme: String,
    pub data_accent_color: String,
    pub data_gray_color: String,
    pub data_has_background: String,
    pub data_panel_background: String,
    pub data_radius: String,
    pub data_scaling: String,
}

#[component]
pub fn ThemeImpl<AsChild, AsChildView>(
    #[prop(default = false.into(), into)] is_root: MaybeSignal<bool>,
    #[prop(into, optional)] has_background: MaybeProp<bool>,
    #[prop(into, optional)] appearance: MaybeProp<Appearance>,
    #[prop(into, optional)] accent_color: MaybeProp<AccentColor>,
    #[prop(into, optional)] gray_color: MaybeProp<GrayColor>,
    #[prop(into, optional)] panel_background: MaybeProp<PanelBackground>,
    #[prop(into, optional)] radius: MaybeProp<Radius>,
    #[prop(into, optional)] scaling: MaybeProp<Scaling>,
    #[prop(default = (|_| ()).into(), into)] on_appearance_change: Callback<Appearance>,
    #[prop(default = (|_| ()).into(), into)] on_accent_color_change: Callback<Appearance>,
    #[prop(default = (|_| ()).into(), into)] on_gray_color_change: Callback<Appearance>,
    #[prop(default = (|_| ()).into(), into)] on_panel_background_change: Callback<Appearance>,
    #[prop(default = (|_| ()).into(), into)] on_radius_change: Callback<Appearance>,
    #[prop(default = (|_| ()).into(), into)] on_scaling_change: Callback<Appearance>,

    #[prop(optional)] node_ref: NodeRef<Div>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    // TODO: style struct
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] as_child: Option<AsChild>,
    children: Children,
) -> impl IntoView
where
    AsChild: Fn(ThemeChildProps) -> AsChildView,
    AsChildView: IntoView,
{
    view! {
        // TODO
    }
}
