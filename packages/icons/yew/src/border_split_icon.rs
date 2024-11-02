use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct BorderSplitIconProps {
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub width: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub height: AttrValue,
}
#[function_component]
pub fn BorderSplitIcon(props: &BorderSplitIconProps) -> Html {
    let node_ref = use_node_ref();
    html! {
        <svg
            ref={node_ref}
            class={&props.class}
            width={&props.width}
            height={&props.height}
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <rect x="7" y="5.025" width="1" height="1" rx=".5" fill={&props.color} />
            <rect x="7" y="3.025" width="1" height="1" rx=".5" fill={&props.color} />
            <rect x="7" y="7.025" width="1" height="1" rx=".5" fill={&props.color} />
            <rect x="7" y="13.025" width="1" height="1" rx=".5" fill={&props.color} />
            <rect x="7" y="1.025" width="1" height="1" rx=".5" fill={&props.color} />
            <rect
                x="13"
                y="7.025"
                width="1"
                height="1"
                rx=".5"
                fill={&props
        .color}
            />
            <rect x="5" y="7.025" width="1" height="1" rx=".5" fill={&props.color} />
            <rect x="3" y="7.025" width="1" height="1" rx=".5" fill={&props.color} />
            <rect x="9" y="7.025" width="1" height="1" rx=".5" fill={&props.color} />
            <rect x="11" y="7.025" width="1" height="1" rx=".5" fill={&props.color} />
            <rect x="7" y="9.025" width="1" height="1" rx=".5" fill={&props.color} />
            <rect x="7" y="11.025" width="1" height="1" rx=".5" fill={&props.color} />
            <rect x="1" y="7.025" width="1" height="1" rx=".5" fill={&props.color} />
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M1 1.49994C1 1.2238 1.22386 0.999939 1.5 0.999939H6V1.99994H2V5.99994H1V1.49994ZM13 1.99994H9V0.999939H13.5C13.7761 0.999939 14 1.2238 14 1.49994V5.99994H13V1.99994ZM1 13.4999V8.99994H2V12.9999H6V13.9999H1.5C1.22386 13.9999 1 13.7761 1 13.4999ZM13 12.9999V8.99994H14V13.4999C14 13.7761 13.7761 13.9999 13.5 13.9999H9.5V12.9999H13Z"
                fill={&props.color}
            />
        </svg>
    }
}
