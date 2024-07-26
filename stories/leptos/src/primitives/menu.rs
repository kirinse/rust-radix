use leptos::prelude::*;
use radix_leptos_menu::*;
use tailwind_fuse::*;

#[component]
pub fn Styled() -> impl IntoView {
    let item_class: Memo<String> = Memo::new(move |_| ItemClass::default().to_class());
    let separator_class = Memo::new(move |_| SeparatorClass::default().to_class());

    view! {
        <MenuWithAnchor>
            <MenuItem attr:class=item_class>
                Undo
            </MenuItem>
            <MenuItem attr:class=item_class>
                Redo
            </MenuItem>
            <MenuSeparator attr:class=separator_class />
            <MenuItem attr:class=item_class>
                Cut
            </MenuItem>
            <MenuItem attr:class=item_class>
                Copy
            </MenuItem>
            <MenuItem attr:class=item_class>
                Paste
            </MenuItem>
        </MenuWithAnchor>
    }
}

#[component]
pub fn Submenus() -> impl IntoView {
    view! {}
}

#[component]
pub fn WithLabels() -> impl IntoView {
    view! {}
}

#[component]
pub fn Typeahead() -> impl IntoView {
    view! {}
}

#[component]
pub fn CheckboxItems() -> impl IntoView {
    view! {}
}

#[component]
pub fn RadioItems() -> impl IntoView {
    view! {}
}

#[component]
pub fn Animated() -> impl IntoView {
    view! {}
}

#[component]
fn MenuWithAnchor(
    #[prop(into, optional)] open: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let open = Signal::derive(move || open.get().unwrap_or(true));

    let content_class = Memo::new(move |_| ContentClass::default().to_class());

    // TODO: add missing props
    view! {
        <Menu open=open modal=false>
            <MenuAnchor>{""}</MenuAnchor>
            <MenuPortal>
                <MenuContent attr:class=content_class>
                    {children.with_value(|children| children())}
                </MenuContent>
            </MenuPortal>
        </Menu>
    }
}

#[component]
fn Submenu() -> impl IntoView {
    view! {}
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "inline-block box-border min-w-[130px] bg-[#fff] border border-solid border-[#ccc] rounded-[6px] p-[5px] shadow-[0px_5px_10px_0px_rgba(0,0,0,0.1)] font-['apple-system, BlinkMacSystemFont, helvetica, arial, sans-serif'] text-[13px] focus-within:border-[#111]"
)]
pub struct ContentClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "flex items-center justify-between leading-none cursor-default select-none whitespace-nowrap h-[25px] p-[0px_10px] text-[#ccc] rounded-[3px]"
)]
pub struct LabelClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "flex items-center justify-between leading-none cursor-default select-none whitespace-nowrap h-[25px] p-[0px_10px] text-[#111] rounded-[3px] outline-none data-highlighted:bg-[#111] data-highlighted:text-[#fff] data-disabled:text-[#ccc]"
)]
pub struct ItemClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "flex items-center justify-between leading-none cursor-default select-none whitespace-nowrap h-[25px] p-[0px_10px] text-[#111] rounded-[3px] outline-none data-highlighted:bg-[#111] data-highlighted:text-[#fff] data-disabled:text-[#ccc] [&:not([data-highlighted])[data-state=\"open\"]]:bg-[#ccc] [&:not([data-highlighted])[data-state=\"open\"]]:text-[#111]"
)]
pub struct SubTriggerClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "h-[1px] m-[5px_10px] bg-[#ccc]")]
pub struct SeparatorClass {}
