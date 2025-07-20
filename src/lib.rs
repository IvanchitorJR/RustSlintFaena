slint::include_modules!();

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    let ui = MainWindow::new().unwrap();
    ui.on_login({
        let ui_handle = ui.as_weak();
        move || {
            let ui_instance = ui_handle.unwrap();
            let correo = ui_instance.get_email();
            let pswd = ui_instance.get_pswd();

            if correo.is_empty() || pswd.is_empty(){
                ui_instance.set_empty_error("Usuario y/o contraseña incorrectos".into());
                ui_instance.set_empty_error_text_color(ui_instance.get_rojo());
                ui_instance.set_input_border_color(ui_instance.get_border());
            } else {
                ui_instance.set_empty_error("Login exitoso".into());
                ui_instance.set_empty_error_text_color(ui_instance.get_verde());
                ui_instance.set_input_border_color(ui_instance.get_verde());
                println!("Login con: {} / {}", correo, pswd);
            }
        }
    });
    ui.run().unwrap();
}