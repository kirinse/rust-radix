use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct BorderBottomIconProps {
    #[prop_or(15)]
    pub width: usize,
    #[prop_or(15)]
    pub height: usize,
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub node_ref: NodeRef,
}
#[function_component]
pub fn BorderBottomIcon(props: &BorderBottomIconProps) -> Html {
    html! {
        <svg
            ref={props.node_ref.clone()}
            class={props.class.clone()}
            width={props.width.to_string()}
            height={props.height.to_string()}
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M1 13.25L14 13.25V14.75L1 14.75V13.25Z"
                fill={& props.color}
            />
            <rect x="7" y="5" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="13" y="5" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="7" y="3" width="1" height="1" rx=".5" fill={& props.color} />
            <rect
                x="13"
                y="3"
                width="1"
                height="1"
                rx=".5"
                fill={& props
        .color}
            />
            <rect
                x="7"
                y="7"
                width="1"
                height="1"
                rx=".5"
                fill={&
        props.color}
            />
            <rect x="7" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="13" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="13" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="5" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="5" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="3" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="3" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="9" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="9" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="11" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="11" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect
                x="7"
                y="9"
                width="1"
                height="1"
                rx=".5"
                fill={& props
        .color}
            />
            <rect
                x="13"
                y="9"
                width="1"
                height="1"
                rx=".5"
                fill={&
        props.color}
            />
            <rect x="7" y="11" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="13" y="11" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="5" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="3" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="9" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="11" width="1" height="1" rx=".5" fill={& props.color} />
        </svg>
    }
}
