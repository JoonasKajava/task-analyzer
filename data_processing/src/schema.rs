// @generated automatically by Diesel CLI.

diesel::table! {
    task_sources (id) {
        id -> Nullable<Integer>,
        source_name -> Nullable<Text>,
        api_key -> Text,
        url -> Text,
    }
}
