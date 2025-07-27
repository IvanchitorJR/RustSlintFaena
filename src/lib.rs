slint::include_modules!();
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
//estructura de los usuarios en rust, posteriolmente se convierte en json
struct Usuario {
    id: i32,
    nombre: String,
    email: String,
    contrasena: String,
    premium: bool,
}

#[tokio::main]
#[cfg(target_os = "android")]
#[no_mangle]
pub async fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    let ui = MainWindow::new().unwrap();
    let client = reqwest::Client::new(); //cliente http que provee reqwest
    //Boton de login al activarse en login.slint
    ui.on_login({
        let ui_handle = ui.as_weak(); //se hace una variable para tener la ui en el scope actual
        move || { //closure (lo que conocemos como listener)
            let ui_instance = ui_handle.unwrap(); 
            let correo = ui_instance.get_email(); //recibimos los datos del usuario
            let pswd = ui_instance.get_pswd();

            if correo.is_empty() || pswd.is_empty(){ //verificaciones, aca es primero esto antes del query
                ui_instance.set_empty_error("Datos vacios".into());
                ui_instance.set_empty_error_text_color(ui_instance.get_rojo());
                ui_instance.set_input_border_color(ui_instance.get_border());
            } 
            else {
                let client = client.clone(); //usarlo dentro de tokio::spawn, clónarlo aquí también
                let url = "http://faena-backend.onrender.com/api/usuarios"; //url para la api REST de faena
                tokio::spawn(async move{
                    match client.get(url).send().await {
                        Ok(res)=>{
                            match res.json::<Vec<Usuario>>().await {
                                Ok(usuarios)=>{
                                    for usuario in usuarios{
                                        println!("{:#?}", usuario);
                                    }
                                }
                                Err(err) => eprintln!("Error al deserializar usuarios: {:?}", err),
                            }
                        }
                        Err(err) => eprintln!("Error al hacer request: {:?}", err),
                    }
                });
            }
        }
    });
    ui.run().unwrap();
}