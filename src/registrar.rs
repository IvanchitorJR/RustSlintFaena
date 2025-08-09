use crate::modelo::Usuario;
use reqwest::Client;
use std::sync::Arc;
use slint::Weak;

use crate::MainWindow;

// Funciones para validar la password
fn buscar_caracter_especial(password:&str) -> bool{
    password.chars().any(|letra| !letra.is_ascii_alphanumeric())
}
fn buscar_caracter_digito(password: &str) -> bool{
    password.chars().any(|letra| letra.is_ascii_digit())
}
fn buscar_caracter_mayuscula(password: &str) -> bool{
    password.chars().any(|letra| letra.is_ascii_uppercase())
}
fn buscar_caracter_minuscula(password: &str) -> bool{
    password.chars().any(|letra| letra.is_ascii_lowercase())
}

// Funcion para validar el usuario que quiere registrar la persona
pub fn validar_usuario(username: &str, correo: &str, pswd: &str, confirm_pswd: &str) -> Result<(), &'static str>{
    if correo.is_empty() || username.is_empty() || pswd.is_empty() || confirm_pswd.is_empty(){
        Err("No puede haber campos vacíos")
    }
    else if pswd != confirm_pswd{
        Err("Las contraseñas no coinciden")
    }
    else if pswd.chars().count() < 8{
        Err("La contraseña necesita ser de minimo 8 caracteres")
    }
    else if !buscar_caracter_especial(pswd) {
        Err("La contraseña no tiene minimo 1 caracter especial")
    }
    else if !buscar_caracter_digito(pswd) {
        Err("La contraseña no tiene minimo 1 digito")
    }
    else if !buscar_caracter_mayuscula(pswd) {
        Err("La contraseña no tiene minimo 1 mayuscula")
    }
    else if !buscar_caracter_minuscula(pswd) {
        Err("La contraseña no tiene minimo 1 minuscula")
    }
    else{
        Ok(())
    }
}

pub async fn registrar_usuario(cliente: Arc<Client>,username: String,correo: String,password: String,ui_handle: Weak<MainWindow>){
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