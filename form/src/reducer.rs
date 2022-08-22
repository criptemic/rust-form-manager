use std::rc::Rc;
use yew::prelude::*;

use crate::form::FormState;

pub struct FieldValue {
    pub field: String,
    pub value: Option<String>,
}

pub enum FormReducerAction<Values> {
    SetStatus(Option<String>),
    SetIsSubmitting(bool),
    SetIsValidating(bool),
    SetValues(Values),
    SetTouched(Values),
    SetErrors(Values),
    SetFieldValue(FieldValue),
    SetFieldTouch(FieldValue),
    SetFieldError(FieldValue),
    SetFormState(FormState<Values>),
    SubmitAttempt,
    SubmitFailure,
    SubmitSuccess,
}

pub struct FormReducerState<Values> {
    pub values: Rc<Values>,
    pub errors: Rc<Values>,
    pub touched: Rc<Values>,
    pub status: Option<String>,
    pub is_submitting: bool,
    pub is_validating: bool,
    pub submit_count: i32,
}

fn action_returns<Values>(
    prev_state: Rc<FormReducerState<Values>>,
) -> Rc<FormReducerState<Values>> {
    // TODO: Action Need to implement for all Cases..
    return prev_state;
}

impl<Values> Reducible for FormReducerState<Values> {
    type Action = FormReducerAction<Values>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_ctr = match action {
            FormReducerAction::SetValues(_payload) => action_returns(self),
            FormReducerAction::SetTouched(_payload) => action_returns(self),
            FormReducerAction::SetErrors(_payload) => action_returns(self),
            FormReducerAction::SetStatus(_payload) => action_returns(self),
            FormReducerAction::SetIsSubmitting(_payload) => action_returns(self),
            FormReducerAction::SetIsValidating(_payload) => action_returns(self),
            FormReducerAction::SetFieldValue(_payload) => action_returns(self),
            FormReducerAction::SetFieldTouch(_payload) => action_returns(self),
            FormReducerAction::SetFieldError(_payload) => action_returns(self),
            FormReducerAction::SetFormState(_payload) => action_returns(self),
            FormReducerAction::SubmitAttempt => action_returns(self),
            FormReducerAction::SubmitFailure => action_returns(self),
            FormReducerAction::SubmitSuccess => action_returns(self),
        };
        next_ctr.into()
    }
}
