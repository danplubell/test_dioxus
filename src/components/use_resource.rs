use dioxus::logger::tracing::debug;
use dioxus::prelude::*;

#[component]
fn UseResource() -> Element {
    let mut count = use_signal(|| 0);

    // use_resource creates a tracked value that is derived from count
    // Since we read count inside the closure, it becomes a dependency of the resource
    // Whenever count changes, the resource will rerun
    let half_count = use_resource(move || async move {
        // You can do async work inside resources
        gloo_timers::future::TimeoutFuture::new(100).await;
        count() / 2
    });

    use_effect(move || {
        // half_count is itself a tracked value
        // When we read half_count, it becomes a dependency of the effect
        // and the effect will rerun when half_count changes
        debug!("{:?}", half_count());
    });

    rsx! {
        button { onclick: move |_| count += 1, "Change Signal" }

        div { "Count is {count}" }
        div { "Half count is {half_count():?}" }
    }
}
