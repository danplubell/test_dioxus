use dioxus::logger::tracing::debug;
use dioxus::prelude::*;

#[component]
pub fn Memo() -> Element {
    let mut count = use_signal(|| 0);

    // use_memo creates a tracked value that is derived from count
    // Since we read count inside the closure, it becomes a dependency of the memo
    // Whenever count changes, the memo will rerun
    let half_count = use_memo(move || count() / 2);

    use_effect(move || {
        // half_count is itself a tracked value
        // When we read half_count, it becomes a dependency of the effect
        // and the effect will rerun when half_count changes
        debug!("{half_count}");
    });

    rsx! {
        button { onclick: move |_| count += 1, "Increment" }

        div { "Count is {count}" }
        div { "Half count is {half_count}" }
    }
}
