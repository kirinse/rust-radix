use leptos::prelude::{Get, Memo, RwSignal, Set, Signal};

pub fn use_previous<T: Clone + PartialEq + Send + Sync + 'static>(value: Signal<T>) -> Memo<T> {
    let current = RwSignal::new(value.get());
    let previous = RwSignal::new(value.get());

    Memo::new(move |_| {
        let value = value.get();
        let current_value = current.get();
        if current_value != value {
            previous.set(current_value);
            current.set(value.clone());
        }
        previous.get()
    })
}
