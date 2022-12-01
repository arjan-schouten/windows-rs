use windows::Win32::Foundation::*;

#[test]
fn test() {
    let status = VARIANT_BOOL::default();
    assert_eq!(status, VARIANT_FALSE);

    let status = VARIANT_TRUE;
    assert_eq!(status.0, -1);
    assert_eq!(status.as_bool(), true);
    assert_eq!(status.ok().is_ok(), true);

    unsafe {
        SetLastError(ERROR_ACCESS_DENIED);
    }
    let status = VARIANT_FALSE;
    assert_eq!(status.0, 0);
    assert_eq!(status.as_bool(), false);
    let result = status.ok();
    assert_eq!(result.is_ok(), false);
    let error = result.unwrap_err();
    assert_eq!(error.code(), E_ACCESSDENIED);
}
