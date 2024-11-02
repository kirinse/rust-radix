use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct ComponentPlaceholderIconProps {
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
pub fn ComponentPlaceholderIcon(props: &ComponentPlaceholderIconProps) -> Html {
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
                d="M12.8034 7.14644C12.9986 6.95118 13.3152 6.95118 13.5105 7.14644C13.7057 7.3417 13.7057 7.65829 13.5105 7.85355C13.3152 8.04881 12.9986 8.04881 12.8034 7.85355C12.6081 7.65828 12.6081 7.3417 12.8034 7.14644ZM12.0962 8.56065C12.2915 8.75592 12.2915 9.0725 12.0962 9.26776C11.901 9.46302 11.5844 9.46302 11.3891 9.26776C11.1939 9.0725 11.1939 8.75592 11.3891 8.56065C11.5844 8.36539 11.901 8.36539 12.0962 8.56065ZM10.682 9.97487C10.8773 10.1701 10.8773 10.4867 10.682 10.682C10.4868 10.8772 10.1702 10.8772 9.97493 10.682C9.77967 10.4867 9.77967 10.1701 9.97493 9.97487C10.1702 9.7796 10.4868 9.7796 10.682 9.97487ZM9.26782 11.3891C9.46308 11.5843 9.46308 11.9009 9.26782 12.0962C9.07256 12.2915 8.75598 12.2915 8.56071 12.0962C8.36545 11.9009 8.36545 11.5843 8.56072 11.3891C8.75598 11.1938 9.07256 11.1938 9.26782 11.3891ZM7.1465 13.5104C6.95124 13.3151 6.95124 12.9986 7.1465 12.8033C7.17091 12.7789 7.19721 12.7575 7.22494 12.7392C7.41901 12.6111 7.68275 12.6324 7.85361 12.8033C8.04887 12.9986 8.04887 13.3151 7.85361 13.5104C7.65835 13.7057 7.34176 13.7057 7.1465 13.5104Z"
                fill={&props.color}
            />
            <path
                d="M2.90386 8.56065C2.7086 8.75592 2.7086 9.0725 2.90386 9.26776 3.09912 9.46302 3.41571 9.46302 3.61097 9.26776 3.80623 9.0725 3.80623 8.75592 3.61097 8.56065 3.41571 8.36539 3.09912 8.36539 2.90386 8.56065zM4.31807 9.97487C4.12281 10.1701 4.12281 10.4867 4.31807 10.682 4.51334 10.8772 4.82992 10.8772 5.02518 10.682 5.22044 10.4867 5.22044 10.1701 5.02518 9.97487 4.82992 9.7796 4.51334 9.7796 4.31807 9.97487zM5.73229 11.3891C5.53703 11.5843 5.53703 11.9009 5.73229 12.0962 5.92755 12.2914 6.24413 12.2915 6.43939 12.0962 6.63466 11.9009 6.63466 11.5843 6.43939 11.3891 6.24413 11.1938 5.92755 11.1938 5.73229 11.3891zM2.19675 7.85355C2.36761 7.68269 2.38897 7.41895 2.26082 7.22488 2.09046 6.96684 1.71063 6.92546 1.48965 7.14644 1.29439 7.3417 1.29439 7.65829 1.48965 7.85355 1.68491 8.04881 2.00149 8.04881 2.19675 7.85355zM3.61097 5.73223C3.41571 5.53696 3.09912 5.53696 2.90386 5.73223 2.7086 5.92749 2.7086 6.24407 2.90386 6.43933 3.09912 6.6346 3.41571 6.6346 3.61097 6.43933 3.80623 6.24407 3.80623 5.92749 3.61097 5.73223zM5.02518 4.31801C4.82992 4.12275 4.51334 4.12275 4.31807 4.31801 4.12281 4.51328 4.12281 4.82986 4.31807 5.02512 4.51334 5.22038 4.82992 5.22038 5.02518 5.02512 5.22044 4.82986 5.22044 4.51328 5.02518 4.31801zM6.43939 2.9038C6.24413 2.70854 5.92755 2.70854 5.73229 2.9038 5.53703 3.09906 5.53703 3.41564 5.73229 3.61091 5.92755 3.80617 6.24413 3.80617 6.43939 3.61091 6.63466 3.41564 6.63466 3.09906 6.43939 2.9038zM7.85361 1.48959C7.65835 1.29432 7.34176 1.29432 7.1465 1.48959 6.95124 1.68485 6.95124 2.00143 7.1465 2.19669 7.34176 2.39196 7.65835 2.39196 7.85361 2.19669 8.04887 2.00143 8.04887 1.68485 7.85361 1.48959zM9.26782 2.9038C9.07256 2.70854 8.75598 2.70854 8.56071 2.9038 8.36545 3.09906 8.36545 3.41564 8.56071 3.61091 8.75598 3.80617 9.07256 3.80617 9.26782 3.61091 9.46308 3.41564 9.46308 3.09906 9.26782 2.9038zM10.682 4.31801C10.4868 4.12275 10.1702 4.12275 9.97493 4.31801 9.77967 4.51328 9.77967 4.82986 9.97493 5.02512 10.1702 5.22038 10.4868 5.22038 10.682 5.02512 10.8773 4.82986 10.8773 4.51328 10.682 4.31801zM12.0962 5.73223C11.901 5.53696 11.5844 5.53696 11.3891 5.73223 11.1939 5.92749 11.1939 6.24407 11.3891 6.43933 11.5844 6.6346 11.901 6.6346 12.0962 6.43933 12.2915 6.24407 12.2915 5.92749 12.0962 5.73223z"
                fill={&props.color}
            />
        </svg>
    }
}
