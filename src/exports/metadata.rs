use std::os::raw::c_char;

use fnlog::fn_debug;

#[export_name = "UPLAY_METADATA_SetSingleEventTag"]
pub fn uplay_metadata_set_single_event_tag(
    _string_name_utf8: *const c_char,
    _string_value_utf8: *const c_char,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_METADATA_SetContinuousTag"]
pub fn uplay_metadata_set_continuous_tag(
    _string_name_utf8: *const c_char,
    _string_value_utf8: *const c_char,
) -> usize {
    fn_debug!("__CALL__");
    return 0;
}

#[export_name = "UPLAY_METADATA_ClearContinuousTag"]
pub fn uplay_metadata_clear_continuous_tag(_string_name_utf8: *const c_char) -> usize {
    fn_debug!("__CALL__");
    return 0;
}
