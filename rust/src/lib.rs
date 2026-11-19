use slint::platform::api::PlatformWindow;

slint::include_modules!();

#[no_mangle]
#[allow(non_snake_case)]
pub fn android_main(app: android_activity::AndroidApp) {
    let window = AppWindow::new().unwrap();
    window.run().unwrap();
}
