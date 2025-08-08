slint::include_modules!();
use std::sync::Arc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
//estructura de los usuarios en rust, posteriolmente se convierte en json
pub struct Usuario {
    pub nombre: String,
    #[serde(rename = "contrasena")]
    pub contrase_a: String,
    pub email: String,
    pub premium: bool,
}
// Funciones para validar la password
fn buscar_caracter_especial(password:&str) -> bool{
    let resultado = password.chars().any(|letra| !letra.is_ascii_alphanumeric());
    resultado
}
fn buscar_caracter_digito(password: &str) -> bool{
    let resultado: bool = password.chars().any(|letra| letra.is_ascii_digit());
    resultado
}
fn buscar_caracter_mayuscula(password: &str) -> bool{
    let resultado: bool = password.chars().any(|letra| letra.is_ascii_uppercase());
    resultado
}
fn buscar_caracter_minuscula(password: &str) -> bool{
    let resultado: bool = password.chars().any(|letra| letra.is_ascii_lowercase());
    resultado
}

// Funcion para validar el usuario que quiere registrar la persona
fn validar_usuario(username: &str, correo: &str, pswd: &str, confirm_pswd: &str) -> Result<(), &'static str>{
    if correo.is_empty() || username.is_empty() || pswd.is_empty() || confirm_pswd.is_empty(){
        Err("No puede haber campos vacíos")
    }
    else if pswd != confirm_pswd{
        Err("Las contraseñas no coinciden")
    }
    else if pswd.chars().count() < 8{
        Err("La contraseña necesita ser de minimo 8 caracteres")
    }
    else if !buscar_caracter_especial(&pswd) {
        Err("La contraseña no tiene minimo 1 caracter especial")
    }
    else if !buscar_caracter_digito(&pswd) {
        Err("La contraseña no tiene minimo 1 digito")
    }
    else if !buscar_caracter_mayuscula(&pswd) {
        Err("La contraseña no tiene minimo 1 mayuscula")
    }
    else if !buscar_caracter_minuscula(&pswd) {
         Err("La contraseña no tiene minimo 1 minuscula")
    }
    else{
        Ok(())
    }
}

//Funciones principales de los botones login y registrarse
async fn registrar_usuario(cliente: Arc<Client>, username: String, correo: String, password: String, ui_handle: slint::Weak<MainWindow>){
    let url="https://faena-backend.onrender.com/api/usuarios"; //url para la api rest con los usuarios
    match cliente.get(url).send().await{ //consulta de todos los usuarios para verificar correos
        Ok(res)=>{ //respuesta de GET
            match res.json::<Vec<Usuario>>().await{
                Ok(usuarios)=>{ //si tenemos a los usuarios
                    let mut correo_registrado = false;
                    let mut usuario_registrado=false;
                    for usuario in usuarios{
                        if usuario.email==correo{
                            correo_registrado=true;
                            break;
                        }
                    }
                    if !correo_registrado{
                        let nuevo_usuario= Usuario{
                            nombre: username.clone(),
                            email: correo.clone(),
                            contrase_a: password.clone(),
                            premium: false,
                        };
                        let res= cliente.post(url).json(&nuevo_usuario).send().await;
                        match res {
                            Ok(_res)=>{
                                if _res.status().is_success(){
                                    usuario_registrado=true;
                                    eprintln!("Bienvenido a la faenation: {:?}", _res);
                                }
                                else{
                                    usuario_registrado=false;
                                    eprintln!("ERROR: {:?}", _res);
                                }
                            }
                            Err(err)=>{
                                usuario_registrado=false;
                                eprintln!("Fallo al hacer peticion POST: {:?}", err);
                            }
                        }
                    }
                    slint::invoke_from_event_loop(move ||{
                        if let Some(ui) = ui_handle.upgrade(){
                            if usuario_registrado{
                                ui.set_pantalla_actual(3);
                            }
                            if correo_registrado{
                                ui.set_empty_error("Correo ya registrado".into());
                                ui.set_empty_error_text_color(ui.get_rojo());
                                ui.set_input_border_color(ui.get_border());
                            }
                        }
                    }).unwrap();
                }
                Err(err) => { //error si no tenermos usuarios
                        eprintln!("Error al conseguir los usuarios: {:?}", err);
                }
            }
        }
    Err(err) => { //error de GET
        eprintln!("Error al hacer la peticion reqwest: {:?}", err);
    }
}
}
    

async fn verificar_credenciales(cliente: Arc<Client>, correo: String, password: String, ui_handle: slint::Weak<MainWindow>){
    let url = "https://faena-backend.onrender.com/api/usuarios"; //url para la api rest con los usuarios
    match cliente.get(url).send().await { //enviamos la peticion y vemos las posibles respuestas
        Ok(res) => { //si recibimos una respuesta
            match res.json::<Vec<Usuario>>().await {
                Ok(usuarios)=>{ //si existen usuarios o pudo la app recibir y deserializar la respuesta en un vector de usuarios(struct)
                    let mut encontrado: bool = false;
                    for usuario in usuarios{
                        if usuario.email==correo && usuario.contrase_a==password{
                            encontrado=true;
                            break;
                        }
                    } 
                    slint::invoke_from_event_loop(move ||{
                        if let Some(ui) = ui_handle.upgrade(){
                            if encontrado{
                                ui.set_empty_error("Bienvenido a Faena".into());
                                println!("Welcome to faena");
                                ui.set_pantalla_actual(3); //pantalla startMenu 
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
    let client = Arc::new(reqwest::Client::new());  //cliente http que provee reqwest
    let client1 = client.clone(); // para la primera closure
    let client2 = client.clone(); // para la segunda closure
    //Boton de login al activarse en login.slint
    ui.on_login({
        let ui_handle = ui.as_weak(); //se hace una variable para tener la ui en el scope actual
        move || { //closure (lo que conocemos como listener)
            let ui_instance = ui_handle.unwrap(); 
            let correo = ui_instance.get_email_login().to_string(); //recibimos los datos del usuario
            let pswd = ui_instance.get_pswd_login().to_string();

            if correo.is_empty() || pswd.is_empty(){ //verificaciones, aca es primero esto antes del query
                ui_instance.set_empty_error("Datos vacios".into());
                ui_instance.set_empty_error_text_color(ui_instance.get_rojo());
                ui_instance.set_input_border_color(ui_instance.get_border());
            } 
            else {
                let client = client1.clone(); //usarlo dentro de tokio::spawn
                tokio::spawn(verificar_credenciales(client, correo, pswd, ui_handle.clone()));
            }
        }
    });
    ui.on_registrar({
        let ui_handle = ui.as_weak(); //se hace una variable para tener la ui en el scope actual
        move || { //closure (lo que conocemos como listener)
            let ui_instance = ui_handle.unwrap(); 
            //recibimos las propiedades
            let correo = ui_instance.get_email_registro().to_string(); 
            let pswd = ui_instance.get_pswd_registro().to_string();
            let username = ui_instance.get_username().to_string();
            let confirm_pswd = ui_instance.get_confirm_pswd().to_string();
            //Revisamos los requesitos para el registro
            match validar_usuario(&username, &correo, &pswd, &confirm_pswd){
                Ok(_)=>{
                    let client= client2.clone();
                    tokio::spawn(registrar_usuario(client, username, correo, pswd, ui_handle.clone()));
                }
                Err(err)=>{
                    ui_instance.set_empty_error(err.into());
                    ui_instance.set_empty_error_text_color(ui_instance.get_rojo());
                    ui_instance.set_input_border_color(ui_instance.get_border());
                }
            }
        }
    });
    ui.run().unwrap();
}