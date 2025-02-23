use dioxus::logger::tracing::debug;
use dioxus::prelude::*;

#[component]
pub fn Effect() -> Element {
    // use_signal creates a tracked value called count
    let mut count = use_signal(|| 0);

    use_effect(move || {
        // When we read count, it becomes a dependency of the effect
        let current_count = count();
        // Whenever count changes, the effect will rerun
        debug!("{current_count}");
    });

    rsx! {
        button { onclick: move |_| count += 1, "Increment" }

        div { "Count is {count}" }
    }
}