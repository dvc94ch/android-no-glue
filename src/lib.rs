mod app;
pub use app::app_main;

#[cfg(target_os = "android")]
#[no_mangle]
pub unsafe extern "C" fn ANativeActivity_onCreate(
    activity: *mut std::os::raw::c_void,
    saved_state: *mut std::os::raw::c_void,
    saved_state_size: usize,
) {
    std::env::set_var("RUST_BACKTRACE", "1");
    android_glue::init(
        activity as _,
        saved_state as _,
        saved_state_size as _,
        app_main,
    );
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn run_app() {
    app_main();
}
