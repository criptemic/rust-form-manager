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
