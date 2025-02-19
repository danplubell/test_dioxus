use dioxus::logger::tracing::{debug, Level};
use dioxus::prelude::*;
use log::log;

fn main() {
    dioxus::logger::init(Level::DEBUG).expect("logger failed to init");
    dioxus::launch(App);
}
#[component]
fn Counter(count: Signal<i32>) -> Element {
    rsx! {
        button { onclick: move |_| count +=1 , "Increment" }
    }
}
#[component]
fn App() -> Element {

    let mut count = use_signal(|| 0);
    use_effect(move || {
        // When we read count, it becomes a dependency of the effect
        let current_count = count();
        // Whenever count changes, the effect will rerun
        debug!("{current_count}");
    });

    rsx! {
        Counter{ count }
//        button { onclick: move |_| count += 1, "Increment" }

        div { "Count is {count}" }
    }
}

