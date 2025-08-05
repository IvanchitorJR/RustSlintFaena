slint::include_modules!();
use reqwest::Client;
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

async fn verificar_credenciales(cliente: Client, correo: String, password: String, ui_handle: slint::Weak<MainWindow>){
    let url = "https://faena-backend.onrender.com/api/usuarios"; //url para la api rest con los usuarios
    match cliente.get(url).send().await { //enviamos la peticion y vemos las posibles respuestas
        Ok(res) => { //si recibimos una respuesta
            match res.json::<Vec<Usuario>>().await {
                Ok(usuarios)=>{ //si existen usuarios o pudo la app recibir y deserializar la respuesta en un vector de usuarios(struct)
                    let mut encontrado: bool = false;
                    for Usuario in usuarios{
                        if Usuario.email==correo && Usuario.contrasena==password{
                            encontrado=true;
                            break;
                        }
                    } 
                    slint::invoke_from_event_loop(move ||{
                        if let Some(ui) = ui_handle.upgrade(){
                            if encontrado{
                                ui.set_empty_error("Bienvenido a Faena".into());
                                println!("Welcome to faena");
                            }
                            else{
                                ui.set_empty_error("Usuario y/o contrasena incorrectas".into());
                                ui.set_empty_error_text_color(ui.get_rojo());
                                ui.set_input_border_color(ui.get_border());
                                println!("No hay");
                            }
                        }
                    }).unwrap();
                }
                Err(err)=>{
                    eprintln!("Error al recibir los usuarios: {:?}", err);
                }
            }
        }
        Err(err)=>{
                eprint!("Error al hacer la peticion, {:?}", err);
            }
    }
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
            let correo = ui_instance.get_email().to_string(); //recibimos los datos del usuario
            let pswd = ui_instance.get_pswd().to_string();

            if correo.is_empty() || pswd.is_empty(){ //verificaciones, aca es primero esto antes del query
                ui_instance.set_empty_error("Datos vacios".into());
                ui_instance.set_empty_error_text_color(ui_instance.get_rojo());
                ui_instance.set_input_border_color(ui_instance.get_border());
            } 
            else {
                let client = client.clone(); //usarlo dentro de tokio::spawn
                tokio::spawn(verificar_credenciales(client, correo, pswd, ui_handle.clone()));
            }
        }
    });
    ui.run().unwrap();
}