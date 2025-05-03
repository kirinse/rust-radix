use std::fmt::Debug;

use leptos::prelude::*;

pub struct UseControllableStateParams<T: Send + Sync + 'static> {
    pub prop: MaybeProp<T>,
    pub default_prop: MaybeProp<T>,
    pub on_change: Option<Callback<Option<T>>>,
}

#[track_caller]
pub fn use_controllable_state<T: Clone + PartialEq + Send + Sync + Debug + Default>(
    UseControllableStateParams {
        prop,
        default_prop,
        on_change,
    }: UseControllableStateParams<T>,
) -> (Signal<Option<T>>, Callback<Option<T>>) {
    let is_controlled = Signal::derive(move || prop.get().is_some());
    let prop_value = prop.get();
    let (prop_value, set_prop_value) = signal(prop_value);

    let (uncontrolled_prop, set_uncontrolled_prop) =
        use_uncontrolled_state(UseUncontrollableStateParams {
            default_prop,
            on_change,
        });
    let value = Signal::derive(move || match is_controlled.get() {
        true => prop_value.get(),
        false => uncontrolled_prop.get(),
    });

    let set_value = Callback::new(move |next_value: Option<T>| {
        // logging::log!(
        //     "is_controlled: {:?}, next_value: {:?}, prop: {:?}",
        //     is_controlled.get(),
        //     &next_value,
        //     prop_value.get(),
        // );
        if next_value != value.get() {
            if is_controlled.get() {
                if let Some(on_change) = on_change {
                    on_change.run(next_value.clone());
                }
                set_prop_value.set(next_value.clone());
            } else {
                set_uncontrolled_prop.set(next_value);
            }
        }
    });

    (value, set_value)
}

pub struct UseUncontrollableStateParams<T: Send + Sync + 'static> {
    pub default_prop: MaybeProp<T>,
    pub on_change: Option<Callback<Option<T>>>,
}

fn use_uncontrolled_state<T: Clone + Default + PartialEq + Send + Sync>(
    UseUncontrollableStateParams {
        default_prop,
        on_change,
    }: UseUncontrollableStateParams<T>,
) -> (ReadSignal<Option<T>>, WriteSignal<Option<T>>) {
    let (value, set_value) = signal(Some(default_prop.get_untracked().unwrap_or_default()));

    Effect::new(move |prev_value: Option<Option<T>>| {
        let value = value.get();
        if (prev_value.is_none() && value.is_some()) || prev_value.is_some_and(|p| p != value) {
            if let Some(on_change) = on_change {
                on_change.run(value.clone());
            } else {
                set_value.set(value.clone());
            }
        }
        value
    });

    (value, set_value)
}
