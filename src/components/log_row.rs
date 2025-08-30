use dioxus::prelude::*;

#[component]
pub fn log_row() -> Element {
    rsx! {
        table {
            tr {
                td { "Key / Task" }
                td { "Sub key" }
                td { "Start time" }
                td { "End time" }
            }
            tr {
                td {
                    input { r#type: "text" }
                }
                td {
                    input { r#type: "text" }
                }
                td {
                    input { r#type: "text" }
                }
                td {
                    input { r#type: "text" }
                }
            }
        }
    }
}
