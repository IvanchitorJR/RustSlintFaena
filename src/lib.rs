slint::include_modules!();

//Archivos donde se ejecutan ciertas acciones y archivos necesarios para el desarrollo de APP
mod modelo;
mod registrar;
mod login;
mod entrar_sala;

use std::sync::Arc;
use reqwest::Client;
use registrar::{registrar_usuario, validar_usuario};
use login::verificar_credenciales;
use entrar_sala::unirse_sala;

#[tokio::main]
#[cfg(target_os = "android")]
#[no_mangle]
pub async fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let ui = MainWindow::new().unwrap();
    let client = Arc::new(reqwest::Client::new());  //cliente http que provee reqwest
    let client1 = client.clone();
    let client2 = client.clone();

    //Boton de login
    ui.on_login({
        let ui_handle = ui.as_weak();
        move || {
            let ui_instance = ui_handle.unwrap(); 
            let correo = ui_instance.get_email_login().to_string();
            let pswd = ui_instance.get_pswd_login().to_string();

            if correo.is_empty() || pswd.is_empty(){
                ui_instance.set_empty_error("Datos vacios".into());
                ui_instance.set_empty_error_text_color(ui_instance.get_rojo());
                ui_instance.set_input_border_color(ui_instance.get_border());
            } 
            else {
                let client = client1.clone();
                tokio::spawn(verificar_credenciales(client, correo, pswd, ui_handle.clone()));
            }
        }
    });

    //Boton de registrar
    ui.on_registrar({
        let ui_handle = ui.as_weak();
        move || {
            let ui_instance = ui_handle.unwrap(); 
            let correo = ui_instance.get_email_registro().to_string(); 
            let pswd = ui_instance.get_pswd_registro().to_string();
            let username = ui_instance.get_username().to_string();
            let confirm_pswd = ui_instance.get_confirm_pswd().to_string();

            match validar_usuario(&username, &correo, &pswd, &confirm_pswd){
                Ok(_) => {
                    let client= client2.clone();
                    tokio::spawn(registrar_usuario(client, username, correo, pswd, ui_handle.clone()));
                }
                Err(err) => {
                    ui_instance.set_empty_error(err.into());
                    ui_instance.set_empty_error_text_color(ui_instance.get_rojo());
                    ui_instance.set_input_border_color(ui_instance.get_border());
                }
            }
        }
    });
    ui.on_enviar_codigo({
        let ui_handle = ui.as_weak();
        move || {
            let ui_instance = ui_handle.unwrap();
            let codigo = ui_instance.get_codigo().to_string();
            if codigo.chars().count() != 6{
                ui_instance.set_empty_error("El código debe de llevar 6 caracteres");
                ui_instance.set_empty_error_text_color(ui_instance.get_rojo());
            }
            else {
                entrar_sala();
            }
        }
    });

    ui.run().unwrap();
}
