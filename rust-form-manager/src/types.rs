pub struct FormErrors<Values> {
    // TODO: Mapping map object
    pub errors: Values,
}
pub struct FormTouched<Values> {
    // TODO: Mapping map object
    pub touched: Values,
}

/**
 * Form state tree
 */
pub struct FormState<Values, Status> {
    /** Form values */
    pub values: Values,
    /** map of field names to specific error for that field */
    pub errors: FormErrors<Values>,
    /** map of field names to whether the field has been touched */
    pub touched: FormTouched<Values>,
    /** whether the form is currently submitting */
    pub is_submitting: bool,
    /** whether the form is currently validating (prior to submission) */
    pub is_validating: bool,
    /** Top level status state, in case you need it */
    pub status: Status,
    /** Number of times user tried to submit the form */
    pub submit_count: f64,
}

/**
 * Formik computed properties. These are read-only.
 */
pub struct FormComputedProps<Values, Status> {
    /** True if any input has been touched. False otherwise. */
    pub dirty: bool,
    /** True if state.errors is empty */
    pub is_valid: bool,
    /** The initial values of the form */
    pub initial_values: Values,
    /** The initial errors of the form */
    pub initial_errors: FormErrors<Values>,
    /** The initial visited fields of the form */
    pub initial_touched: FormTouched<Values>,
    /** The initial status of the form */
    pub initial_status: Status,
}

pub struct FormHelpers;

impl FormHelpers {
    pub fn set_status() {}
    pub fn set_errors() {}
}
