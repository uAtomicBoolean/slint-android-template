slint::include_modules!();

#[unsafe(no_mangle)]
fn android_main(android_app: slint::android::AndroidApp) -> Result<(), slint::PlatformError> {
    // initialize android before creating main window
    slint::android::init(android_app).unwrap();
    AppWindow::new().unwrap().run()
}
