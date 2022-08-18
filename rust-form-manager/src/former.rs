use yew::prelude::*;

pub fn use_form<Values>() {
    let initial_values = use_ref(|| "");
    let initial_errors = use_ref(|| "");
    let initial_status = use_ref(|| "");
    let is_mounted = use_ref(|| false);
}
