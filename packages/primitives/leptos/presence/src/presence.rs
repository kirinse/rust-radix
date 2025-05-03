use std::{collections::HashMap, sync::Arc};

use leptos::{ev::AnimationEvent, html, prelude::*};
use leptos_node_ref::{AnyNodeRef, any_node_ref};
use radix_leptos_compose_refs::use_composed_refs;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

use crate::use_state_machine::use_state_machine;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum MachineState {
    Mounted,
    UnmountSuspended,
    Unmounted,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum MachineEvent {
    Mount,
    AnimationOut,
    AnimationEnd,
    Unmount,
}

#[component]
pub fn Presence(
    #[prop(into)] present: Signal<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let presence = use_presence(present);
    let composed_ref = use_composed_refs(vec![presence.r#ref, node_ref]);

    // children.with_value(|children| assert_eq!(children().as_children().len(), 1));

    view! {
        <Show when=move || presence.is_present.get()>
            {children.with_value(|children| children()).add_any_attr(any_node_ref::<html::Div, _>(composed_ref))}
            // {map_children(children.with_value(|children| children()), composed_ref)}
        </Show>
    }
}

// TODO:
// fn map_children(children: &[View], node_ref: AnyNodeRef) -> impl IntoView {
//     children
//         .iter()
//         .map(|child| match child {
//             View::Element(element) => element
//                 .clone()
//                 .into_html_element()
//                 .node_ref(node_ref)
//                 .into_view(),
//             View::Component(component) => map_children(&component.children, node_ref),
//             _ => child.into_view(),
//         })
//         .collect_view()
// }

struct UsePresenceReturn {
    is_present: Signal<bool>,
    r#ref: AnyNodeRef,
}

fn use_presence(present: Signal<bool>) -> UsePresenceReturn {
    let node_ref = AnyNodeRef::new();
    let styles = RwSignal::new_local(None);

    let prev_present = RwSignal::new(present.get_untracked());
    let prev_animation_name = RwSignal::new("none".to_string());
    let initial_state = match present.get_untracked() {
        true => MachineState::Mounted,
        false => MachineState::Unmounted,
    };
    let (state, send) = use_state_machine(
        initial_state,
        HashMap::from([
            (
                MachineState::Mounted,
                HashMap::from([
                    (MachineEvent::Unmount, MachineState::Unmounted),
                    (MachineEvent::AnimationOut, MachineState::UnmountSuspended),
                ]),
            ),
            (
                MachineState::UnmountSuspended,
                HashMap::from([
                    (MachineEvent::Mount, MachineState::Mounted),
                    (MachineEvent::AnimationEnd, MachineState::Unmounted),
                ]),
            ),
            (
                MachineState::Unmounted,
                HashMap::from([(MachineEvent::Mount, MachineState::Mounted)]),
            ),
        ]),
    );

    Effect::new(move |_| {
        let current_animation_name = styles.with(|s| get_animation_name(s.as_ref()));
        // let current_animation_name = get_animation_name(styles.get().as_ref());
        prev_animation_name.set(match state.get() {
            MachineState::Mounted => current_animation_name,
            _ => "none".into(),
        });
    });

    Effect::new(move |_| {
        let styles = styles.get();
        let was_present = prev_present.get();
        let is_present = present.get();
        let has_present_changed = was_present != is_present;

        if has_present_changed {
            let prev_animation_name = prev_animation_name.get();
            let current_animation_name = get_animation_name(styles.as_ref());

            if is_present {
                send.run(MachineEvent::Mount);
            } else if current_animation_name == "none"
                || styles
                    .as_ref()
                    .and_then(|styles| styles.get_property_value("display").ok())
                    == Some("none".into())
            {
                // If there is no exit animation or the element is hidden, animations won't run so we unmount instantly.
                send.run(MachineEvent::Unmount);
            } else {
                // When `present` changes to `false`, we check changes to animation-name to
                // determine whether an animation has started. We chose this approach (reading
                // computed styles) because there is no `animationrun` event and `animationstart`
                // fires after `animation-delay` has expired which would be too late.
                let is_animating = prev_animation_name != current_animation_name;

                if was_present && is_animating {
                    send.run(MachineEvent::AnimationOut);
                } else {
                    send.run(MachineEvent::Unmount);
                }
            }

            prev_present.set(is_present);
        }
    });

    // Triggering an ANIMATION_OUT during an ANIMATION_IN will fire an `animationcancel`
    // event for ANIMATION_IN after we have entered `unmountSuspended` state. So, we
    // make sure we only trigger ANIMATION_END for the currently active animation.
    #[allow(clippy::type_complexity)]
    let handle_animation_end: Arc<SendWrapper<Closure<dyn Fn(AnimationEvent)>>> = Arc::new(
        SendWrapper::new(Closure::new(move |event: AnimationEvent| {
            let current_animation_name = get_animation_name(styles.get_untracked().as_ref());
            let is_current_animation = current_animation_name.contains(&event.animation_name());
            if is_current_animation
                && event.target().as_ref()
                    == node_ref
                        .get_untracked()
                        .as_ref()
                        .map(|node| node.unchecked_ref::<web_sys::EventTarget>())
            {
                send.run(MachineEvent::AnimationEnd);
            }
        })),
    );
    let cleanup_handle_animation_end = handle_animation_end.clone();

    #[allow(clippy::type_complexity)]
    let handle_animation_start: Arc<SendWrapper<Closure<dyn Fn(AnimationEvent)>>> = Arc::new(
        SendWrapper::new(Closure::new(move |event: AnimationEvent| {
            if event.target().as_ref()
                == node_ref
                    .get_untracked()
                    .as_ref()
                    .map(|node| node.unchecked_ref::<web_sys::EventTarget>())
            {
                // If animation occurred, store its name as the previous animation.
                prev_animation_name.set(get_animation_name(styles.get_untracked().as_ref()));
            }
        })),
    );
    let cleanup_handle_animation_start = handle_animation_start.clone();

    Effect::new(move |_| {
        if let Some(node) = node_ref.get() {
            node.add_event_listener_with_callback(
                "animationstart",
                (*handle_animation_start).as_ref().unchecked_ref(),
            )
            .expect("Animation start event listener should be added.");

            node.add_event_listener_with_callback(
                "animationcancel",
                (*handle_animation_end).as_ref().unchecked_ref(),
            )
            .expect("Animation cancel event listener should be added.");

            node.add_event_listener_with_callback(
                "animationend",
                (*handle_animation_end).as_ref().unchecked_ref(),
            )
            .expect("Animation end event listener should be added.");
        } else {
            // Transition to the unmounted state if the node is removed prematurely.
            // We avoid doing so during cleanup as the node may change but still exist.
            send.run(MachineEvent::AnimationEnd);
        }
    });

    Effect::new(move |_| {
        if let Some(node) = node_ref.get() {
            styles.set(
                window()
                    .get_computed_style(&node)
                    .expect("Element is valid."),
            );
        }
    });

    on_cleanup(move || {
        if let Some(node) = node_ref.get() {
            node.remove_event_listener_with_callback(
                "animationstart",
                (*cleanup_handle_animation_start).as_ref().unchecked_ref(),
            )
            .expect("Animation start event listener should be removed.");
            node.remove_event_listener_with_callback(
                "animationcancel",
                (*cleanup_handle_animation_end).as_ref().unchecked_ref(),
            )
            .expect("Animation cancel event listener should be removed.");
            node.remove_event_listener_with_callback(
                "animationend",
                (*cleanup_handle_animation_end).as_ref().unchecked_ref(),
            )
            .expect("Animation end event listener should be removed.");
        }
    });

    UsePresenceReturn {
        is_present: Signal::derive(move || {
            [MachineState::Mounted, MachineState::UnmountSuspended].contains(&state.get())
        }),
        r#ref: node_ref,
    }
}

fn get_animation_name(styles: Option<&web_sys::CssStyleDeclaration>) -> String {
    styles
        .and_then(|styles| styles.get_property_value("animation-name").ok())
        .unwrap_or("none".into())
}
