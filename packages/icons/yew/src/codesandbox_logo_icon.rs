use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct CodesandboxLogoIconProps {
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
pub fn CodesandboxLogoIcon(props: &CodesandboxLogoIconProps) -> Html {
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
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M7.71144 0.796902C7.57741 0.734357 7.42257 0.734357 7.28855 0.796902L1.28855 3.5969C1.11251 3.67905 0.999993 3.85573 0.999993 4.04999V10.95C0.999993 11.1443 1.11251 11.3209 1.28855 11.4031L7.28855 14.2031C7.42257 14.2656 7.57741 14.2656 7.71144 14.2031L13.7114 11.4031C13.8875 11.3209 14 11.1443 14 10.95V4.04999C14 3.85573 13.8875 3.67905 13.7114 3.5969L7.71144 0.796902ZM7.49999 3.15674L5.98039 2.51091L7.49999 1.80176L9.01959 2.51091L7.49999 3.15674ZM7.69556 4.16018L10.2382 3.07958L12.2719 4.02865L7.49999 6.05671L2.72808 4.02865L4.76181 3.07958L7.30442 4.16018C7.42939 4.2133 7.57059 4.2133 7.69556 4.16018ZM7.99999 6.93078L13 4.80578V7.92966L11.0821 8.8119C10.7273 8.97509 10.5 9.32988 10.5 9.72039V11.7982L7.99999 12.9649V6.93078ZM11.5 11.3316L13 10.6316V9.03039L11.5 9.72039V11.3316ZM6.99999 6.93078V12.9649L4.50231 11.7993V9.72036C4.50231 9.32985 4.27499 8.97506 3.92022 8.81187L1.99999 7.92856V4.80578L6.99999 6.93078ZM1.99999 10.6316L3.50231 11.3326L3.50231 9.72036L1.99999 9.02929V10.6316Z"
                fill={&props.color}
            />
        </svg>
    }
}
