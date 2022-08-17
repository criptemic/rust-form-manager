use std::rc::Rc;
use yew::prelude::*;

pub enum FormReducerAction<Values> {
    SetValues(Values),
    SetTouched,
    SetErrors,
    SetStatus,
    SetIsSubmitting(bool),
    SetIsValidating,
    SetFieldValue,
    SetFieldTouch,
    SetFieldError,
    ResetForm,
    SetFormikState,
    SubmitAttempt,
    SubmitFailure,
    SubmitSuccess,
}

// struct State<Values> {
//     values: Values,
//     errors: Values,
//     touched: Values,
//     status: Values,
//     is_submitting: bool,
//     is_validating: bool,
//     submit_count: bool,
// }

struct FormReducerState<Values> {
    state: Values,
}

// impl<Values> Default for FormReducerState<Values> {
//     fn default() -> Self {
//         Self { state: 10 }
//     }
// }

impl<Values> Reducible for FormReducerState<Values>
where
    Values: Copy,
{
    type Action = FormReducerAction<Values>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_ctr = match action {
            FormReducerAction::SetValues(_value) => self.state,
            FormReducerAction::SetTouched => self.state,
            FormReducerAction::SetErrors => self.state,
            FormReducerAction::SetStatus => self.state,
            FormReducerAction::SetIsSubmitting(_value) => self.state,
            FormReducerAction::SetIsValidating => self.state,
            FormReducerAction::SetFieldValue => self.state,
            FormReducerAction::SetFieldTouch => self.state,
            FormReducerAction::SetFieldError => self.state,
            FormReducerAction::ResetForm => self.state,
            FormReducerAction::SetFormikState => self.state,
            FormReducerAction::SubmitAttempt => self.state,
            FormReducerAction::SubmitFailure => self.state,
            FormReducerAction::SubmitSuccess => self.state,
        };

        Self { state: next_ctr }.into()
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
