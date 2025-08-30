use dioxus::prelude::*;

use crate::components::log_row::log_row;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "hello" }
        log_row {}
    }
}
