use std::fmt::Debug;

use leptos::prelude::*;
use leptos_maybe_callback::MaybeCallback;

pub struct UseControllableStateParams<T: Send + Sync + 'static> {
    pub prop: MaybeProp<T>,
    pub default_prop: T,
    pub on_change: MaybeCallback<T>,
}

#[track_caller]
pub fn use_controllable_state<T: Clone + PartialEq + Send + Sync + Debug + Default>(
    UseControllableStateParams {
        prop,
        default_prop,
        on_change,
    }: UseControllableStateParams<T>,
) -> (Signal<T>, Callback<T>) {
    let (uncontrolled_prop, set_uncontrolled_prop) =
        use_uncontrolled_state(UseUncontrollableStateParams {
            default_prop,
            on_change: on_change.clone(),
        });
    let is_controlled = prop.get_untracked().is_some();

    let (prop_value, set_prop_value) = signal(prop.get_untracked().unwrap_or_default());

    let value = Signal::derive(move || match is_controlled {
        true => prop_value.get(),
        false => uncontrolled_prop.get(),
    });

    let set_value = Callback::new(move |next_value: T| {
        #[cfg(debug_assertions)]
        leptos::logging::log!(
            "is_controlled: {:?}, next_value: {:?}, value: {:?}",
            is_controlled,
            &next_value,
            value.get(),
        );
        if is_controlled {
            if next_value != value.get() {
                // if let Some(on_change) = on_change.as_ref() {
                on_change.as_callback().run(next_value.clone());
                set_prop_value.set(next_value);
                // }
            }
        } else {
            set_uncontrolled_prop.set(next_value);
        }
    });

    (value, set_value)
}

pub struct UseUncontrollableStateParams<T: Send + Sync + 'static> {
    pub default_prop: T,
    pub on_change: MaybeCallback<T>,
}

fn use_uncontrolled_state<T: Clone + Default + Debug + PartialEq + Send + Sync>(
    UseUncontrollableStateParams {
        default_prop,
        on_change,
    }: UseUncontrollableStateParams<T>,
) -> (ReadSignal<T>, WriteSignal<T>) {
    let (value, set_value) = signal(default_prop);
    let prev_value = StoredValue::new(value.get_untracked());

    Effect::new(move |_| {
        if prev_value.get_value() != value.get() {
            if let Some(on_change) = on_change.as_ref() {
                on_change.run(value.get());
            }
            set_value.set(value.get());
            prev_value.set_value(value.get());
        }
    });

    (value, set_value)
}
