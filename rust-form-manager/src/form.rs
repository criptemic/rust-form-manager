use yew::prelude::*;

pub struct UseForm<Values> {
    pub initial_values: Values,
    // pub validate_on_change: bool,
    // pub validate_on_blur: bool,
    // pub validate_on_mount: bool,
    // pub is_initial_valid: bool,
    // pub enable_initialize: bool,
    //   onSubmit,
}

impl<Values> UseForm<Values> {
    pub fn get_initial_values(self) -> Values {
        self.initial_values
    }

    pub fn on_submit() {}
}

pub fn use_form<Values: 'static>(props: UseForm<Values>) {
    let initial_values = use_ref(|| props.get_initial_values());
    let initial_errors = use_ref(|| "");
    let initial_status = use_ref(|| "");
    let is_mounted = use_ref(|| false);
}

pub struct FormField {
    pub name: String,
    pub age: i32,
}

#[function_component(Form)]
fn form() -> Html {
    let props = UseForm {
        initial_values: FormField {
            name: "".to_string(),
            age: 0,
        },
    };
    let _state = use_form(props);
    html!()
}
