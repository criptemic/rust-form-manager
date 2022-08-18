use std::fmt::Error;
use yew::{Callback, Event};

/**
 * Form state tree
 */
pub struct FormState<Values> {
    /** Form values */
    pub values: Values,
    /** map of field names to specific error for that field */
    pub errors: Values,
    /** map of field names to whether the field has been touched */
    pub touched: Values,
    /** whether the form is currently submitting */
    pub is_submitting: bool,
    /** whether the form is currently validating (prior to submission) */
    pub is_validating: bool,
    /** Top level status state, in case you need it */
    pub status: Option<String>,
    /** Number of times user tried to submit the form */
    pub submit_count: f64,
}

/**
 * Formik computed properties. These are read-only.
 */
pub struct FormComputedProps<Values> {
    /** True if any input has been touched. False otherwise. */
    pub dirty: bool,
    /** True if state.errors is empty */
    pub is_valid: bool,
    /** The initial values of the form */
    pub initial_values: Values,
    /** The initial errors of the form */
    pub initial_errors: Values,
    /** The initial visited fields of the form */
    pub initial_touched: Values,
    /** The initial status of the form */
    pub initial_status: Option<String>,
}

pub enum Message {
    Success,
    Failure,
}

pub trait FormHelpers<Values> {
    fn set_status(status: Option<String>) -> Message;

    fn set_errors(errors: Values) -> Message;

    fn set_submitting(is_submitting: bool) -> Message;

    fn set_touched(touched: Values, should_validate: Option<bool>) -> Message;

    fn set_values(values: Values, should_validate: Option<bool>) -> Message;

    fn set_field_value(
        field: String,
        value: Option<String>,
        should_validate: Option<bool>,
    ) -> Message;

    fn set_field_error(field: String, message: Option<String>) -> Message;

    fn set_field_touched(
        field: String,
        is_touched: Option<bool>,
        should_validate: Option<bool>,
    ) -> Message;

    fn validate_form() -> Result<FormState<Values>, Error>;

    fn validate_field(field: String) -> Message;

    fn reset_form(next_state: FormState<Values>) -> Message;

    fn submit_form() -> Result<Message, Error>;

    fn set_form_state<F>(f: F) -> Message;
}

pub trait FormHandlers<Values> {
    fn handle_submit(e: Option<Callback<Event>>) -> Message;

    fn handle_reset(e: Option<Callback<Event>>) -> Message;

    fn handle_blur(e: Option<Callback<Event>>) -> Message;

    fn handle_change(e: Option<Callback<Event>>) -> Message;

    fn get_field_props<T>(props: T) -> FieldInputProps<T>;

    fn get_field_metadata<T>(props: T) -> FieldMetaProps<T>;

    fn get_field_helpers<T>(props: T) -> FieldHelperProps;
}

pub struct FieldInputProps<T> {
    pub value: T,
    pub name: String,
    pub multiple: Option<bool>,
    pub checked: Option<bool>,
}

impl<T> FieldInputProps<T> {
    pub fn on_change() {}

    pub fn on_blur() {}
}

pub struct FieldMetaProps<T> {
    /** Value of the field */
    pub value: T,
    /** Error message of the field */
    pub error: Option<String>,
    /** Has the field been visited? */
    pub touched: bool,
    /** Initial value of the field */
    pub initial_value: Option<T>,
    /** Initial touched state of the field */
    pub initial_touched: bool,
    /** Initial error message of the field */
    pub initial_error: Option<String>,
}

pub struct FieldHelperProps;

impl FieldHelperProps {
    pub fn set_values<T>(self, value: T, should_validate: Option<bool>) {}
    pub fn set_touched<T>(self, value: bool, should_validate: Option<bool>) {}
    pub fn set_error<T>(self, value: Option<bool>) {}
}
