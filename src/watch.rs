use std::sync::{Arc, Mutex};

use leptos::prelude::*;

use crate::JsStoredValue;

pub fn watch<W, T>(
    deps: impl Fn() -> W + 'static,
    callback: impl Fn(&W, Option<&W>, &Option<T>) -> T + Clone + 'static,
    immediate: bool,
) -> impl Fn() + Clone
where
    W: Clone + 'static,
    T: Clone + 'static,
{
    let last_result = JsStoredValue::new(None::<T>);

    let effect = Effect::new(move |prev| {
        let deps = deps();
        if !immediate {
            return deps;
        }

        let result = callback(&deps, prev.as_ref(), &last_result.get_value());
        last_result.set_value(Some(result));
        deps
    });

    let effect_stop = Arc::new(Mutex::new(Some(effect)));

    move || {
        let mut effect_stop = effect_stop.lock().unwrap();
        if let Some(effect_stop) = effect_stop.take() {
            effect_stop.stop();
        }
    }
}
