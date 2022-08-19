use std::rc::Rc;
use yew::prelude::*;

use crate::types::FormState;

pub struct FieldValue {
    pub field: String,
    pub value: Option<String>,
}

pub enum FormReducerAction<Values> {
    SetValues(Values),
    SetIsSubmitting(bool),
    SetIsValidating(bool),
    SetFieldValue(FieldValue),
    SetFieldTouch(FieldValue),
    SetFieldError(FieldValue),
    SetTouched(Values),
    SetErrors(Values),
    SetStatus(Option<String>),
    ResetForm(FormState<Values>),
    SetFormikState(FormState<Values>),
    SubmitAttempt,
    SubmitFailure,
    SubmitSuccess,
}

pub struct FormReducerState<Values> {
    pub values: Values,
    pub errors: Values,
    pub touched: Values,
    pub status: Option<String>,
    pub is_submitting: bool,
    pub is_validating: bool,
    pub submit_count: bool,
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
            FormReducerAction::ResetForm(_payload) => action_returns(self),
            FormReducerAction::SetFormikState(_payload) => action_returns(self),
            FormReducerAction::SubmitAttempt => action_returns(self),
            FormReducerAction::SubmitFailure => action_returns(self),
            FormReducerAction::SubmitSuccess => action_returns(self),
            // FormReducerAction::SubmitSuccess => {
            //     let state = {
            //         let data = Rc::new(RefCell::new(self));
            //         let temp = data.borrow_mut();
            //         temp.is_submitting = false;
            //         temp
            //     };
            //     state.deref_mut()
            // }
        };

        next_ctr.into()
    }
}

// #[function_component(UseReducer)]
// fn reducer() -> Html {
//     let state = use_reducer();
//     let on_handle_reducer = {
//         let state = state.clone();
//         Callback::from(move |event: MouseEvent| {
//             state.dispatch(FormReducerAction::SetValues("hello".to_string()))
//         });
//     };

//     html!()
// }
