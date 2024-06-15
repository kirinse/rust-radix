use leptos::*;
use radix_leptos_label::*;
use tailwind_fuse::*;

// TODO: add router and separate pages for each component, similar to Storybook

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h1 class="text-xl pb-3">Styled</h1>
        <Styled />

        <h1 class="text-xl pb-3">With Control</h1>
        <WithControl />

        <h1 class="text-xl pb-3">With Input Number</h1>
        <WithInputNumber />
    }
}

#[component]
fn Styled() -> impl IntoView {
    let root_class = create_memo(move |_| RootClass::default().to_class());

    view! {
        <Label attr:class=root_class>Label</Label>
    }
}

#[component]
fn WithControl() -> impl IntoView {
    let control_class = create_memo(move |_| ControlClass::default().to_class());

    view! {
        <h1>Wrapping control</h1>
        <Label>
            <Control attr:class=control_class /> Label
        </Label>

        <h1>Referencing control</h1>
        <Control attr:id="control" attr:class=control_class />
        <Label attr:for="control">Label</Label>
    }
}

#[component]
fn WithInputNumber() -> impl IntoView {
    view! {
        <Label>
            <span>Name:</span>
            <input type="number" />
        </Label>
    }
}

#[component]
fn Control(#[prop(attrs)] attributes: Vec<(&'static str, Attribute)>) -> impl IntoView {
    view! {
        <button {..attributes} on:click=move |_| window().alert_with_message("clicked").expect("Alert should be successful.")>
            Control
        </button>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "inline-block align-middle cursor-default border-[1px] border-solid border-[gainsboro] p-[10px]"
)]
pub struct RootClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "inline-flex border-[1px] border-solid border-[gainsboro] p-[10px] align-middle m-[0px_10px] hover:bg-[red]"
)]
pub struct ControlClass {}
