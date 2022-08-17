use yew::{html, use_reducer, use_ref, use_state, Component, Context, Html, Properties};

// reducer's Action

pub struct FormConfig {
    initial_status: Option<String>,
}

pub fn use_form<Values>() -> FormConfig {
    let initial_values = use_ref(|| "");
    let initial_errors = use_ref(|| "");
    let initial_status = use_ref(|| "");
    let is_mounted = use_ref(|| false);

    FormConfig {
        initial_status: None,
    }
}
