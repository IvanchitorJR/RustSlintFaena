slint::include_modules!();

#[no_mangle]
pub fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    let ui = MainWindow::new().unwrap();
    ui.run().unwrap();
}