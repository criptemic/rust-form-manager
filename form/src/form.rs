use std::rc::Rc;
use yew::prelude::*;

use crate::reducer::{FieldValue, FormReducerAction, FormReducerState};

pub struct FormProps<Values> {
    pub initial_values: Values,
    pub validate_on_change: bool,
    pub validate_on_blur: bool,
    pub validate_on_mount: bool,
    pub is_initial_valid: bool,
    pub enable_initialize: bool,
    pub on_submit: Callback<Values>,
}

pub struct FormState<Values> {
    pub inner: UseReducerHandle<FormReducerState<Values>>,
    // pub handle_change: Callback<Event>,
}

fn validated(value: Option<bool>) {
    match value {
        Some(true) => {}
        Some(false) => {}
        None => {}
    }
}

impl<Values> FormState<Values> {
    pub fn set_status(&self, payload: Option<String>) {
        self.inner.dispatch(FormReducerAction::SetStatus(payload))
    }
    pub fn set_submitting(&self, is_submitting: bool) {
        self.inner
            .dispatch(FormReducerAction::SetIsSubmitting(is_submitting));
    }

    pub fn set_validating(&self, is_validating: bool) {
        self.inner
            .dispatch(FormReducerAction::SetIsValidating(is_validating));
    }

    pub fn set_values(&self, values: Values, should_validate: Option<bool>) {
        self.inner.dispatch(FormReducerAction::SetValues(values));
        validated(should_validate);
    }

    pub fn set_errors(&self, values: Values) {
        self.inner.dispatch(FormReducerAction::SetErrors(values));
    }

    pub fn set_touched(&self, values: Values, should_validate: Option<bool>) {
        self.inner.dispatch(FormReducerAction::SetTouched(values));
        validated(should_validate);
    }

    pub fn set_field_value(
        &self,
        field: String,
        value: Option<String>,
        should_validate: Option<bool>,
    ) {
        self.inner
            .dispatch(FormReducerAction::SetFieldValue(FieldValue {
                field,
                value,
            }));
        validated(should_validate);
    }

    pub fn set_field_error(&self, field: String, value: Option<String>) {
        self.inner
            .dispatch(FormReducerAction::SetFieldError(FieldValue {
                field,
                value,
            }));
    }

    pub fn set_field_touched(
        &self,
        field: String,
        value: Option<String>,
        should_validate: Option<bool>,
    ) {
        self.inner
            .dispatch(FormReducerAction::SetFieldTouch(FieldValue {
                field,
                value,
            }));
        validated(should_validate);
    }

    pub fn reset_form(&self, new_values: FormState<Values>) {
        self.inner
            .dispatch(FormReducerAction::SetFormState(new_values));
    }
}

pub fn use_form<Values: 'static>(props: FormProps<Values>) -> FormState<Values> {
    let values = Rc::new(props.initial_values);
    let errors = values.clone();
    let touched = values.clone();

    let inner = use_reducer(move || FormReducerState {
        values,
        errors,
        touched,
        is_submitting: false,
        is_validating: false,
        submit_count: 0,
        status: None,
    });

    // let handle_change = {};

    FormState {
        inner,
        // handle_change,
    }
}
